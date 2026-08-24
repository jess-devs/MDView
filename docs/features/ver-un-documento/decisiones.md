# Decisiones — ver-un-documento

> Estado: borrador
> Última actualización: 2026-08-23
> Modo: new-project

Aquí se registra toda decisión que alguien pudiera querer revertir más adelante,
tomada mientras se construye esta feature. La prueba no es de qué categoría sea
la decisión, sino esta: ¿podría alguien llegar dentro de seis meses, mirar el
código, preguntarse por qué es así y no poder averiguarlo? Si la respuesta es sí,
va aquí. Eso incluye cambiar la forma de un tipo compartido, añadir una opción de
configuración o elegir dónde vive una responsabilidad nueva, no solo dependencias
y módulos.

Cada decisión se indexa además en `03-arquitectura.md`, en el mismo turno en que
se escribe. Nunca se borra una decisión superada: se marca.

Las siete primeras se tomaron en la Fase 4, antes de escribir una sola línea de
código. Los datos de crates.io que las sustentan se consultaron el 2026-08-20.

---

## AD-01 — Adoptar `gpui-component` como capa de componentes de interfaz

Fecha: 2026-08-20

**Contexto.** GPUI da ventana, dibujo y disposición, pero no trae componentes
hechos. MDView necesita barra de pestañas (feature 3), barra de desplazamiento
(RF-16), tema claro y oscuro (RF-18) y, más adelante, un editor de texto para la
edición aplazada. Construir todo eso sobre una biblioteca pre-1.0 es el grueso
del proyecto.

**Alternativas consideradas.**

- *Solo `gpui`, todo propio.* Control total y ninguna dependencia de terceros más
  allá de Zed. Rechazada: obliga a construir pestañas, desplazamiento y sistema
  de temas desde cero, que es más trabajo que el visor en sí, y a hacerlo sobre
  una API que aún no es estable.
- *`gpui-component`.* Elegida. Versión 0.5.1, publicada el 2026-02-05, licencia
  Apache-2.0, mantenimiento activo. Depende de `gpui ^0.2.2` **desde crates.io,
  sin dependencias de git**, así que adoptarla no empeora la situación de
  suministro respecto a usar `gpui` a secas. Trae además `html5ever`, que es
  exactamente lo que RF-13 necesitará, y `ropey`, la estructura de datos sobre la
  que se construye un editor de texto.
- *Otro framework de interfaz en Rust.* No se evaluó: el usuario fijó GPUI como
  restricción en Fase 1 y esa decisión no se reabre aquí.

**Decisión.** Se adopta `gpui-component`, con el conjunto de *features* de Cargo
reducido al mínimo: sin analizadores de tree-sitter por lenguaje, sin webview y
sin el inspector de desarrollo. RF-10 prohíbe expresamente colorear el código,
así que los analizadores de lenguaje no aportan nada y sí engordan el binario.

**Consecuencias.**

- Se hereda una dependencia de unas 48.000 líneas de una sola organización. Si
  dejara de mantenerse, MDView se queda con ella.
- `gpui-component` fija `ropey` en `=2.0.0-beta.1`, una versión beta clavada.
  No afecta a esta feature, pero es un olor a tener presente.
- El objetivo de RNF-01 se vuelve más delicado: hay que vigilar que el tamaño de
  la dependencia no se traduzca en tiempo de arranque. Es una razón más para la
  medida informal temprana que ya recoge `plan.md`.
- Cuando llegue la edición aplazada, su coste será mucho menor. Esto es
  exactamente el factor que `01-alcance.md` pedía que se pesara en esta decisión.

Estado: activa

---

## AD-02 — Analizar el Markdown con `pulldown-cmark` propio, no con el renderizador de `gpui-component`

Fecha: 2026-08-20

**Contexto.** `gpui-component` incluye su propio renderizador de Markdown. Usarlo
ahorraría casi toda HU-02.

**Alternativas consideradas.**

- *Usar el renderizador de Markdown de `gpui-component`.* Rechazada como camino
  principal por dos requisitos que exigen control sobre cómo se traduce cada
  elemento: RF-12 obliga a presentar el front matter como una tabla de
  propiedades en lugar de como texto, y RF-13 fija un subconjunto **cerrado** de
  etiquetas HTML con una regla concreta para las que quedan fuera. Un
  renderizador de terceros hace lo que hace; doblarlo a estas dos reglas puede ir
  de trivial a imposible, y no se sabe cuál de las dos cosas es.
- *`pulldown-cmark` propio.* Elegida. Versión 0.13.4, publicada el 2026-05-20,
  licencia MIT, 137 millones de descargas acumuladas. Es el analizador de
  CommonMark de referencia en Rust y trae como extensiones las tablas, las listas
  de tareas y el tachado que pide RF-09.
- *Escribir el analizador de Markdown desde cero.* Rechazada sin discusión: es
  trabajo resuelto por otros y hacerlo mal se paga en cada documento.

**Decisión.** `document` y `markdown` usan `pulldown-cmark` y producen el árbol de
elementos propio descrito en `03-arquitectura.md`. El renderizador de Markdown de
`gpui-component` no se usa; sí se usan sus componentes de interfaz, según AD-01.

**Consecuencias.**

- HU-02 y HU-03 son trabajo real: hay que traducir cada elemento a componentes.
  Se aceptaron ese coste y ese riesgo a cambio de poder cumplir RF-12 y RF-13.
- **Esta decisión está tomada sin comprobar** hasta qué punto el renderizador de
  `gpui-component` es moldeable. Antes de empezar HU-02 conviene dedicar un rato
  a mirarlo: si resultara que admite las dos reglas, esta decisión debería
  revisarse y marcarse como superada, no defenderse por haberla escrito antes.
- RF-12 y RF-13 pertenecen a la feature 2, no a esta. La decisión se toma ahora
  porque determina cómo se construye HU-02, que sí es de esta.

**Revisión (2026-08-23), antes de empezar HU-02.** Se leyó el código real de
`crates/ui/src/text/` en el repositorio de `gpui-component` (versión v0.5.1
exacta, la que fija AD-03), no solo su documentación.

- *RF-12 (front matter como tabla de propiedades).* `format/markdown.rs` no
  tiene ningún tratamiento de front matter: pasa el texto crudo directamente a
  `markdown::to_mdast` (el analizador CommonMark de la crate `markdown`, no
  `pulldown-cmark`). Un `---` inicial se interpretaría como regla horizontal
  seguida de un párrafo, no como propiedades. Pero esto resulta ser irrelevante
  para la decisión: extraer el front matter es una operación de texto sobre la
  cadena cruda, previa a cualquier analizador de Markdown, así que no depende
  de si el renderizador es el propio o el de `gpui-component`. RF-12 no es un
  argumento a favor ni en contra de esta decisión.
- *RF-13 (subconjunto cerrado de HTML).* Este es el argumento real, y se
  confirma con evidencia concreta. El HTML embebido dentro de Markdown (tanto
  en bloque como en línea) se resuelve en `format/markdown.rs` llamando a
  `format::html::parse`, que reconoce un conjunto de etiquetas fijado en un
  `match` interno: `em`/`i`, `strong`/`b`, `del`/`s`, `code`, `a`, `img` en
  línea, y `br`, `h1`-`h6`, `ul`, `ol`, `li`, `table`, `blockquote` en bloque.
  Comparado con la lista cerrada de RF-13 (`img`, `a`, `b`, `strong`, `i`,
  `em`, `code`, `br`, `hr`, `p`, `ul`, `li`, `table`, `details`, `summary`,
  `div` con alineación centrada):
  - Le faltan cinco etiquetas que RF-13 exige con formato propio: `hr`, `p`,
    `details`, `summary`, `div` centrado.
  - Formatea de más cinco que RF-13 exige mostrar como texto plano sin su
    marcado por no estar en la lista: `h1`-`h6`, `del`/`s`, `ol`, `blockquote`.
  - La función que decide esto es `pub(crate)`: no hay ninguna API pública
    para sustituir o ampliar esa tabla de etiquetas desde fuera de la crate.
    Esto coincide con lo que la propia documentación de `TextView` declara
    como objetivo explícito: *"Not Goals: Customization of the complex style
    ... If you want to like this, you must fork your version."*

**Conclusión.** AD-02 se mantiene activa, ahora con evidencia en vez de con
la sospecha con la que se tomó. `gpui-component` no es moldeable a la lista
cerrada de RF-13 sin forkear la dependencia, que es justo el coste que esta
decisión ya había aceptado pagar por otra vía (renderizador propio). No se
marca como superada.

Estado: activa

---

## AD-03 — Fijar versiones exactas de `gpui` y `gpui-component`

Fecha: 2026-08-20

**Contexto.** GPUI es pre-1.0 y su API puede romperse entre versiones menores. En
crates.io lleva en 0.2.2 desde el 2025-10-22: diez meses sin publicar. Cuando
salga la siguiente, el salto puede ser grande.

**Alternativas consideradas.**

- *Rangos de compatibilidad (`^0.2.2`).* Lo normal en Rust. Rechazada: con una
  biblioteca pre-1.0, `^` permite saltos que sí rompen, y romperían la
  compilación en mitad de una historia.
- *Versiones exactas (`=0.2.2`, `=0.5.1`).* Elegida.

**Decisión.** Ambas dependencias se fijan con `=`. La versión no se actualiza
mientras haya una historia abierta; actualizar es una tarea con su propio momento,
nunca un efecto colateral de otra cosa.

**Consecuencias.** Las mejoras y correcciones de las dependencias no llegan
solas: hay que ir a por ellas. A cambio, ninguna historia se rompe por algo que
no se tocó. Cuando se actualice, el cambio de versión se registra aquí como
decisión propia.

Estado: activa

---

## AD-04 — Separar el proyecto en cuatro módulos con GPUI confinado a uno

Fecha: 2026-08-20

**Contexto.** Hay que decidir dónde vive cada responsabilidad antes de escribir
código, o acabará viviendo donde caiga.

**Alternativas consideradas.**

- *Un solo módulo.* Para siete historias podría bastar. Rechazada porque el
  proyecto ya tiene cuatro features previstas y una quinta aplazada, y porque
  mezclar la lectura del archivo con el dibujo hace imposible razonar sobre el
  Markdown sin arrancar una ventana.
- *Separar por capas técnicas al estilo de un servidor.* Rechazada: no hay
  persistencia ni servicios; sería ceremonia vacía.
- *Cuatro módulos: `document`, `markdown`, `render`, `app`.* Elegida. La regla
  que los ordena es que GPUI y `gpui-component` solo se conocen desde `render`.

**Decisión.** La de la tabla de fronteras de `03-arquitectura.md`.

**Consecuencias.** Añadir Linux o cambiar de biblioteca de componentes toca
`render` y `app` y no toca `document` ni `markdown`. A cambio, hay que definir y
mantener el árbol de elementos intermedio, que es trabajo que un enfoque directo
se ahorraría. Si el árbol acaba necesitando detalles de dibujo, la frontera
estaba mal puesta y hay que registrarlo, no agujerearla.

Estado: activa

---

## AD-05 — El solo lectura es un estado explícito del modelo, no la ausencia de edición

Fecha: 2026-08-20

**Contexto.** `01-alcance.md` aplazó la edición con confirmación a una versión
posterior y dejó escrita una obligación para esta fase: que el diseño le deje
sitio, de modo que añadirla luego no obligue a rehacer lo construido.

**Alternativas consideradas.**

- *No hacer nada ahora.* La aplicación no edita, luego todo es de solo lectura
  implícitamente. Rechazada: es justo lo que la obligación pedía evitar. Cuando
  llegara la edición habría que introducir el concepto de modo por todas partes
  a la vez.
- *Construir ya la maquinaria de edición desactivada.* Rechazada por el extremo
  contrario: sería construir la función aplazada, que es precisamente lo que se
  decidió no hacer ahora.
- *Un estado explícito con un solo valor posible.* Elegida.

**Decisión.** El estado de la aplicación lleva un campo de modo cuyo tipo tiene
hoy un único valor, el de solo lectura. Las operaciones que en el futuro
modificarían el documento no existen todavía; lo que existe es el sitio donde
preguntar por el modo.

**Consecuencias.** Hoy es un tipo con un solo valor, que parece innecesario y a
un lector desprevenido le sobrará. Por eso está escrito aquí. Mañana, añadir la
edición consiste en añadir un valor y las operaciones que lo acompañan, sin
recorrer el resto del código introduciendo el concepto.

Estado: activa

---

## AD-06 — Compilar como aplicación de ventanas de Windows, sin consola

Fecha: 2026-08-20

**Contexto.** En Windows, un ejecutable de Rust se compila por omisión como
aplicación de consola. Eso hace que, al abrir un `.md` con doble clic (RF-02),
aparezca además una ventana negra de consola. Pero RF-01 exige poder invocarlo
desde la línea de comandos, lo que hace pensar que la consola hace falta.

**Alternativas consideradas.**

- *Subsistema de consola.* Rechazada: cumpliría RF-01 pero ensuciaría RF-02 con
  una ventana de consola en cada doble clic, que es el caso principal de uso.
- *Subsistema de ventanas, adjuntándose a la consola del proceso padre cuando la
  haya.* Permitiría escribir mensajes por la salida estándar al invocarlo desde
  una terminal. Rechazada por innecesaria: RF-17 ya establece que los errores se
  comunican con un aviso dentro de la aplicación, no por consola.
- *Subsistema de ventanas, sin consola en absoluto.* Elegida.

**Decisión.** El binario se compila como aplicación de ventanas. Los argumentos
de la línea de comandos se siguen leyendo con normalidad, que es lo que RF-01
necesita; lo que no hay es salida por consola.

**Consecuencias.** No se puede depurar imprimiendo por pantalla. Esa es una de
las razones por las que AD-07 vuelca las medidas a un fichero en lugar de
imprimirlas.

Estado: activa

---

## AD-07 — Medir el arranque con dos marcas de tiempo volcadas a un fichero, activadas por variable de entorno

Fecha: 2026-08-20

**Contexto.** CA-06.1 exige medir el tiempo hasta el **documento visible**, y
`plan.md` deja escrito que ni un cronómetro ni `Measure-Command` sirven, porque
ninguno sabe cuándo hay un fotograma pintado con texto dentro. Además, por AD-06
no hay consola donde imprimir el resultado.

**Alternativas consideradas.**

- *Imprimir por salida estándar.* Imposible con AD-06.
- *Mostrar la cifra en la propia ventana.* Rechazada: obligaría a que la
  aplicación enseñe al usuario final algo que solo interesa al desarrollador, o a
  construir un modo de depuración visible.
- *Volcar a un fichero, activado por variable de entorno.* Elegida.

**Decisión.** Si la variable de entorno `MDVIEW_TIMING` contiene una ruta, la
aplicación añade a ese fichero dos marcas —la de entrada al programa y la del
primer fotograma que ya contiene el documento renderizado— y su diferencia. Si la
variable no está definida, no se mide, no se abre ningún fichero y no se paga
nada.

**Consecuencias.** La verificación de CA-06.1 consiste en definir la variable,
arrancar en frío tres veces y leer el fichero, lo que además deja las tres cifras
por escrito sin depender de que nadie las apunte. El fichero generado durante la
verificación es dato de prueba: se borra al cerrar HU-06, según la sección de
limpieza de `plan.md`.

Estado: activa

---

## AD-08 — Ancho máximo de lectura: 720 px

Fecha: 2026-08-23

**Contexto.** RF-20 exige que la columna de texto deje de crecer a partir de un
ancho máximo de lectura, pero deja el valor concreto sin fijar a propósito
(CA-01.5). Alguien tiene que elegirlo al implementar HU-01.

**Alternativas consideradas.**

- *Un valor en caracteres (medida tipográfica, ~65-75 caracteres por línea),
  recalculado según la fuente.* Más correcto tipográficamente, pero
  `gpui`/`gpui-component` no exponen todavía en esta feature una forma sencilla
  de medir el ancho de un carácter para convertirlo a píxeles antes de layout;
  habría que resolverlo con un componente de texto más elaborado que no hace
  falta para HU-01. Aplazada: si se necesita más adelante, esta decisión se
  marca superada.
- *720 px fijos.* Elegida. Es la cifra que usan como referencia habitual sitios
  de lectura continua de prosa técnica (documentación, artículos), y con el
  tamaño de letra de párrafo por defecto (16 px) cae dentro del rango de 65-75
  caracteres por línea que la tipografía editorial considera cómodo de leer.
- *Ancho de ventana completo, sin límite.* Rechazada: es justo lo que RF-20
  prohíbe; en monitores anchos las líneas se volverían incómodamente largas.

**Decisión.** La columna de texto tiene `max-width: 720px`, centrada con
márgenes automáticos a ambos lados cuando la ventana es más ancha.
Implementado en `render.rs` como la constante `READING_WIDTH`.

**Consecuencias.** Es un número fijo, no derivado del tamaño de fuente: si
HU-02 o HU-04 cambian la tipografía base de forma notable, hay que revisar si
720 px sigue siendo la medida cómoda, y registrarlo como revisión de esta
decisión si cambia.

Estado: activa

---

## AD-09 — Valores tipográficos y visuales de HU-02

Fecha: 2026-08-23

**Contexto.** RF-08/RF-09 y sus criterios (CA-02.1 a CA-02.10) exigen que los
elementos se *distingan* entre sí (tamaños decrecientes, negrita/cursiva
distinguibles, casillas marcadas/sin marcar, cabecera de tabla distinguible,
etc.) sin fijar cifras ni glifos concretos. Alguien tiene que elegirlos al
implementar. Se agrupan aquí porque son decisiones pequeñas y de la misma
naturaleza (valores de estilo, no de arquitectura), no porque compartan una
sola alternativa considerada.

**Decisión.**

- **Escala de encabezados**, implementada en `render.rs` como `heading_size`:
  H1 32px, H2 28px, H3 24px, H4 20px, H5 18px, H6 16px (= tamaño de párrafo).
  Escala arbitraria pero estrictamente decreciente, que es lo único que pide
  CA-02.1; no se derivó de un sistema tipográfico formal.
- **Tamaño de párrafo base**: 16px, constante `BODY_SIZE`.
- **Marcadores de lista**: viñeta `•` para listas no ordenadas, `N.` para
  ordenadas (respetando el número inicial de RF-08 si el Markdown lo fija),
  y las casillas Unicode `☐`/`☑` para listas de tareas en vez de un
  componente de casilla interactivo de `gpui-component` — no hace falta que
  sea interactiva (RF-08 es de solo lectura, AD-05) y el glifo ya cumple
  CA-02.9 sin añadir superficie de la dependencia.
- **Cita**: `padding-left` de 16px (`pl_4`) más un borde izquierdo de 2px
  (`border_l_2`) del color `theme.border`.
- **Regla horizontal**: línea de 1px de alto con fondo `theme.border`.
- **Tabla**: filas como `flex` con columnas `flex_1` (mismo ancho, alineadas
  entre sí); la fila de cabecera lleva fondo `theme.muted` y un borde inferior
  de 2px, las filas de cuerpo un borde inferior de 1px, ambos en
  `theme.border`. Sin líneas verticales: no lo exige CA-02.8 y añadirlas es
  más superficie visual sin más información.
- **Código en línea**: familia `theme.mono_font_family` (ya provista por
  `gpui-component`, RF-10 la necesitará igual en HU-03) y fondo
  `theme.muted`, aplicados por *run* de texto (ver nota técnica más abajo).

**Nota técnica que motiva parte de lo anterior.** `gpui::TextRun` (la unidad
con la que `StyledText` mezcla estilos dentro de un mismo párrafo) no tiene
campo de tamaño de fuente: ni `TextRun` ni `Font` lo llevan. El tamaño de
fuente solo se puede fijar de forma ambiental, con `.text_size()` en el `div`
que envuelve el texto — por eso la negrita/cursiva/tachado/código sí varían
por tramo dentro de un párrafo (vía `HighlightStyle`), pero el tamaño no: cada
bloque (encabezado, párrafo, celda) es una única llamada a `render_spans` con
un tamaño uniforme. No hace falta más para HU-02: ningún criterio pide mezclar
tamaños dentro de la misma línea.

**Consecuencias.** Ninguno de estos valores está pensado como definitivo; son
la primera cifra razonable, no un sistema de diseño. Si HU-04 (tema del
sistema) o una revisión visual posterior los cambia, se anota aquí como
revisión, no se reabre esta decisión por sorpresa.

Estado: activa

---

## AD-10 — Scroll horizontal de los bloques de código: `ScrollHandle` propio por bloque, no la envoltura automática de `gpui-component`

Fecha: 2026-08-23

**Contexto.** CA-03.5 y CA-03.6 exigen que cada bloque de código se desplace
horizontalmente por sí solo, sin mover el resto del documento ni a otros
bloques de código. `gpui-component` ofrece un método de conveniencia,
`overflow_x_scrollbar()`, que añade de una vez el scroll y su barra visual.

**Alternativas consideradas.**

- *`overflow_x_scrollbar()` de `gpui-component`, dentro del bucle que
  renderiza los bloques.* Rechazada tras comprobarlo: esta envoltura deriva
  el identificador de su estado de desplazamiento con
  `#[track_caller]`/`Location::caller()`, es decir, del punto del código
  fuente donde se llama — el mismo para todas las llamadas dentro de un
  bucle. Con un solo bloque de código no se nota; con dos o más, **todos
  comparten la misma posición de scroll**: desplazar uno desplaza a los
  demás. Se comprobó abriendo el documento de prueba con dos bloques de
  código y desplazando el primero: el segundo se movía a la vez.
- *`ScrollHandle` propio, guardado con `window.use_keyed_state` bajo una
  clave que incluye el índice del bloque, más los métodos de más bajo nivel
  `overflow_x_scroll()` + `track_scroll(&handle)` + `.scrollbar(&handle,
  ScrollbarAxis::Horizontal)`.* Elegida. Cada bloque de código lleva su
  propio índice (contador que recorre `render_block` en orden), así que cada
  uno obtiene un `ScrollHandle` independiente.

**Decisión.** Los bloques de código no usan `overflow_x_scrollbar()`. Cada
uno construye su propio `ScrollHandle` con una clave `("code-block-scroll",
índice)`, y la estructura de su `div` replica la que usa internamente
`Scrollable::render` de `gpui-component`: un contenedor `.relative()`, dentro
un área de scroll `.flex().flex_row().overflow_x_scroll().track_scroll(...)`
con el texto como hijo `.flex_1()`, y la barra de scroll como hijo hermano
del área de scroll (no anidada dentro de ella) vía `.scrollbar(...)`.
Implementado en `render.rs`.

**Consecuencias.**

- Replicar esta estructura a mano fue necesario porque `gpui-component` no
  expone una variante de `overflow_x_scrollbar()` que acepte un identificador
  explícito; la única forma de evitar la colisión es no usar esa envoltura.
  Si una versión futura de `gpui-component` añade esa variante, esta decisión
  se revisa.
- La estructura importa: anidar la barra de scroll *dentro* del área con
  overflow (en vez de como hermana) impide que se vea o reciba eventos —lo
  primero que se probó, y falló—. Cualquiera que toque este código debe
  conservar los tres niveles (contenedor relativo / área de scroll / barra
  hermana), no simplificarlos de vuelta a uno solo.
- La rueda del ratón vertical simple **no** desplaza un bloque que solo
  tiene overflow horizontal; hace falta rueda horizontal real (Shift+rueda en
  la mayoría de ratones, gesto horizontal de trackpad) o arrastrar la barra.
  Esto no es una decisión de diseño, es una limitación de GPUI observada al
  verificar, y queda anotada por si alguien la redescubre y cree que es un
  bug nuevo.

Estado: activa

---

## AD-11 — Detectar el tema con `window.appearance()` de GPUI, no leyendo el registro de Windows

Fecha: 2026-08-23

**Contexto.** RF-18 exige adoptar el tema claro u oscuro de Windows al
arrancar. `plan.md` dejaba anotados dos caminos previsibles: leer la
configuración del sistema directamente (registro de Windows,
`AppsUseLightTheme`) o usar lo que expusieran GPUI/`gpui-component`. Además,
al verificar HU-02 se observó que la aplicación ya arrancaba en oscuro sin
que se hubiera escrito código de tema, lo que hacía sospechar que
`gpui-component` ya resolvía esto solo.

**Investigación.** Se leyó el código real de `gpui-component` v0.5.1:
`gpui_component::init(cx)` llama a `Theme::sync_system_appearance(None, cx)`,
que internamente usa `cx.window_appearance()` de GPUI —una API nativa de la
plataforma, no una lectura manual del registro—. Eso explica el
comportamiento ya observado en HU-02. Sin embargo, esa llamada ocurre una
sola vez, **antes de que exista ninguna ventana** (en `Application::run`,
antes de `cx.open_window`), así que depende de qué valor devuelva
`cx.window_appearance()` sin ventana todavía; y no hay ninguna suscripción a
cambios de tema en caliente por defecto.

**Alternativas consideradas.**

- *Leer `AppsUseLightTheme` del registro de Windows directamente.* Rechazada:
  `gpui-component` ya resuelve la detección inicial a través de una API de
  plataforma de GPUI, más portable que leer una clave de registro específica
  de Windows a mano, y evita duplicar lo que la dependencia ya hace.
- *Confiar en la sincronización única de `gpui_component::init` sin tocar
  nada más.* Rechazada: no hay evidencia de que se actualice si el usuario
  cambia el tema de Windows mientras MDView está abierto, y la sincronización
  inicial ocurre antes de que la ventana exista.
- *Volver a sincronizar el tema contra la ventana real al abrirla, y
  suscribirse a cambios en caliente con `window.observe_window_appearance`.*
  Elegida.

**Decisión.** En el cierre de `cx.open_window` (`app.rs`), tras crear la
ventana, se llama a `Theme::sync_system_appearance(Some(window), cx)` para
resincronizar contra la ventana ya real, y se registra
`window.observe_window_appearance(...)` para repetir esa sincronización cada
vez que el sistema operativo avise de un cambio de apariencia, mientras la
aplicación esté abierta.

**Consecuencias.**

- RF-18 solo exige adoptar el tema **al arrancar**; el añadido de
  seguimiento en caliente es más de lo que pide el requisito, pero es una
  línea de más sobre una suscripción que GPUI ya ofrece, y se verificó que
  funciona: cambiar el tema de Windows con MDView ya abierto lo actualiza sin
  reiniciar (ver `04-calidad.md`, HU-04).
- Si `gpui-component` cambiara su propio `init` para suscribirse sola en el
  futuro, esta resincronización manual sería redundante pero inofensiva; se
  revisaría entonces.

Estado: activa

---

## AD-12 — Avisos de error con el componente `Notification` de `gpui-component`, mensaje y clasificación propios

Fecha: 2026-08-23

**Contexto.** RF-17/CA-05.x exigen un aviso temporal que nombre el archivo y
la causa, y que desaparezca solo (CA-05.4). `document::load` hasta HU-04
devolvía `io::Result<String>` y `app.rs` lo descartaba con `.ok()`
—exactamente el punto que `03-arquitectura.md` señala como el lugar donde el
error debe empezar a subir como valor en vez de silenciarse—.

**Decisión.**

- `document::LoadError` (nuevo, en `document.rs`) clasifica la lectura
  fallida en las tres causas que pide RF-17 (`NotFound`,
  `PermissionDenied`, `NotText`) más un cajón de sastre `Unreadable` para
  cualquier otro error de E/S que `fs::read` pudiera devolver (p. ej. la
  ruta es un directorio) y que ningún criterio de esta feature ejercita.
  `document` solo clasifica; no redacta el texto que se muestra, según la
  frontera de responsabilidades de `03-arquitectura.md`.
- `app.rs` traduce esa clasificación a un mensaje en español que nombra el
  archivo, y lo deja en `AppState.pending_notice`.
- `render.rs` lo consume una sola vez, en el primer `render()`, con
  `window.push_notification(Notification::error(mensaje), cx)` — el
  componente de `gpui-component`, con su comportamiento por omisión de
  auto-ocultarse a los 5 segundos (`autohide` por defecto), en vez de
  construir un aviso propio con temporizador manual.

**Alternativas consideradas.**

- *Cronómetro propio con `Timer` y un campo de visibilidad en el estado.*
  Rechazada: reimplementaría lo que `Notification`/`NotificationList` ya
  hacen, incluida la animación de aparición/desaparición.
- *Duración de auto-ocultado distinta de 5 s.* No se cambió: CA-05.4 solo
  exige que desaparezca solo, no en cuánto tiempo, y 5 s es razonable para
  leer una frase corta.

**Nota técnica.** `Root` (el componente raíz que ya usa `app.rs`) **no**
pinta las notificaciones por sí solo: `impl Render for Root` no incluye la
capa de notificaciones en su árbol. Hay que componerla a mano llamando a la
función asociada `Root::render_notification_layer(window, cx)` desde la
vista de nivel superior propia (`DocumentView::render`) y añadir su
resultado (`Option<impl IntoElement>`) como hijo. Sin este paso,
`push_notification` actualiza el estado pero no se pinta nada — así es como
se descubrió, verificando: la primera versión no tenía esta línea y el aviso
nunca aparecía, aunque el código compilaba y no fallaba.

**Consecuencias.** Si más adelante se necesitan diálogos o *sheets* de
`gpui-component` (no previstos en esta feature), la misma composición manual
hace falta para `Root::render_dialog_layer` y `Root::render_sheet_layer`.

Estado: activa

---

## Decisiones que ya se sabe que habrá que tomar

No son decisiones: son avisos de dónde van a aparecer, para que no se tomen por
descuido y sin dejar rastro.

- **Qué biblioteca analiza el YAML del front matter** (RF-12). Pertenece a la
  feature `contenido-enriquecido`; no bloquea nada de esta.
- **Cómo se abren los enlaces externos** (RF-15). El crate `open`, versión 5.4.1
  publicada el 2026-08-05 y licencia MIT, es el candidato evidente, pero la
  decisión pertenece a la feature `contenido-enriquecido`.
- **Cómo se registra la asociación de `.md`**, con instalador o con auto-registro
  al arrancar. Sigue abierta en `01-alcance.md` y pertenece a la feature
  `integracion-con-windows`.
