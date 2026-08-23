# Alcance

> Estado: borrador
> Última actualización: 2026-08-20
> Modo: new-project

El alcance descrito aquí es el de la **primera versión** de MDView. Los términos
usados (documento, pestaña, instancia única, arranque en frío, documento visible,
GFM, front matter, aviso temporal, subconjunto de HTML) están definidos en el
glosario de `00-contexto.md`.

Este documento se amplió durante la Fase 2, al responderse seis preguntas que
estaban abiertas. Lo que entró en el alcance por esas respuestas queda recogido
en la sección de preguntas cerradas. Después, y antes de trocear el trabajo, se
añadió la sección de funciones aplazadas: cosas decididas y pospuestas, que no
son lo mismo que cosas descartadas.

## Dentro del alcance

**Renderizado**

- Renderizar un documento que use CommonMark: encabezados de nivel 1 a 6,
  párrafos, énfasis y negrita, listas ordenadas y no ordenadas con anidamiento,
  citas, reglas horizontales, código en línea y bloques de código.
- Renderizar los añadidos de GFM: tablas, listas de tareas y texto tachado.
- Mostrar los bloques de código en tipografía monoespaciada y con fondo
  diferenciado, **sin coloreado por lenguaje**. Una línea más ancha que la
  ventana se conserva entera y el bloque se desplaza horizontalmente por sí
  solo, sin arrastrar al resto del documento.
- Mostrar las imágenes referenciadas mediante ruta relativa al archivo `.md` que
  se está viendo.
- Mostrar los enlaces con aspecto de enlace, distinguibles del texto normal.
- Presentar el front matter del documento, si lo tiene, como una tabla de
  propiedades en la parte alta del documento, en lugar de como texto en crudo.
- Interpretar un subconjunto cerrado de etiquetas HTML incrustadas en el
  Markdown: `img`, `a`, `b`, `strong`, `i`, `em`, `code`, `br`, `hr`, `p`, `ul`,
  `li`, `table`, `details`, `summary`, y `div` con atributo de alineación
  centrada. Los atributos de estilo y las hojas de estilo se ignoran.
- Desplazamiento vertical dentro del documento.
- Reajustar el texto al ancho de la ventana cuando esta cambia de tamaño,
  hasta un ancho máximo de lectura, a partir del cual la columna de texto se
  mantiene centrada sin seguir creciendo.

**Apariencia**

- Adoptar el tema claro u oscuro que tenga configurado Windows.

**Apertura de archivos**

- Abrir uno o varios archivos pasados como argumentos en la línea de comandos.
- Si se invoca sin ninguna ruta, mostrar una ventana que indique qué es MDView
  y por qué medios se le puede pedir que abra un archivo.
- Registrar MDView en Windows como aplicación capaz de abrir archivos `.md`, de
  modo que el doble clic en el Explorador la use.
- Comportarse como **instancia única**: si ya hay un proceso de MDView en
  ejecución, abrir otro archivo enfoca esa ventana y añade el documento en una
  pestaña nueva, que pasa a ser la pestaña activa; el documento que estaba
  abierto permanece en la suya.

**Pestañas**

- Abrir un documento en una pestaña nueva, cambiar de pestaña y cerrar una
  pestaña.
- Mostrar en cada pestaña el nombre del archivo.
- Al cerrar la última pestaña abierta, la aplicación termina.
- Al pulsar un enlace relativo que apunte a otro archivo `.md`, abrirlo en una
  pestaña nueva.
- Al pulsar un enlace `http` o `https`, abrirlo en el navegador predeterminado
  del sistema.

**Errores**

- Si un archivo no existe, no se puede leer o no es texto, no se abre pestaña
  para él y se muestra un aviso temporal que identifica el archivo y la causa.

**Rendimiento**

- Desde el doble clic hasta el documento visible, en arranque en frío y con un
  archivo de unos 50 KB, transcurre menos de 1 segundo.

**Plataforma**

- Windows.

## Fuera del alcance

Todo lo siguiente se consideró y se excluyó a propósito.

| Excluido | Por qué |
| --- | --- |
| Editar y guardar el archivo | **Aplazado, no descartado.** Ver la sección de funciones aplazadas. La justificación anterior decía que editar convertiría MDView en otro producto; se reconsideró el 2026-08-20 y se consideró floja, porque no poder corregir una errata obliga a abrir el IDE que la aplicación quiere evitar. |
| Búsqueda dentro del documento | Se descartó para que la primera versión sea la mínima que ya sirve. Candidata clara para la segunda. |
| Índice o tabla de contenidos lateral | Valioso, pero no necesario para leer un documento. Se reconsideró el 2026-08-20, al concretarse qué debía mostrar el visualizador de estructura, y el usuario lo descartó de nuevo, tanto en su forma de árbol de encabezados como de árbol de la estructura completa. |
| Ver el Markdown en crudo | **Aplazado, no descartado.** Ver la sección de funciones aplazadas. |
| Recarga automática al cambiar el archivo en disco | Añadiría un vigilante del sistema de archivos que la primera versión no necesita, y con un arranque de menos de un segundo, volver a abrir el documento cuesta poco. |
| Diálogo de apertura dentro de la aplicación | Se decidió que los archivos entran desde el Explorador o desde la línea de comandos. |
| Arrastrar y soltar archivos sobre la ventana | Misma decisión que la anterior. |
| Coloreado de sintaxis en los bloques de código | Exige una dependencia de resaltado y un trabajo de temas que no cabe en la primera versión. |
| Diagramas Mermaid y fórmulas LaTeX | Requieren motores propios de diagramas y de matemáticas. Es un salto de esfuerzo desproporcionado. |
| Imágenes alojadas en la web | Implicaría que la aplicación hiciera peticiones de red, con su caché y su decisión de privacidad. Se descartó. |
| HTML completo con hojas de estilo | Exigiría incrustar un motor de navegador o escribir uno. Rompería el objetivo de arranque en menos de un segundo. Se interpreta el subconjunto listado arriba y nada más. |
| Cambiar de tema en caliente | La aplicación toma el tema de Windows al arrancar. Reaccionar a un cambio de tema con la aplicación ya abierta queda fuera; con un arranque de menos de un segundo, el coste de reabrirla es despreciable. |
| Exportar a HTML o PDF, e imprimir | No forma parte de leer un documento. |
| Varias ventanas simultáneas | La decisión de instancia única implica una sola ventana. Reordenar pestañas arrastrándolas o moverlas entre ventanas queda fuera por lo mismo. |
| Recordar las pestañas abiertas entre ejecuciones | Añade estado persistente. Una aplicación que arranca en menos de un segundo no lo necesita para ser útil. |
| Temas personalizables por el usuario | Un visor tiene que verse bien por defecto, no ser configurable. Seguir el tema de Windows sí entra; ofrecer ajustes de color, no. |
| Linux y macOS | Son el objetivo declarado del proyecto, pero no de esta versión. macOS además no se puede verificar sin una máquina Apple, según se recoge en `00-contexto.md`. |

## Aplazado a versiones posteriores

Estas dos funciones se propusieron el 2026-08-20, después de aprobarse los
requisitos y antes de trocear el trabajo. **No se rechazaron: se decidió
construirlas más adelante.** Se recogen aquí, con la obligación que cada una
impone a la Fase 4, para que no se pierdan y para que nadie las confunda con las
funciones descartadas de la tabla anterior.

### Edición con confirmación explícita

Poder modificar el documento, pero solo después de que el usuario habilite la
edición de forma deliberada, como hace Word con los documentos protegidos. El
objetivo es evitar ediciones accidentales, no impedir editar.

**Motivo del aplazamiento:** un editor de texto exige cursor, selección, entrada
de teclado internacional, deshacer y rehacer, guardado, estado de modificado y
tratamiento del caso en que el archivo cambie en disco mientras se edita. Es
plausiblemente más trabajo que toda la primera versión junta. Terminar antes un
visor que funcione vale más que retrasarlo todo.

**Obligación que impone a la Fase 4:** el diseño debe dejar sitio para ello. En
concreto, la aplicación debe tratar el modo de solo lectura como un estado
explícito desde el primer día, y no como la simple ausencia de edición, para que
añadir el modo de edición más adelante no obligue a rehacer lo construido.

**Consecuencia sobre un requisito ya aprobado:** RF-07 de `02-requisitos.md` dice
que al cerrarse la última pestaña la aplicación termina. Con edición y cambios sin
guardar, eso sería pérdida de datos. RF-07 deberá revisarse cuando la edición
entre. No se cambia ahora, porque en la primera versión no hay nada que perder.

**Coste condicionado por la Fase 4:** si en Fase 4 se adopta `gpui-component`,
esta función hereda su componente de editor y su coste baja mucho. Si se opta por
un renderizador propio, hay que construir el editor entero. Es un factor que debe
pesar en esa decisión, aunque no la determine.

### Ver el Markdown en crudo

Alternar entre el documento renderizado y el texto tal como está escrito en el
archivo. En la misma conversación se ofrecieron además un árbol de encabezados y
un árbol de la estructura completa del documento; el usuario descartó ambos, y esa
exclusión está recogida en la tabla anterior.

**Motivo del aplazamiento:** es barato, porque el texto ya está en memoria, pero
no hace falta para leer un documento, que es lo único que promete la primera
versión. Además es la antesala natural del modo de edición, así que lo más
probable es que ambas funciones se construyan juntas.

**Obligación que impone a la Fase 4:** ninguna más allá de la anterior. Si el
diseño conserva el texto original del archivo junto a su versión renderizada, esta
función resulta casi gratuita.

### Distribución: instalador, PATH y versión portable

Un instalador que además registre MDView en el PATH del sistema, de modo que
`mdview archivo.md` funcione desde cualquier carpeta sin escribir la ruta
completa del ejecutable. Junto a él, una versión portable que no requiera
instalación.

Propuesto por el usuario el 2026-08-20, al preguntársele cómo debía registrarse
la asociación de `.md`. Su respuesta fue que prefiere un instalador con
integración en el PATH, pero que eso merece ser una feature propia, y que la
opción portable también le parece buena.

**Por qué es una feature y no un detalle de `integracion-con-windows`:** el
empaquetado es el punto donde el objetivo multiplataforma declarado en
`00-contexto.md` deja de ser una intención y se convierte en trabajo concreto y
distinto por sistema: instalador en Windows, paquete o imagen autocontenida en
Linux, aplicación empaquetada en macOS. Meterlo dentro de la feature de Windows
lo dejaría escondido justo donde nadie lo buscará al portar el proyecto.

**Alcance nuevo que introduce:** la integración en el PATH no la cubre ningún
requisito actual. RF-01 se cumple invocando el ejecutable con su ruta completa;
poder teclear solo `mdview` es comodidad añadida y necesitará su propio
requisito cuando esta feature se abra.

**Consecuencia sobre la asociación de `.md`:** RF-02 sigue perteneciendo a
`integracion-con-windows`, pero el mecanismo con el que se registre dependerá de
si existe instalador o no. Ver la pregunta abierta.

**A verificar antes de construir RF-02:** desde Windows 8, la clave del registro
que fija la aplicación predeterminada de una extensión está protegida con un
hash que el sistema valida, para impedir que los programas se apropien de las
extensiones al instalarse. Si eso es así, ningún mecanismo —instalador incluido—
puede dejar MDView como predeterminada sin un clic del usuario en el diálogo de
Windows; solo puede registrarla como opción disponible. **Está sin comprobar
contra la documentación de Microsoft**, y hay que comprobarlo antes de escribir
los criterios de aceptación de RF-02, porque cambia lo que se puede prometer.

## Supuestos

Se dan por ciertos sin haberlos confirmado. Cada uno es un riesgo: si alguno es
falso, cambia trabajo ya planificado.

1. **Archivo ya abierto.** Si se pide abrir un `.md` que ya tiene pestaña, se
   enfoca esa pestaña en lugar de crear una segunda con el mismo documento.
2. **La aplicación no hace peticiones de red.** Abrir un enlace externo consiste
   en pedirle al sistema operativo que lance el navegador predeterminado; MDView
   no descarga nada por su cuenta. Se considera una propiedad deseable y
   verificable, no un efecto colateral.
3. **Sin límites declarados.** No hay objetivo de tamaño máximo de archivo ni de
   número de pestañas abiertas a la vez. Se medirá lo que ocurra y se registrará
   como línea base al verificar la primera historia.
4. **Codificación.** Los archivos que se abren son texto UTF-8. Qué ocurre con
   otras codificaciones no está decidido.
5. **Título de pestaña.** La pestaña muestra el nombre del archivo, no el primer
   encabezado del documento.

## Preguntas cerradas

Se conservan con su respuesta para que nadie las reabra por desconocimiento.
Las seis primeras estaban abiertas al terminar la Fase 1 y se respondieron al
empezar la Fase 2; las cuatro últimas surgieron al escribir las historias, en
Fase 3, y se respondieron al terminarla. Todo el 2026-08-20.

| Pregunta | Respuesta |
| --- | --- |
| ¿Qué ocurre al pulsar un enlace externo? | Se abre en el navegador predeterminado del sistema. Esto anuló el supuesto anterior, que los daba por inertes. |
| ¿Qué se hace con el front matter? | Se presenta como tabla de propiedades. |
| ¿Qué se hace con el HTML incrustado? | Se interpreta el subconjunto cerrado de etiquetas listado en el alcance. El usuario preguntó por interpretarlo como lo haría un navegador; se descartó por incompatible con el objetivo de arranque. |
| ¿Qué ve el usuario ante un archivo ilegible? | No se abre pestaña y se muestra un aviso temporal con el archivo y la causa. |
| ¿Qué pasa al cerrar la última pestaña? | La aplicación termina. |
| ¿Cómo se decide el tema claro u oscuro? | Se adopta el que tenga configurado Windows. |
| ¿Renderizador propio o el de `gpui-component`? | Ambos, repartidos: se adopta `gpui-component` para los componentes de interfaz (AD-01) y se analiza el Markdown con `pulldown-cmark` propio (AD-02), porque RF-12 y RF-13 exigen controlar la traducción de cada elemento. |
| ¿Cómo se representa el estado de solo lectura? | Como un campo de modo explícito con un único valor posible hoy (AD-05), para que añadir la edición más adelante no obligue a introducir el concepto por todo el código a la vez. |
| ¿Se soporta Windows 10 además de Windows 11? | No. Microsoft dejó de darle soporte en octubre de 2025 y verificarlo exigiría una máquina o una virtual que no existe. RNF-03 se mantiene en Windows 11 de 64 bits. |
| ¿Qué hace la aplicación si se la invoca sin ninguna ruta? | Muestra una ventana que indica qué es MDView y cómo pedirle que abra un archivo. Es ahora RF-19. |
| ¿Qué pasa con una línea de código más ancha que la ventana? | Se conserva entera y el bloque se desplaza horizontalmente por sí solo. Se añadió como cláusula de RF-10. |
| ¿El texto se reajusta al cambiar el tamaño de la ventana? | Sí, hasta un ancho máximo de lectura, a partir del cual la columna queda centrada. Es ahora RF-20. |

## Preguntas abiertas

| Pregunta | Quién la responde |
| --- | --- |
| ¿La asociación de `.md` se hace con un instalador, con auto-registro al arrancar o con un registro explícito bajo demanda? | Sigue abierta, pero con dirección: el usuario declaró el 2026-08-20 que prefiere un instalador, y que la versión portable también le convence. Se resolverá junto con la feature de distribución, porque el mecanismo depende de si existe instalador. Decidirlo antes sería decidir a ciegas. |
