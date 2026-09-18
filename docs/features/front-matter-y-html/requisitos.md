# Requisitos — front-matter-y-html

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Esta feature cubre la otra mitad de lo que `ver-un-documento/plan.md` llamaba
`contenido-enriquecido`: marcado que el analizador de Markdown todavía no
entiende, sin entrada ni salida nueva —a diferencia de `enlaces-e-imagenes`, que
necesitaba el disco y el navegador. La partición y el motivo están en
`enlaces-e-imagenes/plan.md`.

Cada requisito refina uno de `02-requisitos.md`. Un refinamiento **acota**: aquí
no aparece nada que su requisito padre no tuviera ya. Los términos usados están
en el glosario de `00-contexto.md`.

## Funcionales

### RF-12.1 — Front matter como tabla de propiedades

Refina: RF-12
Si el documento empieza —desde su primer byte, sin líneas en blanco ni
contenido antes— con un bloque delimitado por una línea `---` sola, más
contenido, más otra línea `---` sola, el sistema debe presentarlo como una
tabla de dos columnas (nombre de la propiedad, valor) en la parte alta del
documento, antes de cualquier otro elemento, y no como texto en crudo ni como
reglas horizontales.

Un documento cuyo primer bloque **no** tiene esa forma —el `---` no está en la
primera línea, o no hay un segundo `---` que lo cierre— no tiene front matter:
se muestra como el resto del documento siempre se ha mostrado, regla horizontal
incluida si el `---` aparece más adelante. Esto no amplía RF-12: fija cuándo
«el documento empieza con un front matter» es cierto, que el padre no decidía.

### RF-13.1 — Subconjunto de HTML incrustado

Refina: RF-13
El sistema debe interpretar, cuando aparezcan dentro del Markdown, las
etiquetas `img`, `a`, `b`, `strong`, `i`, `em`, `code`, `br`, `hr`, `p`, `ul`,
`li`, `table`, `details`, `summary`, y `div` con atributo de alineación
centrada, mostrándolas con el formato equivalente al del elemento Markdown
correspondiente:

| Etiqueta | Equivalente Markdown |
| --- | --- |
| `img` | `![alt](src)` — RF-11 |
| `a` | `[texto](href)` — RF-08 |
| `b`, `strong` | negrita |
| `i`, `em` | cursiva |
| `code` | código en línea |
| `br` | salto de línea dentro de un párrafo |
| `hr` | regla horizontal |
| `p` | párrafo |
| `ul`, `li` | lista no ordenada |
| `table` (con `tr`/`td`/`th`) | tabla |
| `div align="center"` | el propio `div`, con su contenido centrado |

Los atributos de estilo y las hojas de estilo se ignoran. De una etiqueta que
no esté en esta lista no se muestra su marcado, pero sí el texto que
contenga, incluido el de las etiquetas anidadas dentro de ella que tampoco
estén en la lista.

`details` y `summary` no tienen equivalente en CommonMark: no existe una
sección plegable en Markdown. Cómo se muestran es una decisión de esta
feature, no un refinamiento — se registra en `decisiones.md` al
implementarse, no aquí.

## No funcionales

### RNF-03.1 — Plataforma de ejecución

Refina: RNF-03
El sistema debe ejecutarse en Windows 11 de 64 bits.

## Requisitos del sistema que esta feature no cubre

RF-01 a RF-07, RF-14, y las once historias ya cerradas de RF-08, RF-09,
RF-10, RF-11, RF-15, RF-16, RF-17, RF-18, RF-19, RF-20, RNF-01 y RNF-02 en
`ver-un-documento`/`enlaces-e-imagenes`. RF-14 va en
`varios-documentos-en-pestanas`; el resto de RF-01 a RF-07 en
`integracion-con-windows`, según el reparto de
`ver-un-documento/plan.md`.

RNF-01 y RNF-02 no se refinan aquí: esta feature no toca el disco ni la red
más allá de lo que ya hace el analizador de Markdown existente —analizar más
texto (el front matter, el HTML incrustado) es cómputo puro, no E/S nueva—,
así que no hay ningún caso de fallo ni ningún umbral nuevo que fijar. Se
vigila igualmente con una medida informal al cerrar la feature, no como
criterio de aceptación.
