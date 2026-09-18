# Historias — front-matter-y-html

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Cinco historias, un único tipo de usuario: el lector de documentación técnica de
`00-contexto.md`. RF-13.1 lista quince etiquetas heterogéneas —texto en línea,
bloques, tabla, sección plegable—; se reparten en cuatro historias en vez de
una sola para que cada una sea pequeña, en vez de exigir todo el subconjunto de
una sentada. El motivo del reparto exacto está en `plan.md`.

---

## HU-01 — Ver el front matter como una tabla de propiedades

Cubre: RF-12.1

**Historia.** Como lector de documentación técnica, quiero que el front matter
de un documento se muestre como una tabla legible, porque hoy un README con
front matter me enseña `---` y líneas `clave: valor` como si fueran parte del
texto que tengo que leer.

**Criterios de aceptación.** Se comprueban sobre un documento de prueba con
front matter de al menos tres propiedades, seguido de un encabezado y un
párrafo.

1. CA-01.1 — Un documento cuyo primer bloque es `---`, una o más líneas
   `clave: valor`, y `---`, muestra una tabla de dos columnas (propiedad,
   valor) en la parte alta del documento, antes de cualquier otro elemento.
2. CA-01.2 — Ni las líneas `---` ni la sintaxis `clave:` aparecen como texto
   en el cuerpo del documento: solo se ven los nombres y los valores, en la
   tabla.
3. CA-01.3 — Las propiedades aparecen en la tabla en el mismo orden en que
   están escritas en el documento.
4. CA-01.4 — El encabezado y el párrafo que siguen al front matter se siguen
   mostrando con normalidad debajo de la tabla.
5. CA-01.5 — Un documento cuyo `---` no está en la primera línea —por ejemplo,
   un párrafo seguido de una regla horizontal— no se trata como front matter:
   se muestra igual que antes de esta historia, regla horizontal incluida.

**Estado.** verificada. Los cinco criterios pasan por observación; el detalle
está en `docs/04-calidad.md`.

---

## HU-02 — Ver texto de HTML incrustado con el formato equivalente

Cubre: RF-13.1 (parcial: `b`, `strong`, `i`, `em`, `code`, `br`, `a`, `img`, y
el trato de las etiquetas no listadas)

**Historia.** Como lector de documentación técnica, quiero que el HTML en
línea que un documento incruste —negrita, cursiva, código, enlaces,
imágenes— se vea igual que si estuviera escrito en Markdown, porque hoy ese
HTML se descarta en silencio (ver «Defecto observado» de
`enlaces-e-imagenes/plan.md`, AD-15) y el documento pierde contenido.

**Criterios de aceptación.** Se comprueban sobre un documento de prueba con un
párrafo que mezcla las ocho etiquetas con texto Markdown normal alrededor.

1. CA-02.1 — `<b>texto</b>` y `<strong>texto</strong>` se muestran en negrita,
   igual que `**texto**`.
2. CA-02.2 — `<i>texto</i>` y `<em>texto</em>` se muestran en cursiva, igual
   que `*texto*`.
3. CA-02.3 — `<code>texto</code>` se muestra en tipografía monoespaciada con
   fondo distinguible, igual que `` `texto` ``.
4. CA-02.4 — `<a href="...">texto</a>` se muestra distinguible del texto que
   lo rodea y, al pulsarlo, se abre en el navegador igual que un enlace
   Markdown (RF-15.1).
5. CA-02.5 — `<img src="..." alt="...">` se muestra como la imagen que
   referencia, resuelta contra el directorio del documento igual que
   `![alt](src)` (RF-11.1); si no se puede mostrar, se ve su `alt`.
6. CA-02.6 — `<br>` dentro de un párrafo produce un salto de línea, y el texto
   que sigue se ve en la línea siguiente sin salir del párrafo.
7. CA-02.7 — Una etiqueta que no esté en la lista de RF-13.1 —por ejemplo
   `<span>`— no muestra su marcado, pero sí el texto que contiene, y el resto
   del párrafo se sigue mostrando.

**Estado.** en curso. Seis de los siete criterios pasan por observación
completa; CA-02.4 pasa en su parte visual (el enlace se ve distinguible) y
en que el clic llega a MDView y el navegador reacciona (procesos nuevos
justo después), pero falta la confirmación visual de qué pestaña abrió —
bloqueada por permisos del entorno de esta sesión, no por el código — ver
`docs/04-calidad.md`.

---

## HU-03 — Ver bloques de HTML incrustado con el formato equivalente

Cubre: RF-13.1 (parcial: `p`, `ul`, `li`, `hr`, `div align="center"`)

**Historia.** Como lector de documentación técnica, quiero que los párrafos,
listas, reglas y bloques centrados escritos en HTML dentro de un documento se
vean igual que sus equivalentes en Markdown, por la misma razón que HU-02:
hoy desaparecen.

**Criterios de aceptación.** Se comprueban sobre un documento de prueba con
cada una de las cinco etiquetas, y un encabezado antes y después.

1. CA-03.1 — `<p>texto</p>` se muestra como un párrafo, igual que ese mismo
   texto escrito sin etiquetas.
2. CA-03.2 — `<ul><li>uno</li><li>dos</li></ul>` se muestra como una lista no
   ordenada de dos elementos, con su marcador, igual que la lista Markdown
   equivalente.
3. CA-03.3 — `<hr>` se muestra como una regla horizontal, igual que `---` en
   Markdown.
4. CA-03.4 — `<div align="center">texto</div>` muestra su contenido centrado
   horizontalmente en la columna de lectura.
5. CA-03.5 — El encabezado anterior y el posterior a estos bloques se siguen
   mostrando: ningún bloque HTML trunca el documento.

**Estado.** verificada. Los cinco criterios pasan por observación; el
detalle está en `docs/04-calidad.md`.

---

## HU-04 — Ver una tabla de HTML incrustada con el formato equivalente

Cubre: RF-13.1 (parcial: `table`)

**Historia.** Como lector de documentación técnica, quiero que una tabla
escrita en HTML dentro de un documento se vea igual que una tabla Markdown,
por la misma razón que HU-02 y HU-03.

**Criterios de aceptación.** Se comprueban sobre un documento de prueba con
una tabla HTML de cabecera y dos filas, con `<th>`/`<td>`.

1. CA-04.1 — La tabla se muestra como una rejilla, con la fila de cabecera
   (`<th>`) distinguible del resto, igual que una tabla Markdown equivalente.
2. CA-04.2 — Cada celda (`<td>`) aparece en su columna y fila correctas.
3. CA-04.3 — El contenido antes y después de la tabla se sigue mostrando.

**Estado.** verificada. Los tres criterios pasan por observación; el
detalle está en `docs/04-calidad.md`.

---

## HU-05 — Ver una sección plegable de HTML incrustada

Cubre: RF-13.1 (parcial: `details`, `summary`)

**Historia.** Como lector de documentación técnica, quiero poder leer el
contenido de un `<details>` —un patrón habitual para esconder secciones
largas («Ver más», registros de cambios, capturas adicionales)— en vez de que
desaparezca como hoy.

**Criterios de aceptación.** Se comprueban sobre un documento de prueba con un
`<details>` con `<summary>` seguido de contenido, entre dos párrafos.
`details` y `summary` no tienen equivalente en CommonMark: qué cuenta como
«se muestra bien» para ellos lo fija la decisión de `decisiones.md` que se
tome al implementar esta historia, no estos criterios por sí solos.

1. CA-05.1 — El texto de `<summary>` se distingue visualmente del contenido
   de `<details>` que lo sigue.
2. CA-05.2 — El contenido de `<details>` es legible sin ninguna acción del
   usuario: no hace falta pulsar nada para leerlo. [decidido en
   `decisiones.md`: sin plegado interactivo, ver plan.md]
3. CA-05.3 — El párrafo anterior y el posterior al `<details>` se siguen
   mostrando.

**Estado.** verificada. Los tres criterios pasan por observación; el
detalle está en `docs/04-calidad.md`.

---

## Cobertura

| Requisito | Historia que lo cubre |
| --- | --- |
| RF-12.1 | HU-01 |
| RF-13.1 | HU-02, HU-03, HU-04, HU-05 |
| RNF-03.1 | HU-01 |

Los dos requisitos de la feature están cubiertos. RNF-03.1 se asigna a HU-01,
la primera que se observa, igual que en las dos features anteriores.
