# Decisiones — enlaces-e-imagenes

> Estado: cerrada, sus cinco historias verificadas
> Última actualización: 2026-09-18
> Modo: new-feature

Aquí se escribe toda decisión de esta feature que alguien pueda querer revertir
más adelante, con sus alternativas y sus consecuencias, y se indexa en el mismo
turno en `../../03-arquitectura.md`. La numeración `AD-nn` es continua en todo el
proyecto.

Las que se sabe que van a aparecer más adelante, con lo que ya se averiguó de
ellas, siguen anotadas al final de este archivo.

---

## AD-15 — Arreglar la causa compartida de la pérdida de contenido en `parse_blocks`

Fecha: 2026-08-24

**Contexto.** `plan.md` documentó el defecto antes de empezar: `parse_blocks`
corta su bucle en `Some(End(_)) => break`, pensado para reconocer el cierre del
bloque que abrió quien llamó (por ejemplo, `TagEnd::BlockQuote` cuando
`parse_blocks` se llamó para el contenido de una cita). El problema es que esa
rama no distingue ese `End` de cualquier otro: los `End` de `Link`, `Image` y
`HtmlBlock` no los abre ni los consume ninguna rama existente, así que en cuanto
aparecen, el primero de ellos dispara el mismo `break` y el resto del documento
—o del párrafo— se descarta en silencio. Un enlace parte el párrafo en dos y
pierde el texto posterior; un bloque HTML trunca el documento entero a partir de
ahí.

**Alternativas consideradas.**

- *Caso especial para enlaces en `parse_blocks`.* Interceptar solo
  `Start(Link)`/`End(Link)` ahí donde se observó el fallo. Rechazada: HU-01 pide
  explícitamente la causa compartida, no un parche puntual, y el mismo defecto
  reaparecería con `Image` y con `HtmlBlock` la próxima vez que alguien los
  tocara — de hecho `HtmlBlock` es RF-13, en `front-matter-y-html`, y llegaría
  ahí todavía roto si no se arregla aquí.
- *Hacer que `Some(End(_)) => break` ignore cualquier `End` no reconocido en vez
  de cortar.* Rechazada: cambia el criterio de "esto es el cierre del padre" por
  "esto es cualquier cosa que no sepamos abrir", que es más difícil de razonar y
  esconde en silencio un `End` genuinamente inesperado si en el futuro se activa
  una extensión de `pulldown-cmark` que hoy no está habilitada.
- *Que cada evento que abre algo lo cierre explícitamente, igual que ya hace el
  código con `Emphasis`/`Strong`/`Strikethrough`.* Elegida. `parse_inline` gana
  ramas para `Start(Tag::Link)`/`End(TagEnd::Link)` y
  `Start(Tag::Image)`/`End(TagEnd::Image)`, así que ya no se le escapan sin
  consumir. `parse_blocks` gana una rama para `Tag::HtmlBlock` (ya estaba en
  `is_block_start`, pero cualquier `Tag` no listado explícitamente caía en el
  `_ => {}` que no consumía nada) que vacía su contenido y su `End` sin producir
  ningún bloque nuevo, en el mismo estilo que ya existe para `Tag::CodeBlock`.

**Decisión.** Se implementa la tercera opción: ningún evento de apertura de
`Link`, `Image` o `HtmlBlock` queda sin una rama que lo abra y lo cierre. El
`Some(End(_)) => break` de `parse_blocks` vuelve a significar únicamente "esto
es el cierre de quien me llamó", que era su intención original.

**Consecuencias.**

- Efecto colateral esperado y no verificado aquí: un bloque HTML deja de truncar
  el documento (RF-13 vive en `front-matter-y-html` y allí se observará; aquí
  solo se deja constancia).
- `Span` gana un campo nuevo para poder representar el enlace sin romper la regla
  de que `markdown` no conoce colores — ver AD-16.
- Coste: unas 20 líneas repartidas entre `parse_inline` y `parse_blocks`. Ningún
  tipo de evento nuevo, ninguna dependencia nueva.

Estado: activa

---

## AD-16 — Representar el enlace como un campo `url` en `Span`, no como un tipo nuevo

Fecha: 2026-08-24

**Contexto.** RF-08.2 exige que el texto de un enlace se muestre distinguible
dentro de su párrafo, sin exponer la URL en el cuerpo del documento. `render` es
quien decide colores y subrayado (03-arquitectura.md); `markdown` solo puede
entregarle la URL como dato. Un enlace es inline —puede caer en medio de un
párrafo, y `[**negrita**](url)` incluso puede combinarse con otro estilo—, así
que la solución tiene que vivir al nivel de `Span`, no de `Block`.

**Alternativas consideradas.**

- *`Block::Link` nuevo.* Rechazada de entrada: un enlace no es un bloque, está
  dentro de un párrafo junto a texto normal. Forzarlo a bloque incumpliría
  CA-01.1 (las tres partes en el mismo párrafo).
- *Convertir `Span.text: String` en un enum,* p. ej.
  `enum SpanContent { Text(String), Link { text: String, url: String } }`.
  Rechazada: un enlace con negrita (`[**texto**](url)`) ya necesita combinar dos
  ejes ortogonales — como hoy se combinan negrita y cursiva en `SpanStyle` sin
  que ninguna sea un caso del enum de la otra—, y meter el enlace en el mismo
  enum que el texto obligaría a que cada consumidor de `Span` (hoy solo
  `render.rs`) supiera desenvolver ambos casos en cada sitio donde hoy solo lee
  `span.text`.
- *Meter la URL en `SpanStyle`.* Rechazada: `SpanStyle` son banderas de
  apariencia (negrita, cursiva, tachado, código en línea); la URL no es
  apariencia, es un dato semántico que `render` traduce a apariencia. Juntarlas
  difuminaría la frontera que `03-arquitectura.md` traza a propósito entre "qué
  es este texto" y "cómo se pinta".
- *Campo `url: Option<String>` en `Span`, junto a `style`.* Elegida. Es
  ortogonal al texto y al estilo, igual que ellos son ortogonales entre sí, y no
  obliga a ningún consumidor existente a cambiar de forma de leer `span.text`.

**Decisión.** `Span` gana `pub url: Option<String>`. `parse_inline` lo rellena
con el `dest_url` del `Tag::Link` que envuelve al span mientras dure ese enlace,
y lo deja en `None` el resto del tiempo. `render_spans` en `render.rs` colorea
con `theme.link` y subraya (`UnderlineStyle`) cualquier span cuyo `url` sea
`Some`, replicando el patrón ya usado para tachado.

**Consecuencias.**

- Cambio en un tipo compartido: los tres literales de `Span` que ya existían en
  `render.rs` (el estado vacío de RF-19) necesitan el campo nuevo. Coste: tres
  líneas, sin lógica.
- `Tag::Image` se consume en `parse_inline` con el mismo mecanismo que `Link`
  —para que su `End` tampoco quede suelto—, pero sin asignarle ningún `url`:
  mostrar la imagen es HU-03. Su texto alternativo cae hoy a texto plano dentro
  del párrafo, que coincide con el comportamiento de repliegue que pedirá
  RF-11.1 cuando una imagen no se pueda mostrar, pero eso **no se declara como
  HU-03 implementada**: es solo la consecuencia de no dejar el evento de imagen
  sin consumir.
- Cuando HU-02 (abrir el enlace) se construya, ya tiene de dónde leer la URL sin
  volver a tocar `markdown`.

Estado: activa

---

## AD-17 — Primeros tests del proyecto: módulo `#[cfg(test)]` junto al código, sin framework

Fecha: 2026-08-24

**Contexto.** El proyecto no tenía ningún test hasta ahora. HU-01 corrige un
defecto real ya observado (AD-15) y conviene dejar una prueba que falle si
alguien lo reintroduce, sin convertir eso en la ocasión de montar infraestructura
de pruebas para todo el proyecto.

**Alternativas consideradas.**

- *Tests de integración en `tests/`, con un archivo `.md` de fixture.* Rechazada:
  obligaría a exponer `markdown::parse` pensando en un consumidor externo
  estable, y a mantener un archivo aparte del caso que prueba, cuando el caso de
  entrada cabe en una línea de `&str`.
- *Añadir un framework de test (`rstest` o similar).* Rechazada sin más: son dos
  aserciones, no una matriz de casos, y ninguna dependencia nueva entra sin
  necesitarla.
- *`#[cfg(test)] mod tests` al final de `src/markdown.rs`, con `use super::*` y
  dos `#[test] fn`.* Elegida. Es lo que ya soporta `cargo test` sin nada
  adicional, y vive junto al código que prueba.

**Decisión.** Se adopta la tercera opción como patrón: los tests de este
proyecto viven en un módulo `#[cfg(test)]` dentro del propio archivo que
prueban, mientras quepan sin fixtures externos. Los dos tests de HU-01 cubren
exactamente los dos casos que `plan.md` documentó como defecto observado.

**Consecuencias.**

- `cargo test` ahora compila y ejecuta un target de test además del binario
  normal; el binario de producción no cambia (`cfg(test)` se descarta fuera de
  ese perfil).
- No hay integración continua que ejecute estos tests automáticamente todavía.
  Es una ausencia, no una decisión tomada; queda fuera del alcance de HU-01.
- El patrón no obliga a nada: un caso que sí necesite una matriz de entradas o
  un fixture real puede justificar entonces `rstest` o `tests/`, decidiéndolo en
  ese momento y no antes.

Estado: activa

---

## AD-18 — Cómo se activa un enlace al pulsarlo: `InteractiveText` con un contador de identificadores por documento

Fecha: 2026-08-24

**Contexto.** `StyledText`, lo que hoy pinta un párrafo o un encabezado, no
reacciona a nada. En `gpui` 0.2.2 (`src/elements/text.rs`), `InteractiveText`
envuelve un `StyledText` y añade `on_click(ranges: Vec<Range<usize>>, listener)`:
el listener recibe el índice del rango pulsado, no la URL, así que hace falta
emparejar cada rango con su URL en el mismo sitio donde ya se conocen los dos —
`render_spans`, que ya recorre los `Span` para construir el texto combinado y
sus estilos. `InteractiveText::new` además exige un `ElementId` único por
instancia, para que `gpui` conserve su estado (qué rango está bajo el cursor)
de un fotograma al siguiente.

**Alternativas consideradas.**

- *Derivar el id de un hash del contenido del bloque.* Rechazada: dos párrafos
  con el mismo texto colisionarían, y ya existe en este mismo archivo un patrón
  probado para esto — ver AD-10.
- *Envolver siempre en `InteractiveText`, tenga o no enlaces el bloque.*
  Rechazada: pagar el coste de hit-testing y de un `ElementId` por cada párrafo
  del documento cuando la inmensa mayoría no tiene enlaces. `render_spans` ya
  sabe, span a span, si hay alguna URL; comprobarlo antes de envolver es
  gratis.
- *Un contador global compartido entre bloques de código y bloques de texto.*
  Considerada; descartada porque numeran dos cosas distintas —una clave de
  `ScrollHandle`, un `ElementId` de clic— y compartir el contador haría que
  tocar uno perturbara la numeración del otro sin que nada lo avisara. Dos
  parámetros más en las firmas de `render_block`/`render_list` es el precio, y
  es barato.
- *Un contador `text_index: &mut usize`, hilvanado por `render_block` y
  `render_list` exactamente como ya hace `code_index` para las `ScrollHandle`
  de AD-10.* Elegida: mismo patrón ya en uso, sin inventar uno nuevo.

**Decisión.** `render_spans` devuelve, junto al `StyledText`, la lista de pares
`(rango de bytes, URL)` de los spans que forman parte de un enlace. Un nuevo
`render_text` envuelve ese resultado en `InteractiveText` solo cuando esa lista
no está vacía, con id `("text-block", n)`, `n` sacado de `text_index`. El
listener del clic resuelve el rango pulsado a su URL y llama a
`app::activate_link`.

**Consecuencias.**

- El contador reproduce la misma secuencia de ids en cada fotograma mientras
  `AppState.blocks` no cambie entre fotogramas, que es el caso hoy (se
  construye una vez, al cargar el documento). El día que el árbol de elementos
  pueda cambiar sin recrear `AppState` —una recarga en caliente, por
  ejemplo— este supuesto habría que revisarlo; no antes.

**Consecuencia añadida (2026-08-24), al cerrar CA-02.5.** Las celdas de tabla
(`render_table_row`) llamaban a `render_spans` directamente y solo usaban su
mitad `StyledText`: un enlace dentro de una tabla se veía distinguible (color y
subrayado) pero no era pulsable. Se registró como carencia conocida, no como
decisión aparte, con el argumento de que ninguna CA de HU-02 lo exigía. Ese
argumento resultó ser un error de alcance, no de arquitectura: RF-15.1 no
distingue dónde está escrito el enlace, y la lista de criterios que se comprobó
estaba incompleta frente al requisito. `historias.md` ganó CA-02.5 para
cerrarlo. La corrección **no cambió esta decisión, la completó**: `render_table`
y `render_table_row` ahora reciben el mismo `text_index: &mut usize` que ya
hilvanaban `render_block`/`render_list`, y cada celda pasa por `render_text`
igual que un párrafo o un encabezado, tomando su id de la misma secuencia. No
hay contador nuevo ni prefijo nuevo de `ElementId`: al ser una única secuencia
monótona compartida por párrafos, encabezados y ahora celdas, la unicidad
dentro de un documento se mantiene por construcción, sin necesidad de
coordinar dos contadores.

Estado: activa

---

## AD-19 — Con qué se abre un enlace externo: crate `open`, función `that_detached`, sin fijar versión exacta

Fecha: 2026-08-24

**Contexto.** RF-15.1 exige invocar al navegador predeterminado del sistema
para una URL `http`/`https`, sin bloquear el hilo de interfaz de GPUI
(CA-02.4) y sin que la URL pueda interpretarse como sintaxis de shell —viene
del cuerpo de un documento Markdown, que aquí se trata como dato no confiable.

**Alternativas consideradas.**

- *`std::process::Command::new("cmd").args(["/c","start","",url])`.*
  Rechazada: el intérprete de `cmd.exe` trata `&`, `^` y otros caracteres de la
  URL como sintaxis de shell; una URL con parámetros de consulta
  (`?a=1&b=2`) puede partirse o algo peor. Cero dependencias, pero insegura con
  una entrada que el documento controla.
- *`windows`/`windows-sys` llamando a `ShellExecuteW` directamente.* Rechazada:
  correcta, pero una dependencia grande para una sola llamada FFI cuando ya
  existe un crate pequeño y enfocado que la envuelve con seguridad.
- *Crate `open`, versión 5.4.2 (2026-08-24, MIT).* Elegida. Se leyó su
  implementación real para Windows (`src/windows.rs`): la ruta por omisión
  ejecuta `powershell.exe -NoProfile -NonInteractive -Command "Start-Process
  -FilePath $env:OPEN_RS_TARGET"`, pasando la URL por una **variable de
  entorno**, nunca interpolada en la cadena de comando, con `explorer.exe` como
  segundo intento. El propio test del crate
  (`default_open_does_not_embed_the_target_in_shell_code`) comprueba
  exactamente la inyección que esta elección evita al no ser `cmd /c start`. La
  característica `insecure`, que restauraría ese lanzador antiguo, **no está
  activada**. En el objetivo Windows, `cargo tree -p open` resuelve a cero
  dependencias transitivas, así que no compromete RNF-02.1 ni añade superficie
  de compilación.
- Dentro de `open`, `that()` frente a `that_detached()`. `that()` espera
  (`Command::status()`) a que el lanzador (PowerShell o Explorer) termine antes
  de devolver el control, y la propia documentación del crate avisa de que eso
  puede tardar «cientos de milisegundos» — tiempo de sobra para notarse como una
  congelación en el único hilo de interfaz de GPUI, justo lo que CA-02.4
  comprueba. `that_detached()` lanza el proceso y devuelve el control de
  inmediato (`Command::spawn()`, sin esperar). Elegida por eso.

**Decisión.** `open = "5.4.2"` en `Cargo.toml`, sin fijar versión exacta: AD-03
solo obliga a fijarla en las dependencias de interfaz —`gpui` y
`gpui-component`— y `open` no lo es; sigue el mismo criterio ya usado con
`pulldown-cmark`. `app::activate_link(url: &str)` comprueba que el esquema sea
`http://` o `https://` antes de llamar a `open::that_detached(url)`; cualquier
otro esquema (`mailto:`, un enlace relativo a otro `.md` —RF-14, otra
feature—) no hace nada por ahora. La comprobación de esquema y la llamada
viven las dos en `app`, no en `render`: `render` solo detecta el clic y le pasa
la URL a `crate::app::activate_link`, así que sigue sin necesitar saber qué
significa «activar un enlace», que es la frontera que fija
`03-arquitectura.md`. Su `io::Result` se descarta (`let _ = ...`): ningún
criterio pide un mensaje de error si falla, y CA-02.4 solo exige que MDView
siga respondiendo, que descartar el error ya garantiza sin recurrir a
`panic!`.

**Consecuencias.**

- No hay test para `activate_link`, conforme a la propia regla de AD-17:
  ejercitarla de verdad abre un navegador, lo que no cabe en un test sin
  fixtures, y los cuatro criterios de HU-02 son explícitamente de observación,
  no de código. Los cuatro se verificaron sobre la aplicación en marcha
  (ver `pruebas/` y el cierre de la historia).
- Si una feature futura necesita informar de un fallo al abrir el navegador
  («no hay navegador predeterminado configurado»), el `io::Result`
  descartado tendrá que convertirse en un valor devuelto o registrado. Se
  descarta hoy por los criterios de hoy, no porque abrir un enlace no pueda
  fallar nunca.

Estado: activa

---

## AD-20 — Cómo se muestran las imágenes: `gpui::img()`, tipo `Inline` propio, resolución en `markdown::parse`

Fecha: 2026-09-18

**Contexto.** RF-11.1 exige mostrar una imagen referenciada por ruta relativa
al directorio del `.md` (no al directorio de invocación, CA-03.2), y mostrar
el texto alternativo cuando no se pueda —archivo inexistente, sin permiso, o
no decodificable— sin que el resto del documento deje de verse (CA-03.3,
CA-03.4). Los dos «Pendientes» que dejó la sesión anterior ya estaban
investigados y se resuelven aquí.

**Qué decodifica la imagen.** `gpui::img(source)`, ya presente de forma
transitiva (`image` 0.25.10 entra por `gpui`/`gpui-component`, no por una
dependencia añadida a mano). Confirmado leyendo `elements/img.rs` de
`gpui` 0.2.2 el 2026-08-24 y otra vez hoy: `ImageSource::Resource(Resource::Path(_))`
carga con `fs::read` sin ningún cliente HTTP de por medio; `Resource::Uri`
es la única variante que llama a `client.get(...)`, y solo se alcanza si
`img()` se construye **desde una cadena** (`impl From<&str> for ImageSource`
intenta interpretarla como URI antes que como ruta). Por eso `render::render_image`
nunca llama a `img(String)`: siempre `img(PathBuf)` — `impl From<PathBuf> for
ImageSource` va directo a `Resource::Path`, sin pasar por esa ambigüedad.
`fs::read` ocurre dentro de `gpui`, no de `document` ni de `render`: la
frontera de `03-arquitectura.md` («`render` no conoce el sistema de
archivos») se mantiene porque `render` nunca abre el archivo él mismo, solo
le entrega a `gpui` una ruta ya resuelta.

**Dónde vive la ruta del documento y quién resuelve.** `app::run` calcula
`Path::new(path).parent()` —el directorio del `.md`, no `env::current_dir()`,
que es justo la distinción que exige CA-03.2— y se lo pasa a
`markdown::parse(source, base_dir)`. La resolución (unir `base_dir` con la
cadena que trae el Markdown) ocurre **dentro de `markdown`**, no en `app` ni
en `render`: es la única función que ya tiene ambos datos —el texto del
`.md` y, ahora, su directorio— en el mismo sitio, en el mismo momento en que
construye el árbol de elementos. `Path::join` es cómputo puro (no toca
disco), así que no contradice que `markdown` «no conozca el sistema de
archivos» en el sentido que le importa a esa frase: no lee nada, no abre
nada. `document` no cambia: sigue sin saber nada de rutas de imagen.

**Por qué una imagen `http`/`https` nunca se resuelve a una ruta.**
`01-alcance.md` y RNF-02.1 ya lo decidían; aquí se hace estructural:
`markdown::resolve_image` devuelve `None` para cualquier destino que empiece
por `http://` o `https://`, **antes** de intentar `Path::join` con él (que
además produciría una ruta sin sentido). `render::render_image` con
`resolved: None` nunca llama a `img()`: siempre muestra el texto
alternativo. No hay ninguna vía por la que una URL remota llegue a `img()`.

**Cómo entra la imagen en el árbol de elementos: `Inline`, no `Span` con un
campo más.** RF-08.2 (HU-01) resolvió los enlaces añadiendo un campo `url` a
`Span` (AD-16), porque un enlace es texto con una propiedad más. Una imagen
no es texto: mostrarla como una imagen de verdad —no su texto alternativo
haciendo de texto, que es lo que hacía el código hasta hoy— significa que el
párrafo que la contiene deja de ser una sola tira de texto. `Block::Paragraph`
pasa de `Vec<Span>` a `Vec<Inline>`, con `Inline::Span(Span)` e
`Inline::Image(ImageRef)`. Encabezados y celdas de tabla **no** ganan este
cambio: `parse_inline_spans` sigue produciendo `Vec<Span>` para ellos,
aplanando cualquier imagen a su texto alternativo (el comportamiento que ya
existía). Ninguna historia de esta feature pone una imagen en un encabezado o
una celda; ampliarlo el día que haga falta es un cambio local a esa función.

**Cómo se dibuja una imagen mezclada con texto en el mismo párrafo.**
`StyledText` de GPUI no puede intercalar una imagen de verdad dentro de una
tira de texto (es texto con estilos por tramo, no un flujo de nodos mixto).
`render::render_paragraph` agrupa los `Inline::Span` consecutivos en un
bloque de texto (igual que antes de esta historia) y cada `Inline::Image` en
su propio elemento `img()`, apilados en orden. El caso que piden los
criterios de HU-03 —una imagen sola en su propio párrafo, el patrón habitual
de un README— no pasa por ese apilado: `render_paragraph` lo detecta como
caso especial (`[Inline::Image(image)]`) y devuelve solo la imagen, sin ningún
contenedor de texto alrededor. Una imagen mezclada con texto en el mismo
párrafo se ve como texto y luego la imagen debajo, no en la misma línea: es
una degradación conocida, no probada por ningún CA de esta historia.

**Ajuste al ancho de columna (CA-03.5).** `render_image` fija
`.max_w(px(READING_WIDTH))` sobre el elemento `img()`. `gpui::img()` calcula
`style.aspect_ratio` a partir del tamaño decodificado antes de pedirle el
layout a Taffy (visto en `elements/img.rs`); combinado con `max_size.width`,
Taffy reduce ambas dimensiones manteniendo la proporción cuando el ancho
natural excede el máximo — el mismo mecanismo que `max-width` + `aspect-ratio`
en CSS. Verificado por observación, no solo leído: ver el resultado de
HU-03 en `04-calidad.md`.

**Consecuencias.**

- `Span` gana `#[derive(Clone)]`: `render_paragraph` necesita clonar los
  spans que va agrupando en un `Vec` propio antes de pasarlos a
  `render_text`, porque itera `&[Inline]` prestado.
- Los tres tests de `markdown.rs` que ya existían para `parse(source)` pasan
  a `parse(source, None)`; el test que cubría enlace+imagen en el mismo
  párrafo se reescribió porque la imagen ya no se aplana a texto (ver el
  commit). Se añadieron tres tests nuevos: imagen sola en su propio párrafo,
  resolución de ruta relativa contra `base_dir`, y que una URL remota nunca
  resuelve a una ruta local.
- `ImageCacheError` de `gpui` distingue archivo-no-encontrado, sin-permiso y
  formato-no-decodificable, pero `render_image` no lo inspecciona: los tres
  casos de CA-03.3/CA-03.4 piden el mismo resultado observable —el texto
  alternativo—, así que no hay necesidad de diferenciarlos. Si una historia
  futura pidiera un mensaje distinto por causa, ahí se abriría esa
  distinción, no antes.
- El cierre («fallback» del elemento `img()`) es una función `Fn() -> AnyElement`
  sin acceso al `cx` de GPUI (no es `'static`), así que el texto alternativo
  se muestra sin pasar por `render_spans`/el tema: un `div().child(alt)` liso.
  No es un problema para CA-03.3 —pide que se muestre el texto, no que tenga
  un estilo concreto— pero es la razón de que ese texto no lleve el color de
  cuerpo del tema como el resto del documento.

Estado: activa
