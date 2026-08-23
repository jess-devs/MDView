# Historias de usuario — ver-un-documento

> Estado: borrador
> Última actualización: 2026-08-20
> Modo: new-project

Siete historias que cubren los once requisitos de `requisitos.md`. Cada criterio
lleva un identificador `CA-<historia>.<n>` para que el registro de verificación
de `04-calidad.md` pueda referirse a él sin citar su texto.

Los criterios están escritos para que una persona mire la aplicación en marcha y
responda sí o no. Si dos personas pudieran discrepar sobre si un criterio se
cumple, el criterio está mal escrito y hay que reescribirlo, no aprobarlo.

El único rol de usuario es el lector de documentación técnica, definido en
`00-contexto.md`.

---

## HU-01 — Ver un documento pasándole su ruta

Cubre: RF-01.1, RF-16.1, RF-20.1, RNF-03.1

**Historia.** Como lector de documentación técnica, quiero indicarle a MDView la
ruta de un archivo Markdown y verlo con formato, para no tener que abrir un
editor de código solo para leerlo.

**Criterios de aceptación.**

1. CA-01.1 — Al invocar la aplicación pasándole la ruta de un archivo `.md`
   existente y legible, aparece una ventana que muestra el contenido de ese
   archivo con formato, y no su texto en crudo.
2. CA-01.2 — Con un documento más largo que la ventana, es posible desplazarse
   con la rueda del ratón hasta ver la última línea del archivo, y volver hasta
   ver la primera.
3. CA-01.3 — La aplicación arranca y muestra el documento en una máquina con
   Windows 11 de 64 bits.
4. CA-01.4 — Al estrechar la ventana, los párrafos se reajustan al nuevo ancho:
   ninguna línea de texto queda cortada ni obliga a desplazarse horizontalmente
   para terminar de leerla.
5. CA-01.5 — Al ensanchar la ventana más allá del ancho máximo de lectura, la
   columna de texto deja de crecer y queda centrada, con margen a ambos lados.

**Estado.** verificada. Los cinco criterios pasan por observación; el detalle
está en `docs/04-calidad.md`.

---

## HU-02 — Leer un documento con el formato que el autor escribió

Cubre: RF-08.1, RF-09.1

**Historia.** Como lector de documentación técnica, quiero que los encabezados,
las listas, las tablas y el resto de elementos se vean con su formato, para poder
leer el documento en lugar de descifrar su marcado.

**Criterios de aceptación.** Se comprueban sobre un único documento de prueba que
contenga todos los elementos citados.

1. CA-02.1 — Los encabezados de nivel 1 a 6 se muestran los seis con tamaños de
   letra distintos entre sí, decrecientes del nivel 1 al 6.
2. CA-02.2 — El texto con énfasis se muestra en cursiva y el texto con negrita se
   muestra en negrita, ambos distinguibles del texto normal que los rodea.
3. CA-02.3 — Una lista no ordenada de dos niveles muestra los elementos del
   segundo nivel sangrados respecto a los del primero, y cada elemento con su
   viñeta.
4. CA-02.4 — Una lista ordenada muestra los números que le corresponden, en orden
   correlativo.
5. CA-02.5 — Una cita se muestra con un margen izquierdo mayor que el del texto
   normal y con una marca vertical a su izquierda.
6. CA-02.6 — Una regla horizontal se muestra como una línea que recorre el ancho
   del documento.
7. CA-02.7 — El código en línea se muestra en tipografía monoespaciada, distinta
   de la del párrafo que lo contiene.
8. CA-02.8 — Una tabla de tres columnas y tres filas se muestra como una rejilla,
   con la fila de encabezado distinguible de las demás y las celdas de cada
   columna alineadas entre sí.
9. CA-02.9 — Una lista de tareas muestra una casilla vacía en los elementos sin
   marcar y una casilla marcada en los marcados.
10. CA-02.10 — El texto tachado se muestra con una línea que lo atraviesa.

**Estado.** pendiente

---

## HU-03 — Leer bloques de código sin que estorben

Cubre: RF-10.1

**Historia.** Como lector de documentación técnica, quiero que los bloques de
código se distingan del texto y conserven su forma, para poder copiarlos
mentalmente o leerlos sin perder la sangría.

**Criterios de aceptación.**

1. CA-03.1 — Un bloque de código delimitado por vallas se muestra en tipografía
   monoespaciada.
2. CA-03.2 — El bloque se muestra sobre un fondo de color distinto al del fondo
   del documento.
3. CA-03.3 — Todo el texto del bloque se muestra del mismo color: ninguna palabra
   aparece coloreada según el lenguaje declarado en la valla.
4. CA-03.4 — Los saltos de línea y los espacios de sangría del bloque se ven tal
   como están escritos en el archivo.
5. CA-03.5 — Con una línea de código más ancha que la ventana, esa línea no
   aparece partida en dos, y el bloque que la contiene se puede desplazar
   horizontalmente hasta ver su final.
6. CA-03.6 — Al desplazar horizontalmente ese bloque, el resto del documento no
   se mueve: los párrafos anteriores y posteriores siguen en su sitio.

**Estado.** pendiente

---

## HU-04 — Ver el documento con el tema del sistema

Cubre: RF-18.1

**Historia.** Como lector de documentación técnica, quiero que MDView respete el
tema claro u oscuro de mi Windows, para que no me deslumbre al abrirse de noche.

**Criterios de aceptación.**

1. CA-04.1 — Con Windows configurado en modo claro, al abrir un documento el
   fondo de la ventana es claro y el texto del documento es oscuro.
2. CA-04.2 — Con Windows configurado en modo oscuro, al abrir un documento el
   fondo de la ventana es oscuro y el texto del documento es claro.
3. CA-04.3 — En ambos temas, todos los elementos comprobados en HU-02 y HU-03
   siguen siendo legibles: ningún texto se muestra del mismo color que el fondo
   sobre el que está.

**Estado.** pendiente

---

## HU-05 — Enterarme de que el archivo no se puede mostrar

Cubre: RF-17.1

**Historia.** Como lector de documentación técnica, quiero que MDView me diga qué
ha pasado cuando no puede abrir un archivo, para no quedarme mirando una ventana
vacía preguntándome si la aplicación está rota.

**Criterios de aceptación.**

1. CA-05.1 — Al indicar la ruta de un archivo que no existe, no se muestra ningún
   documento y aparece un aviso que nombra ese archivo e indica que no se
   encontró.
2. CA-05.2 — Al indicar la ruta de un archivo existente cuyo contenido no es
   texto, por ejemplo una imagen con la extensión cambiada a `.md`, no se muestra
   ningún documento y aparece un aviso que nombra ese archivo e indica que no es
   un archivo de texto.
3. CA-05.3 — Al indicar la ruta de un archivo existente sobre el que el usuario
   no tiene permiso de lectura, no se muestra ningún documento y aparece un aviso
   que nombra ese archivo e indica que no se pudo leer por permisos.
4. CA-05.4 — El aviso desaparece por sí solo, sin que el usuario tenga que
   cerrarlo.
5. CA-05.5 — En los tres casos anteriores la aplicación no termina de forma
   abrupta ni deja de responder.

**Estado.** pendiente

---

## HU-06 — Ver el documento sin esperar

Cubre: RNF-01.1

**Historia.** Como lector de documentación técnica, quiero que el documento
aparezca de inmediato, porque si tengo que esperar tanto como con un editor de
código, MDView no me aporta nada.

**Criterios de aceptación.**

1. CA-06.1 — En tres arranques en frío consecutivos, con un archivo de
   aproximadamente 50 KB, el tiempo transcurrido desde la invocación hasta que el
   documento está visible es menor que 1 segundo **en los tres**.

El método de medición está descrito en `plan.md`. No es un criterio: es la forma
de observar este, y sin él CA-06.1 no se puede comprobar.

**Estado.** pendiente

---

## HU-07 — Entender qué es esto si lo abro sin un archivo

Cubre: RF-19.1

**Historia.** Como lector de documentación técnica, quiero que MDView me diga qué
es y cómo se usa si lo abro por curiosidad sin ningún archivo, para no quedarme
delante de algo que parece averiado.

**Criterios de aceptación.**

1. CA-07.1 — Al invocar la aplicación sin pasarle ninguna ruta, aparece una
   ventana. No ocurre que no pase nada visible.
2. CA-07.2 — Esa ventana contiene un texto que indica que MDView sirve para ver
   archivos `.md`.
3. CA-07.3 — Esa ventana indica, además, al menos una forma concreta de pedirle
   que abra un archivo.

**Estado.** pendiente

---

## Cobertura

| Requisito | Historia que lo cubre |
| --- | --- |
| RF-01.1 | HU-01 |
| RF-08.1 | HU-02 |
| RF-09.1 | HU-02 |
| RF-10.1 | HU-03 |
| RF-16.1 | HU-01 |
| RF-17.1 | HU-05 |
| RF-18.1 | HU-04 |
| RF-19.1 | HU-07 |
| RF-20.1 | HU-01 |
| RNF-01.1 | HU-06 |
| RNF-03.1 | HU-01 |

Los once requisitos de la feature están cubiertos. Ninguna historia carece de
requisito y ningún requisito carece de historia.

Siete historias es el techo de lo que la lista de validación considera una
feature manejable. Si en Fase 4 apareciera una octava, la señal a atender no
sería «cabe una más», sino que esta feature ha crecido y hay que partirla.
