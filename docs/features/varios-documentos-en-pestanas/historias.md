# Historias — varios-documentos-en-pestanas

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Cuatro historias, un único tipo de usuario: el lector de documentación técnica
de `00-contexto.md`. Se agrupan por acción observable, no una por requisito:
RF-04.1 (no duplicar una pestaña) no tiene una historia propia porque no es
una acción que el lector inicie — es una consecuencia de abrir varios
documentos (HU-01) o de navegar por un enlace (HU-04), y se comprueba dentro
de esas dos.

---

## HU-01 — Abrir varios documentos a la vez

Cubre: RF-01.1, RF-06.1, RF-04.1 (caso de arranque)

**Historia.** Como lector de documentación técnica, quiero abrir varios
documentos pasándole todas sus rutas de una vez, para tenerlos todos a mano
sin invocar MDView varias veces.

**Criterios de aceptación.** Se comprueban invocando MDView con las rutas de
tres documentos de prueba distintos.

1. CA-01.1 — Invocado con tres rutas, aparecen tres pestañas.
2. CA-01.2 — La pestaña de la primera ruta es la activa: su documento es el
   que se ve al arrancar.
3. CA-01.3 — Cada pestaña muestra el nombre del archivo que contiene —con su
   extensión, sin la ruta completa ni el primer encabezado del documento.
4. CA-01.4 — Invocado con una ruta repetida dos veces en la misma lista,
   aparece una sola pestaña para ella, no dos.
5. CA-01.5 — Invocado con tres rutas de las que una no existe, aparecen las
   dos pestañas de las rutas válidas, y un aviso temporal (RF-17) nombra la
   que falló, sin pestaña propia para ella.

**Estado.** pendiente

---

## HU-02 — Cambiar de pestaña

Cubre: RF-05.1 (parcial: cambiar)

**Historia.** Como lector de documentación técnica, quiero pulsar una pestaña
para ver su documento, para moverme entre los que tengo abiertos.

**Criterios de aceptación.** Se comprueban con los tres documentos de HU-01
ya abiertos.

1. CA-02.1 — Pulsar una pestaña que no es la activa la convierte en la
   activa: su documento pasa a verse.
2. CA-02.2 — La pestaña activa se distingue visualmente de las demás.
3. CA-02.3 — Volver a una pestaña ya visitada muestra su documento completo,
   sin diferencias respecto a la primera vez que se vio.

**Estado.** pendiente

---

## HU-03 — Cerrar una pestaña

Cubre: RF-05.1 (parcial: cerrar), RF-07.1

**Historia.** Como lector de documentación técnica, quiero cerrar la pestaña
de un documento que ya no necesito, para no acumular pestañas sin usar.

**Criterios de aceptación.**

1. CA-03.1 — Cerrar una pestaña que no es la activa la quita de la barra; las
   demás pestañas, incluida la activa, no cambian.
2. CA-03.2 — Cerrar la pestaña activa, habiendo más de una abierta, dejan
   otra pestaña como activa y su documento visible.
3. CA-03.3 — Cerrar la única pestaña abierta termina la aplicación.

**Estado.** pendiente

---

## HU-04 — Navegar a otro documento desde un enlace

Cubre: RF-14.1, RF-04.1 (caso de enlace)

**Historia.** Como lector de documentación técnica, quiero pulsar un enlace a
otro archivo `.md` del mismo proyecto y que se abra, para moverme entre
documentos relacionados sin ir a buscarlos a mano.

**Criterios de aceptación.** Se comprueban sobre un documento de prueba con un
enlace relativo a otro `.md` existente, uno a un `.md` que no existe, y uno
`https`.

1. CA-04.1 — Activar un enlace relativo a otro `.md` existente abre una
   pestaña nueva con ese documento y la activa.
2. CA-04.2 — Activar un enlace a un `.md` ya abierto en otra pestaña activa
   esa pestaña en vez de abrir una nueva.
3. CA-04.3 — Activar un enlace a un `.md` que no existe, o no se puede leer,
   muestra el aviso temporal de RF-17; no aparece ninguna pestaña nueva, y el
   documento que se estaba viendo se sigue mostrando.
4. CA-04.4 — Un enlace `http`/`https` en el mismo documento se sigue abriendo
   en el navegador (RF-15.1), sin que esta historia lo haya alterado.

**Estado.** pendiente

---

## Cobertura

| Requisito | Historia que lo cubre |
| --- | --- |
| RF-01.1 | HU-01 |
| RF-04.1 | HU-01, HU-04 |
| RF-05.1 | HU-02, HU-03 |
| RF-06.1 | HU-01 |
| RF-07.1 | HU-03 |
| RF-14.1 | HU-04 |
| RNF-03.1 | HU-01 |

Los seis requisitos funcionales de la feature están cubiertos; RNF-03.1 se
asigna a HU-01, la primera que se observa, igual que en las features
anteriores.
