# Historias — integracion-con-windows

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Tres historias, un único tipo de usuario: el lector de documentación técnica
de `00-contexto.md`. Cubren RF-03.1 únicamente — RF-02 queda pendiente hasta
que exista la feature de distribución, decisión del usuario del 2026-09-18,
registrada en `requisitos.md`.

---

## HU-01 — Una segunda invocación con una ruta nueva se atiende desde el proceso existente

Cubre: RF-03.1 (parcial: ruta nueva)

**Historia.** Como lector de documentación técnica, quiero que abrir otro
`.md` mientras MDView ya está abierto añada una pestaña al mismo proceso en
vez de abrir una ventana nueva, para tener todos mis documentos juntos sin
acumular ventanas — el mismo comportamiento que un navegador.

**Criterios de aceptación.** Se comprueban con MDView ya abierto con un
documento (proceso A), invocando una segunda vez con la ruta de un segundo
documento de prueba distinto.

1. CA-01.1 — Tras la segunda invocación, no hay un segundo proceso de
   MDView: solo sigue en marcha el primero.
2. CA-01.2 — El segundo documento aparece en una pestaña nueva del proceso
   ya en marcha, y esa pestaña queda activa.
3. CA-01.3 — El documento que ya estaba abierto sigue en su pestaña, sin
   cambios.
4. CA-01.4 — La ventana del proceso ya en marcha pasa a primer plano.

**Estado.** pendiente

---

## HU-02 — Una segunda invocación con una ruta ya abierta activa su pestaña

Cubre: RF-03.1 (parcial: ruta repetida), RF-04.1

**Historia.** Como lector de documentación técnica, quiero que invocar
MDView con la ruta de un documento que ya tengo abierto me lleve a esa
pestaña en vez de abrir una copia, igual que si hubiera pulsado un enlace a
él (RF-14.1).

**Criterios de aceptación.** Se comprueban con MDView ya abierto con dos
documentos de prueba, invocando una segunda vez con la ruta del que no está
activo.

1. CA-02.1 — Tras la segunda invocación, sigue habiendo el mismo número de
   pestañas que antes: ninguna nueva.
2. CA-02.2 — La pestaña del documento pedido queda activa.
3. CA-02.3 — La ventana pasa a primer plano, igual que en HU-01.

**Estado.** pendiente

---

## HU-03 — Una segunda invocación sin ninguna ruta trae la ventana a primer plano

Cubre: RF-03.1 (parcial: sin ruta)

**Historia.** Como lector de documentación técnica, quiero que volver a
lanzar MDView sin indicarle ningún archivo —por ejemplo, desde un acceso
directo— me lleve a la ventana que ya tengo abierta en vez de no hacer nada
o de confundirme con una ventana vacía nueva.

**Criterios de aceptación.** Se comprueban con MDView ya abierto con al
menos un documento, invocando una segunda vez sin ningún argumento.

1. CA-03.1 — La ventana del proceso ya en marcha pasa a primer plano.
2. CA-03.2 — La pestaña activa no cambia.
3. CA-03.3 — Ningún documento abierto se ve alterado.

**Estado.** pendiente

---

## Cobertura

| Requisito | Historia que lo cubre |
| --- | --- |
| RF-03.1 | HU-01, HU-02, HU-03 |
| RF-04.1 | HU-02 (reafirma, ya construido en `varios-documentos-en-pestanas`) |
| RNF-03.1 | HU-01 |

RF-03.1 es el único requisito funcional de esta feature; sus tres partes
—ruta nueva, ruta repetida, sin ruta— están cubiertas. RNF-03.1 se asigna a
HU-01, la primera que se observa.
