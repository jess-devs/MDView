# Decisiones — ver-markdown-en-crudo

> Estado: cerrada, su única historia verificada
> Última actualización: 2026-09-18
> Modo: new-feature

Aquí se escribe toda decisión de esta feature que alguien pueda querer revertir
más adelante, con sus alternativas y sus consecuencias, y se indexa en el mismo
turno en `../../03-arquitectura.md`. La numeración `AD-nn` es continua en todo el
proyecto, y sigue desde AD-29 en `distribucion/decisiones.md`.

---

## AD-30 — Ver el crudo: campo `raw` en `DocumentTab`, reutilizando el desplazamiento horizontal de los bloques de código

Fecha: 2026-09-18

**Contexto.** RF-23.1 exige alternar, por pestaña, entre el documento
renderizado y su texto exacto. El texto ya está completo en memoria desde
que `document::load` lo lee — la única pregunta era dónde guardarlo y
cómo mostrarlo, no cómo obtenerlo.

**Decisión — guardar el texto en `DocumentTab`, no reanalizarlo ni
releerlo del disco al alternar.** `DocumentTab` gana `raw: String` (el
mismo `String` que ya se le pasaba a `markdown::parse`) y `raw_view:
bool`. Releer el archivo cada vez que el usuario alterna sería más
trabajo por menos control: si el archivo cambiara en disco entre medias,
el crudo y el renderizado mostrarían dos versiones distintas del mismo
documento, algo que ninguna historia pide y que además contradice
`01-alcance.md`, que excluyó a propósito la recarga automática.

**Decisión — reutilizar el `ScrollHandle` horizontal de `Block::CodeBlock`
(AD-10) para el documento crudo entero.** Se consideró envolver el texto
para que nunca fuera más ancho que la ventana, pero eso ya no sería «el
texto tal cual está en el archivo» (RF-23), que es exactamente el
problema que RF-10 ya resolvió para código: conservar la línea entera y
desplazarla, no partirla. Un único bloque monoespaciado con su propio
`ScrollHandle`, en vez de una lista de líneas, porque no hay ninguna
historia que necesite dirigirse a una línea suelta del crudo — a
diferencia de los bloques de código, que sí son varios por documento y
cada uno necesita su propia posición de scroll.

**Decisión — el control de alternar vive junto a la barra de pestañas,
no dentro de cada `Tab`.** Actúa sobre la pestaña activa, no la describe
visualmente: ponerlo en la fila de la `TabBar` (AD-25), como un elemento
más junto a ella, evita un botón por pestaña que once de cada doce veces
no se está mirando.

**Consecuencias.**

- Cada `DocumentTab` pesa el doble del texto que antes (una copia para
  `raw`, otra ya convertida en `blocks`). Ninguna historia impone un
  límite de memoria (`02-requisitos.md`, sección de requisitos que
  deliberadamente no existen); si se vuelve un problema real con
  documentos muy grandes, ahí se decide, no antes.
- Sin tests nuevos: alternar un `bool` y mostrar un `String` no tiene
  lógica de analizador que probar con `#[cfg(test)]` (AD-17); se verifica
  por observación, comparando el crudo mostrado contra el archivo real.

Estado: activa
