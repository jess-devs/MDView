# Decisiones — ver-un-documento

> Estado: borrador
> Última actualización: 2026-08-20
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

## Decisiones que ya se sabe que habrá que tomar

No son decisiones: son avisos de dónde van a aparecer, para que no se tomen por
descuido y sin dejar rastro.

- **Cómo se detecta el tema claro u oscuro de Windows** (RF-18, HU-04). Los dos
  caminos previsibles son leer la configuración del sistema directamente o usar
  lo que expongan GPUI o `gpui-component`. Se decide al implementar HU-04. No
  bloquea HU-01, HU-02 ni HU-03.
- **Qué biblioteca analiza el YAML del front matter** (RF-12). Pertenece a la
  feature `contenido-enriquecido`; no bloquea nada de esta.
- **Cómo se abren los enlaces externos** (RF-15). El crate `open`, versión 5.4.1
  publicada el 2026-08-05 y licencia MIT, es el candidato evidente, pero la
  decisión pertenece a la feature `contenido-enriquecido`.
- **Cómo se registra la asociación de `.md`**, con instalador o con auto-registro
  al arrancar. Sigue abierta en `01-alcance.md` y pertenece a la feature
  `integracion-con-windows`.
