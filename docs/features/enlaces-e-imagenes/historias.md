# Historias — enlaces-e-imagenes

> Estado: borrador
> Última actualización: 2026-08-24
> Modo: new-feature

Cinco historias, un único tipo de usuario: el lector de documentación técnica de
`00-contexto.md`. Los criterios se comprueban por observación sobre la aplicación
en marcha; el método de los dos que no se observan mirando la ventana —HU-04 y
HU-05— está en `plan.md`.

---

## HU-01 — Leer un documento con enlaces sin perder nada

Cubre: RF-08.2

**Historia.** Como lector de documentación técnica, quiero que un enlace se vea
como un enlace dentro de su párrafo, porque hoy un README con enlaces se muestra
recortado y no me entero de que falta texto.

**Criterios de aceptación.** Se comprueban sobre un documento de prueba con un
párrafo de la forma «texto antes, enlace, texto después» y un encabezado
posterior.

1. CA-01.1 — El párrafo que contiene un enlace muestra las tres partes —el texto
   anterior, el texto del enlace y el texto posterior— en ese orden y dentro de
   un mismo párrafo.
2. CA-01.2 — El texto del enlace se muestra distinguible del texto que lo rodea,
   por color, por subrayado o por ambos.
3. CA-01.3 — La URL del enlace no aparece escrita en el cuerpo del documento.
4. CA-01.4 — El encabezado que va después del párrafo con el enlace, y todo lo
   que le sigue, se siguen mostrando.

**Estado.** verificada. Los cuatro criterios pasan por observación; el detalle
está en `docs/04-calidad.md`.

---

## HU-02 — Abrir un enlace externo en el navegador

Cubre: RF-15.1

**Historia.** Como lector de documentación técnica, quiero pulsar un enlace del
documento y que se abra en mi navegador, para no tener que copiar la dirección a
mano.

**Criterios de aceptación.**

1. CA-02.1 — Al pulsar sobre un enlace `https`, el navegador predeterminado del
   sistema muestra esa dirección.
2. CA-02.2 — Un enlace `http` se comporta igual que uno `https`.
3. CA-02.3 — La dirección no se abre dentro de la ventana de MDView: la ventana
   sigue mostrando el mismo documento en la misma posición.
4. CA-02.4 — Tras abrir el enlace, MDView sigue respondiendo: el documento se
   puede desplazar y la ventana se puede cerrar con normalidad.
5. CA-02.5 — Un enlace `https` escrito dentro de la celda de una tabla se abre
   igual que uno escrito en un párrafo.

CA-02.5 se añadió el 2026-08-24, después de implementar los cuatro anteriores,
al observarse que los enlaces de las celdas de una tabla se mostraban
distinguibles pero no reaccionaban al clic. No amplía la historia: RF-15.1 dice
«al activarse un enlace `http` o `https`» sin distinguir dónde está escrito, así
que la lista de criterios estaba incompleta, no el requisito.

**Estado.** en curso

---

## HU-03 — Ver las imágenes que el documento referencia

Cubre: RF-11.1

**Historia.** Como lector de documentación técnica, quiero ver las capturas y
diagramas que el documento incluye, porque un README cuyas imágenes no aparecen
está contando la mitad.

**Criterios de aceptación.** Se comprueban sobre un documento de prueba guardado
en una carpeta que contenga, a su lado, un subdirectorio con las imágenes.

1. CA-03.1 — Una imagen referenciada con ruta relativa se muestra en el punto del
   documento donde estaba escrita.
2. CA-03.2 — Invocando MDView desde un directorio distinto del que contiene el
   `.md`, esa misma imagen se sigue mostrando.
3. CA-03.3 — Una imagen cuya ruta no corresponde a ningún archivo muestra en su
   lugar su texto alternativo, y el resto del documento se sigue mostrando.
4. CA-03.4 — Una imagen que apunta a un archivo existente que no es una imagen
   decodificable muestra igualmente su texto alternativo, sin que la aplicación
   termine de forma abrupta ni deje de responder.
5. CA-03.5 — Una imagen más ancha que la columna de lectura se muestra ajustada a
   ese ancho, sin desbordarla ni obligar a desplazarse horizontalmente.

**Estado.** pendiente

---

## HU-04 — Saber que MDView no se conecta a nada

Cubre: RNF-02.1

**Historia.** Como lector de documentación técnica, quiero saber que abrir un
documento no manda nada a ninguna parte, porque leo documentos que no son míos y
no quiero que su contenido salga del equipo.

**Criterios de aceptación.** El método de observación está en `plan.md`; sin él
estos criterios no se pueden comprobar.

1. CA-04.1 — Con un documento abierto y visible que contiene enlaces `http` y una
   imagen referenciada por URL `http`, no se observa ninguna conexión TCP ni
   ningún extremo UDP atribuible al proceso de MDView, desde su arranque hasta
   que el documento está en pantalla.
2. CA-04.2 — Esa imagen referenciada por URL muestra su texto alternativo, no un
   hueco vacío ni un error.
3. CA-04.3 — Al abrir un enlace externo según HU-02 sí aparece tráfico, pero
   atribuido al proceso del navegador y no al de MDView.

**Estado.** pendiente

---

## HU-05 — Seguir viendo el documento sin esperar

Cubre: RNF-01.2

**Historia.** Como lector de documentación técnica, quiero que añadir imágenes al
documento no devuelva a MDView al tiempo de arranque de un editor de código.

**Criterios de aceptación.**

1. CA-05.1 — En tres arranques en frío consecutivos, con el documento de prueba
   con imágenes descrito en `plan.md`, el tiempo transcurrido desde la invocación
   hasta que el documento está visible es menor que 1,2 segundos **en los tres**.

El umbral es el de RNF-01 tras AD-14, no uno propio de esta feature. El método de
medición es el mismo que el de CA-06.1 en `ver-un-documento/plan.md`.

**Estado.** pendiente

---

## Cobertura

| Requisito | Historia que lo cubre |
| --- | --- |
| RF-08.2 | HU-01 |
| RF-11.1 | HU-03 |
| RF-15.1 | HU-02 |
| RNF-01.2 | HU-05 |
| RNF-02.1 | HU-04 |
| RNF-03.1 | HU-01 |

Los seis requisitos de la feature están cubiertos. Ninguna historia carece de
requisito y ningún requisito carece de historia. RNF-03.1 se asigna a HU-01
igual que en `ver-un-documento`: es la primera historia que se observa, y toda
observación posterior ocurre en la misma plataforma.
