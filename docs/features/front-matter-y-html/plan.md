# Plan — front-matter-y-html

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

## Orden de las features del proyecto

Sin cambios respecto a `enlaces-e-imagenes/plan.md`:

1. `ver-un-documento` — cerrada.
2. `enlaces-e-imagenes` — cerrada, sus cinco historias verificadas.
3. **`front-matter-y-html`** — esta. RF-12.1 y RF-13.1.
4. `varios-documentos-en-pestanas` — RF-14 y las pestañas.
5. `integracion-con-windows` — la última.

## Orden de las historias

1. **HU-01 — Front matter.** Primera porque no depende de nada ni comparte
   código con las demás: es un bloque de documento que se reconoce una vez,
   al principio del análisis, y no vuelve a aparecer.
2. **HU-02 — Texto de HTML en línea.** Segunda porque construye el motor de
   reconocimiento de etiquetas (abrir/cerrar, atributos) que las tres
   historias siguientes reutilizan, y lo hace sobre el terreno más fácil:
   ocho etiquetas que ya tienen un lugar exacto en `Inline`/`Span`
   (RF-08.2 y RF-11.1 de `enlaces-e-imagenes` ya dieron ese lugar).
3. **HU-03 — Bloques de HTML.** Depende del motor de HU-02. `p`, `hr` y
   `div` son directos; `ul`/`li` es la primera vez que ese motor tiene que
   entender anidamiento (una lista dentro de un bloque HTML), no solo
   apertura y cierre planos.
4. **HU-04 — Tabla de HTML.** Depende del motor de HU-02, no de HU-03: una
   tabla no anida con listas ni párrafos en los documentos que esta feature
   cubre. Va después de HU-03 por si el trabajo de anidamiento de `ul`/`li`
   deja algo reutilizable para las filas de `table`, no por dependencia
   estricta.
5. **HU-05 — `details`/`summary`.** Última porque es la única sin
   equivalente en CommonMark: hace falta una decisión de diseño propia (ver
   «Riesgos»), y conviene tomarla con el motor de HU-02/HU-03 ya probado
   contra algo real, no como la primera etiqueta que se interpreta.

## Dependencias

| Historia | Depende de | Motivo |
| --- | --- | --- |
| HU-01 | Ninguna | Front matter es un bloque de documento aparte, sin relación con el HTML incrustado. |
| HU-02 | Ninguna | Primera en tocar HTML: construye el motor de reconocimiento de etiquetas. |
| HU-03 | HU-02 | Reutiliza el motor de reconocimiento de etiquetas para bloques, no solo en línea. |
| HU-04 | HU-02 | Misma razón que HU-03; no depende de HU-03 en sí. |
| HU-05 | HU-03 | `details` es un contenedor de bloque, como `div`/`ul`: necesita que ese nivel del motor ya exista. |

Con el límite de trabajo en curso de 1, las historias se hacen de una en una
aunque HU-02 y HU-04 no dependan directamente entre sí.

## Forma técnica del motor de HTML (HU-02 a HU-05)

Investigado el 2026-09-18, antes de escribir código, leyendo
`pulldown-cmark` 0.13.4:

- El HTML incrustado llega como texto **crudo**, no tokenizado por etiqueta:
  `Event::InlineHtml(CowStr)` para el HTML dentro del flujo de un párrafo
  (normalmente una etiqueta suelta por evento: `<b>`, luego el texto normal
  como `Event::Text`, luego `</b>`), y `Event::Html(CowStr)` dentro de
  `Tag::HtmlBlock`/`TagEnd::HtmlBlock` para el HTML que ocupa un bloque
  entero. `pulldown-cmark` analiza Markdown, no HTML: no interpreta las
  etiquetas, solo dice «esto es HTML, no Markdown».
- **Consecuencia para el diseño.** Hace falta un intérprete propio, pequeño,
  del subconjunto cerrado de quince etiquetas de RF-13.1 — no un motor HTML
  general. `01-alcance.md` ya descartó «HTML completo con hojas de estilo»
  por el objetivo de arranque en menos de 1,2 s; nada distinto aplica aquí.
  Sigue el mismo criterio que AD-02: analizador propio sobre la biblioteca
  existente, no una dependencia nueva que sepa HTML de verdad (`html5ever`
  ya es una dependencia transitiva de `gpui-component`, pero traerlo como
  dependencia directa para interpretar quince etiquetas fijas sería
  bastante más de lo que este alcance necesita).
- **Dónde vive.** El intérprete de etiquetas es parte de `markdown`, igual
  que el analizador de Markdown mismo: convierte HTML crudo en el mismo
  árbol `Inline`/`Block` que ya produce el Markdown, así que `render` sigue
  sin saber que una etiqueta vino de HTML y no de `**negrita**`. La
  decisión formal, con el diseño exacto del intérprete, se registra como AD
  al implementar HU-02 — aquí solo se deja constancia de que se investigó
  antes de decidir, no después.

## Riesgos

- **`details`/`summary` no tiene equivalente en Markdown.** RF-13.1 pide
  «el formato equivalente al del elemento Markdown correspondiente», y no
  hay ninguno. La opción más simple —y la que se toma por defecto salvo que
  se decida lo contrario al implementar HU-05— es mostrar `summary` como una
  línea distinguible seguida del contenido de `details` siempre visible, sin
  plegado interactivo: MDView es un visor de solo lectura (AD-05) cuyo
  objetivo es que todo se pueda leer de un vistazo, y un widget plegable con
  estado propio (expandido/colapsado, por bloque, persistente mientras el
  documento esté abierto) es la clase de complejidad que este proyecto evita
  cuando no la pide un criterio. Si al implementar HU-05 se decide lo
  contrario, se registra ahí, no aquí.
- **Etiquetas mal anidadas o sin cerrar.** El documento de prueba de cada
  historia trae HTML bien formado; un documento real puede traer una
  etiqueta sin cerrar o cerrada en el orden equivocado. El intérprete debe
  degradar sin colgarse ni truncar el documento —la misma exigencia que
  AD-15 ya fijó para los bloques HTML sin interpretar—, pero no hay ningún
  criterio de aceptación que lo exija explícitamente con un caso de prueba
  propio. Se trata como el resto de esta feature: cómputo puro, sin panic,
  por la regla general de `03-arquitectura.md`, no por un CA dedicado.
- **Ninguna dependencia nueva entra sin decisión registrada.** Igual que en
  `enlaces-e-imagenes/plan.md`: si en algún punto de HU-02 a HU-05 parece
  que hace falta una biblioteca de HTML de verdad, esa es la señal de que el
  alcance se está saliendo del subconjunto cerrado de RF-13.1, no una razón
  para añadirla sin más.

## Documentos de prueba

Cada historia trae el suyo, pequeño y de un solo propósito, siguiendo el
patrón ya usado en `enlaces-e-imagenes` (`pruebas/hu-0N-*.md`, no
versionado). No hace falta un documento de prueba de arranque para esta
feature: no refina RNF-01, así que no hay una HU-06 de medición que lo
necesite.
