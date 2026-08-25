# Decisiones — enlaces-e-imagenes

> Estado: en construcción, HU-01 implementada
> Última actualización: 2026-08-24
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

## Pendientes de esta feature

- **Cómo se abren los enlaces externos** (RF-15.1, HU-02). El crate `open` es el
  candidato evidente. Consultado crates.io el 2026-08-24: la última versión es
  la **5.4.2**, publicada ese mismo día, licencia MIT. En Windows sus
  dependencias directas se reducen a `dunce` (opcional); `is-wsl` y `libc` son
  solo de Unix. No arrastra nada que hable por red, así que no compromete
  RNF-02.1. Queda por decidir si se fija la versión exacta: AD-03 obliga a ello
  solo con las dependencias de interfaz, y `open` no lo es.
  (`../ver-un-documento/decisiones.md` anunciaba la 5.4.1 del 2026-08-05, que
  era la última cuando se escribió aquello.)
- **Dónde vive la ruta del archivo abierto** (RF-11.1, HU-03). Hoy
  `document::load` devuelve solo el texto, y resolver una ruta relativa exige
  saber en qué directorio está el `.md`. Toca la frontera entre `document`,
  `markdown` y `render` que describe `../../03-arquitectura.md`.
- **Con qué se decodifican las imágenes** (RF-11.1, HU-03). Sea lo que sea,
  `../../03-arquitectura.md` exige que ninguna dependencia que abra conexiones
  entre sin justificarlo, y RNF-02.1 fija cero conexiones. Leído en el código de
  `gpui` 0.2.2 el 2026-08-24, hay dos cosas que condicionan esta decisión:

  1. **`img()` construido desde una cadena puede salir a la red.** En
     `elements/img.rs`, `impl From<&str> for ImageSource` manda la cadena a
     `Resource::Uri` si parsea como URI, y la carga de `Resource::Uri` hace
     `client.get(...)`: una petición HTTP real. `Resource::Path` hace `fs::read`.
     La conclusión para HU-03 es que la ruta hay que resolverla a un `PathBuf`
     absoluto y pasar eso, nunca el destino crudo que venga del Markdown.
  2. **Hoy MDView no puede hacer peticiones, y nadie lo decidió.** `src/app.rs`
     usa `Application::new()`, y el `App` por omisión instala `NullHttpClient`,
     cuyo `send` devuelve error en lugar de conectarse. Eso respalda RNF-02.1 de
     forma estructural, pero es una propiedad accidental: conviene convertirla en
     decisión registrada al abrir HU-03 o HU-04, porque cualquiera puede añadir
     `with_http_client` por un motivo razonable y romper RNF-02.1 en silencio.
     **No es evidencia para dar CA-04.1 por `pasa`**: que una biblioteca
     garantice algo no es observación.
