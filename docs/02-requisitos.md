# Requisitos

> Estado: borrador
> Última actualización: 2026-08-20
> Modo: new-project

Requisitos de todo el sistema en su primera versión. Los términos usados están
definidos en el glosario de `00-contexto.md`; el reparto entre lo que entra y lo
que no, en `01-alcance.md`.

Cada requisito lleva un identificador estable, `RF-nn` para los funcionales y
`RNF-nn` para los no funcionales, para que las historias y el registro de
verificación puedan referirse a él sin citarlo. Cada uno lleva además su origen,
distinguiendo lo que el usuario afirmó de lo que eligió de una lista propuesta:
no son lo mismo, y quien lea esto dentro de seis meses no debe confundir un
valor por defecto con una restricción real.

Ningún requisito de aquí dice cómo se implementa. Las decisiones técnicas
—biblioteca de renderizado, mecanismo de comunicación entre procesos, forma de
registrar la asociación de archivos— corresponden a la Fase 4 y se recogerán en
`03-arquitectura.md`, que todavía no existe.

## Funcionales

### RF-01 — Apertura desde la línea de comandos

El sistema debe abrir los archivos cuyas rutas se le pasen como argumentos al
invocarlo, admitiendo varias rutas en una sola invocación.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 1).

### RF-02 — Apertura desde el Explorador de Windows

El sistema debe poder quedar configurado como la aplicación que Windows usa para
los archivos `.md`, de modo que un doble clic sobre uno de ellos en el Explorador
muestre su documento en MDView.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 1).

### RF-03 — Instancia única con comportamiento de navegador

Cuando ya existe un proceso de MDView en ejecución, una nueva petición de
apertura debe ser atendida por ese proceso y no por uno nuevo. Su ventana pasa a
primer plano, el documento solicitado se añade en una pestaña que queda como
pestaña activa, y los documentos que ya estaban abiertos permanecen en las suyas.
Origen: descrito por el usuario con sus propias palabras (Fase 1), comparándolo
con el comportamiento de un navegador.

### RF-04 — Documento ya abierto

Si se pide abrir un archivo que ya tiene una pestaña, esa pestaña pasa a ser la
pestaña activa y no se crea una segunda para el mismo documento.
Origen: supuesto 1 de `01-alcance.md`. No confirmado por el usuario. [inferido]

### RF-05 — Gestión de pestañas

El sistema debe permitir cambiar de pestaña y cerrar una pestaña concreta, con
independencia de cuántas haya abiertas.
Origen: pedido por el usuario en el planteamiento inicial del proyecto.

### RF-06 — Identificación de la pestaña

Cada pestaña debe mostrar el nombre del archivo que contiene.
Origen: supuesto 5 de `01-alcance.md`. No confirmado por el usuario. [inferido]

### RF-07 — Cierre de la última pestaña

Al cerrarse la última pestaña abierta, la aplicación debe terminar.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 2).

### RF-08 — Renderizado de CommonMark

El sistema debe mostrar con formato los elementos de CommonMark: encabezados de
nivel 1 a 6, párrafos, énfasis, negrita, listas ordenadas y no ordenadas con
anidamiento, citas, reglas horizontales, código en línea y bloques de código.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 1).

### RF-09 — Renderizado de los añadidos de GFM

El sistema debe mostrar con formato las tablas, las listas de tareas y el texto
tachado.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 1).

### RF-10 — Bloques de código sin coloreado

Los bloques de código deben mostrarse en tipografía monoespaciada y sobre un
fondo distinguible del resto del documento, sin colorear las palabras según el
lenguaje. Una línea más ancha que la ventana debe conservarse entera: el bloque
que la contiene se desplaza horizontalmente por sí solo, sin arrastrar al resto
del documento y sin partir la línea.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 1); el
coloreado quedó explícitamente fuera de alcance. La cláusula sobre las líneas
anchas se añadió en Fase 3, al detectarse que ningún requisito decía qué hacer
con ellas, y el usuario eligió entre las alternativas ofrecidas.

### RF-11 — Imágenes locales

El sistema debe mostrar las imágenes que el documento referencie mediante una
ruta relativa al archivo `.md` que se está viendo.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 1).

### RF-12 — Front matter como tabla de propiedades

Si el documento empieza con un front matter, el sistema debe presentarlo como una
tabla de propiedades en la parte alta del documento, y no como texto en crudo ni
como reglas horizontales.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 2).

### RF-13 — Subconjunto de HTML incrustado

El sistema debe interpretar las siguientes etiquetas HTML cuando aparezcan
dentro del Markdown, mostrándolas con el formato equivalente al del elemento
Markdown correspondiente: `img`, `a`, `b`, `strong`, `i`, `em`, `code`, `br`,
`hr`, `p`, `ul`, `li`, `table`, `details`, `summary`, y `div` con atributo de
alineación centrada. Los atributos de estilo y las hojas de estilo se ignoran.
De una etiqueta que no esté en esta lista no se muestra su marcado, pero sí el
texto que contenga.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 2), tras
descartarse la interpretación completa de HTML por incompatible con RNF-01. El
trato de las etiquetas no listadas no se preguntó y se resuelve aquí por
coherencia con las alternativas que sí se ofrecieron. [inferido]

### RF-14 — Navegación entre documentos

Al activarse un enlace que apunte, mediante ruta relativa, a otro archivo `.md`,
el sistema debe abrir ese archivo en una pestaña nueva.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 1).

### RF-15 — Enlaces externos

Al activarse un enlace `http` o `https`, el sistema debe hacer que se abra en el
navegador predeterminado del sistema operativo.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 2).

### RF-16 — Desplazamiento del documento

El sistema debe permitir desplazarse verticalmente por un documento más largo
que la ventana.
Origen: consecuencia directa de mostrar documentos completos (RF-08).

### RF-17 — Archivo que no se puede mostrar

Si un archivo solicitado no existe, no se puede leer o no es texto, el sistema no
debe abrir una pestaña para él, y debe mostrar un aviso temporal que identifique
el archivo y la causa.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 2).

### RF-18 — Tema claro u oscuro

El sistema debe adoptar, al arrancar, el tema claro u oscuro que tenga
configurado Windows.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 2).

### RF-19 — Arranque sin ningún archivo

Si se invoca la aplicación sin pasarle ninguna ruta, el sistema debe mostrar una
ventana que indique que MDView abre archivos `.md` y por qué medios se le puede
pedir que abra uno.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 3), al
detectarse que ningún requisito decía qué ocurre en ese caso.

### RF-20 — Ancho del texto al redimensionar la ventana

El texto del documento debe reajustarse al ancho de la ventana cuando esta
cambia de tamaño, hasta un ancho máximo de lectura. Superado ese límite, la
columna de texto deja de crecer y se mantiene centrada en la ventana. En ningún
caso el texto queda cortado ni obliga a desplazarse horizontalmente para leer un
párrafo.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 3). El
valor concreto del ancho máximo no está fijado aquí: es una decisión de diseño
que se registrará en `decisiones.md` al implementarlo.

## No funcionales

### RNF-01 — Tiempo hasta el documento visible

Desde que el usuario hace doble clic sobre un archivo hasta que su documento
está visible en pantalla deben transcurrir **menos de 1,2 segundos**, medido en
arranque en frío y con un archivo de aproximadamente 50 KB.
Origen: elegido por el usuario entre las alternativas ofrecidas (Fase 1). La
cifra la propuse yo dentro de una lista; el usuario la eligió, pero no es un
dato que él aportara. La necesidad de que la aplicación sea rápida sí es suya:
es el motivo por el que existe el proyecto.
El umbral fue de 1 segundo hasta el 2026-08-24, cuando la medición de CA-06.1
lo dejó fallando por 55 ms y el usuario eligió renegociarlo en vez de seguir
optimizando: ver AD-14 en `features/ver-un-documento/decisiones.md`.

### RNF-02 — Ausencia de tráfico de red propio

El proceso de MDView no debe iniciar ninguna conexión de red por su cuenta
mientras abre y muestra documentos. El umbral es cero conexiones salientes
atribuibles al proceso. Delegar en el sistema operativo la apertura de un enlace
externo, según RF-15, no cuenta como conexión propia, porque la realiza el
navegador.
Origen: consecuencia de excluir las imágenes remotas (Fase 1) y del supuesto 2
de `01-alcance.md`.

### RNF-03 — Plataforma de ejecución

El sistema debe ejecutarse en Windows 11 de 64 bits, que es donde se desarrolla
y donde se verificará.
Origen: el usuario eligió limitar la primera versión a Windows (Fase 1), y su
entorno de trabajo es Windows 11. En Fase 3 se le preguntó expresamente si debía
soportarse también Windows 10 y respondió que no, entre otras razones porque
Microsoft dejó de darle soporte en octubre de 2025. Windows 10 queda fuera.

## Requisitos que deliberadamente no existen

Esta sección está aquí para que nadie los añada más adelante creyendo que se
olvidaron.

- **No hay objetivo de consumo de memoria, de tamaño del ejecutable ni de
  disponibilidad.** El usuario no ha declarado ninguna cifra y no se inventa
  ninguna. Se medirán y se registrarán como línea base, sin ser criterio de
  aceptación.
- **No hay requisitos de seguridad, privacidad ni protección de datos más allá
  de RNF-02.** La aplicación lee archivos que ya están en el disco del usuario,
  con sus permisos, y no los envía a ninguna parte. No se ha declarado ninguna
  regulación aplicable.
- **No hay requisitos para la edición del documento ni para ver el Markdown en
  crudo.** Ambas funciones se decidieron y se aplazaron a una versión posterior
  el 2026-08-20; están descritas en la sección de funciones aplazadas de
  `01-alcance.md`, con la obligación que imponen a la Fase 4. No tienen requisito
  aquí porque no forman parte de esta versión, pero no están descartadas.
- **No hay requisitos de accesibilidad declarados.** No se ha hablado de lectores
  de pantalla, navegación por teclado ni tamaño mínimo de texto. Si importan,
  hay que decidirlo y añadirlos aquí, no darlos por supuestos.
