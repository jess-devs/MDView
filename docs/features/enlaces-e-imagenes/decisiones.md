# Decisiones — enlaces-e-imagenes

> Estado: vacío, la feature no ha empezado a construirse
> Última actualización: 2026-08-24
> Modo: new-feature

Aquí se escribe toda decisión de esta feature que alguien pueda querer revertir
más adelante, con sus alternativas y sus consecuencias, y se indexa en el mismo
turno en `../../03-arquitectura.md`. La numeración `AD-nn` es continua en todo el
proyecto: la última registrada es AD-14, así que la primera de esta feature será
AD-15.

De momento no hay ninguna. Las que se sabe que van a aparecer, con lo que ya se
averiguó de ellas, son:

- **Cómo se abren los enlaces externos** (RF-15.1, HU-02). El crate `open`,
  versión 5.4.1 publicada el 2026-08-05 y licencia MIT, es el candidato
  evidente. Viene anunciada desde `../ver-un-documento/decisiones.md`.
- **Dónde vive la ruta del archivo abierto** (RF-11.1, HU-03). Hoy
  `document::load` devuelve solo el texto, y resolver una ruta relativa exige
  saber en qué directorio está el `.md`. Toca la frontera entre `document`,
  `markdown` y `render` que describe `../../03-arquitectura.md`.
- **Con qué se decodifican las imágenes** (RF-11.1, HU-03). Sea lo que sea,
  `../../03-arquitectura.md` exige que ninguna dependencia que abra conexiones
  entre sin justificarlo, y RNF-02.1 fija cero conexiones.
- **Cómo se arregla la pérdida de contenido de `parse_blocks`** (RF-08.2,
  HU-01). El defecto está descrito y observado en `plan.md`. Si el arreglo
  cambia la forma del árbol de elementos —un `Block` o un `Span` nuevo—, eso es
  un cambio en un tipo compartido y se registra.
