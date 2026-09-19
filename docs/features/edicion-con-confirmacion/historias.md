# Historias — edicion-con-confirmacion

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Cuatro historias, un único tipo de usuario: el lector de documentación
técnica de `00-contexto.md`, ahora también corrigiendo una errata sin
salir de MDView.

---

## HU-01 — Habilitar la edición y escribir en el documento

Cubre: RF-24.1, RF-24.2

**Historia.** Como lector de documentación técnica, quiero poder pasar un
documento a modo edición a propósito y escribir en él con normalidad
—cursor, selección, deshacer, rehacer— para corregir una errata sin
abrir otro programa.

**Criterios de aceptación.**

1. CA-01.1 — Un documento abre de solo lectura; el control para pasar a
   edición está visible pero la edición no está activa.
2. CA-01.2 — Pulsar el control muestra el texto crudo del documento,
   editable.
3. CA-01.3 — Escribir texto lo inserta en la posición del cursor.
4. CA-01.4 — Seleccionar texto y reemplazarlo (escribir sobre la
   selección) funciona.
5. CA-01.5 — Deshacer revierte el último cambio; rehacer lo repone.

**Estado.** verificada. Los cinco criterios pasan por observación; el
detalle está en `docs/04-calidad.md`.

---

## HU-02 — Guardar los cambios

Cubre: RF-24.3

**Historia.** Como lector de documentación técnica, quiero guardar mis
cambios con una acción explícita y ver el documento renderizado con lo
que acabo de escribir.

**Criterios de aceptación.** Se comprueban tras editar un documento
(HU-01) y guardar.

1. CA-02.1 — El botón de guardar escribe el texto editado en el archivo
   original: releer el archivo desde fuera de MDView muestra el cambio.
2. CA-02.2 — Tras guardar, la pestaña vuelve a mostrarse renderizada,
   con el cambio reflejado en el renderizado.
3. CA-02.3 — El atajo de teclado habitual del sistema para guardar
   produce el mismo resultado que el botón.

**Estado.** verificada. Los tres criterios pasan por observación; el
detalle está en `docs/04-calidad.md`.

---

## HU-03 — Indicador de cambios sin guardar

Cubre: RF-24.4

**Historia.** Como lector de documentación técnica, quiero ver de un
vistazo si una pestaña tiene cambios sin guardar, para no perderlos por
descuido.

**Criterios de aceptación.**

1. CA-03.1 — Editar el texto en modo edición marca la pestaña como con
   cambios sin guardar.
2. CA-03.2 — Guardar (HU-02) quita esa marca.
3. CA-03.3 — Deshacer todos los cambios hasta volver al texto original
   quita la marca también, sin necesidad de guardar.

**Estado.** verificada. Los tres criterios pasan por observación; el
detalle está en `docs/04-calidad.md`.

---

## HU-04 — Confirmar antes de perder cambios sin guardar

Cubre: RF-24.5

**Historia.** Como lector de documentación técnica, quiero que MDView me
pregunte antes de cerrar una pestaña con cambios sin guardar, incluida
la última, para no perder una corrección por un clic de más.

**Criterios de aceptación.** Se comprueban con una pestaña marcada como
con cambios sin guardar (HU-03).

1. CA-04.1 — Cerrar esa pestaña muestra un diálogo con tres opciones:
   guardar, descartar, cancelar.
2. CA-04.2 — «Guardar» guarda el archivo (mismo resultado que HU-02) y
   cierra la pestaña.
3. CA-04.3 — «Descartar» cierra la pestaña sin escribir el archivo.
4. CA-04.4 — «Cancelar» no cierra la pestaña ni cambia nada.
5. CA-04.5 — Si la pestaña con cambios sin guardar es la única abierta,
   cerrarla dispara el mismo diálogo antes de terminar la aplicación
   (RF-07 revisado): no se pierde el documento sin preguntar.
6. CA-04.6 — Cerrar una pestaña **sin** cambios sin guardar no muestra
   ningún diálogo: el comportamiento de RF-05/RF-07 para ese caso no
   cambia.

**Estado.** verificada. Los seis criterios pasan por observación; el
detalle está en `docs/04-calidad.md`.

---

## Cobertura

| Requisito | Historia que lo cubre |
| --- | --- |
| RF-24.1 | HU-01 |
| RF-24.2 | HU-01 |
| RF-24.3 | HU-02 |
| RF-24.4 | HU-03 |
| RF-24.5 | HU-04 |

Los cinco requisitos de esta feature quedan cubiertos entre sus cuatro
historias.
