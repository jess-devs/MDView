# Decisiones — front-matter-y-html

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Aquí se escribe toda decisión de esta feature que alguien pueda querer revertir
más adelante, con sus alternativas y sus consecuencias, y se indexa en el mismo
turno en `../../03-arquitectura.md`. La numeración `AD-nn` es continua en todo el
proyecto, y sigue desde AD-20 en `enlaces-e-imagenes/decisiones.md`.

---

## AD-21 — Front matter: extracción propia sobre el texto crudo, no `ENABLE_YAML_STYLE_METADATA_BLOCKS`

Fecha: 2026-09-18

**Contexto.** RF-12.1 exige tratar como front matter únicamente un bloque
`---`/`---` que empieza en la primera línea del documento, sin nada antes.
`pulldown-cmark` 0.13.4 trae una opción para esto,
`Options::ENABLE_YAML_STYLE_METADATA_BLOCKS`.

**Alternativas consideradas.**

- *Activar `ENABLE_YAML_STYLE_METADATA_BLOCKS` y dejar que `pulldown-cmark`
  reconozca el bloque.* Rechazada tras leer `firstpass.rs` y `scanners.rs`
  del propio crate: `scan_metadata_block` se llama desde `parse_block`, el
  despachador genérico de bloques, sin ninguna comprobación de que el bloque
  esté en la posición 0 del documento. Un `---`/`clave: valor`/`---` que
  apareciera en mitad de un documento —tras un párrafo, dentro de una cita—
  se habría reconocido igual que uno al principio, que es exactamente lo que
  CA-01.5 prohíbe. Además la opción solo reconoce el bloque como `MetadataBlock`
  en el árbol de eventos; sigue sin parsear el YAML de dentro, así que de
  todos modos habría hecho falta escribir el `key: value` a mano.
- *Extracción propia sobre `&str`, antes de construir el `Parser`.* Elegida.
  `extract_front_matter` comprueba que el documento empiece literalmente por
  `---` seguido de un salto de línea, busca la siguiente línea que sea
  exactamente `---`, y recorta ese tramo —delimitadores incluidos— antes de
  pasar el resto a `pulldown_cmark::Parser::new_ext`. Es cómputo puro sobre
  una porción de la cadena (`str::strip_prefix`, `str::find`), no E/S: no
  contradice que `markdown` no toque el disco.

**Decisión.** `markdown::parse` llama a `extract_front_matter(source)` antes
de construir el `Parser`, y antepone `Block::FrontMatter(Vec<(String,
String)>)` al árbol de bloques si lo encuentra. El análisis de cada línea es
deliberadamente mínimo, no YAML de verdad: divide en la primera `:` con
`str::split_once`, recorta espacios a los lados, e ignora en silencio
cualquier línea sin `:` — no hay listas anidadas, tipos, ni comillas. RF-12.1
pide una tabla de propiedades, no fidelidad con la especificación YAML, y
todo valor —sea texto, fecha o número— se muestra igual en una celda de
tabla.

**Consecuencias.**

- Un bloque cuya primera línea tras el `---` de apertura está en blanco, o
  cuyo `---` de cierre nunca aparece, no se trata como front matter y se
  reprocesa como contenido normal (cubierto por
  `an_unclosed_leading_dashes_block_is_not_front_matter`).
- Una línea de front matter que use estructura YAML real —una lista con
  `- item`, un valor entre comillas con `:` dentro sin espacio, un mapa
  anidado— no se interpreta como tal: si tiene `:`, la parte después del
  primer `:` se toma entera como valor de texto; si no lo tiene, la línea se
  descarta sin más. Si una historia futura necesita listas o valores
  anidados en la tabla, esto hay que revisarlo entonces, no antes: ningún
  documento de prueba de esta historia lo pide.
- `render::render_front_matter` reutiliza el estilo visual de
  `render_table` en vez de tener uno propio: el front matter es metadato,
  no algo que el lector edite, así que no necesita su propio aspecto.

---

## AD-22 — Intérprete de HTML incrustado: módulo `html` propio, una etiqueta por evento

Fecha: 2026-09-18

**Contexto.** RF-13.1 exige interpretar quince etiquetas de HTML fijas.
`plan.md` ya investigó, antes de escribir código, que `pulldown_cmark`
entrega el HTML incrustado como texto crudo sin tokenizar
(`Event::InlineHtml`/`Event::Html`), así que hace falta un intérprete
propio del subconjunto cerrado.

**Alternativas consideradas.**

- *Depender de `html5ever`/`markup5ever`,* ya presentes de forma transitiva
  vía `gpui-component`. Rechazada: son un analizador y un árbol DOM de HTML
  completo, con el algoritmo de construcción de árbol de la especificación
  —manejo de HTML mal formado, reglas de reubicación de nodos, espacios de
  nombres—, pensado para HTML de verdad en una página web. Interpretar
  quince etiquetas fijas con ese motor sería resolver un problema mucho más
  grande que el que RF-13.1 plantea, y añadirlo como dependencia directa
  exigiría justificar esa complejidad sin necesitarla —el mismo criterio que
  `01-alcance.md` ya aplicó para descartar «HTML completo con hojas de
  estilo».
- *Un tokenizador de una sola etiqueta por llamada, sin árbol.* Elegida.
  `html::parse_tag(raw: &str) -> Option<Tag>` interpreta el texto de un
  único evento (`<b>`, `</b>`, `<img src="..." alt="...">`) en nombre,
  abre/cierra, autocierre y atributos. No construye ninguna estructura de
  documento: cada `Event::InlineHtml`/`Event::Html` que `pulldown_cmark` ya
  entrega por separado se interpreta por separado, en el mismo bucle que ya
  recorre los demás eventos de `markdown::parse_paragraph_inline`. Encaja
  con la forma en que la biblioteca entrega los datos en vez de pelearse con
  ella construyendo un árbol que esta biblioteca no da y que este alcance no
  pide.

**Decisión.** Nuevo módulo `src/html.rs`, quinto módulo del proyecto —
`03-arquitectura.md` pasa de cuatro a cinco—, con la misma regla que los
demás: no conoce GPUI ni `gpui-component`. Vive aparte de `markdown` porque
es una gramática distinta (etiquetas HTML, no bloques e inlines de
CommonMark) que va a crecer en las tres historias siguientes (HU-03 a
HU-05); meterlo dentro de `markdown.rs` lo habría dejado como un archivo de
mil líneas mezclando dos analizadores. `markdown::apply_inline_html_tag`
traduce el resultado de `html::parse_tag` a los mismos campos que ya usaba
el analizador de Markdown (`style: &mut SpanStyle`, `link: &mut
Option<String>`, `out: &mut Vec<Inline>`): una etiqueta reconocida cambia
esos campos exactamente igual que su equivalente Markdown, así que
`render` sigue sin saber que un fragmento vino de HTML.

Una etiqueta que `html::parse_tag` no reconoce como bien formada —falta el
`<` o el `>`, es un comentario, no tiene nombre— y una etiqueta reconocida
pero fuera de las ocho que HU-02 interpreta (`b`, `strong`, `i`, `em`,
`code`, `a`, `img`, `br`) reciben el mismo trato: `apply_inline_html_tag` no
hace nada con ellas. El texto entre su apertura y su cierre llega de todos
modos como eventos `Text` normales, ajenos a esta función, así que se sigue
mostrando sin el marcado de la etiqueta — es la letra exacta de RF-13.1
para las etiquetas no listadas, no un caso aparte que haya que programar.

**Consecuencias.**

- Etiquetas cruzadas de forma inválida —`<b><i>x</b></i>`— no se detectan
  ni se corrigen: cada atributo de estilo es un booleano independiente
  (`style.bold`, `style.italic`, `style.code`) que la etiqueta de cierre
  correspondiente apaga, sin pila de etiquetas abiertas. Es el mismo modelo
  que ya usaban `Emphasis`/`Strong` de Markdown antes de esta historia; un
  navegador real corrige este caso reubicando nodos, cosa que este alcance
  no necesita porque ningún documento de prueba lo pide.
- `<img>` dentro del flujo de un párrafo usa los atributos `src`/`alt`
  directamente — no acumula texto entre apertura y cierre como hace la
  imagen Markdown, porque `<img>` no tiene contenido, es una etiqueta sin
  cierre semántico. Un `<img>` solo en su propia línea, rodeado de líneas en
  blanco, no llega por este camino: CommonMark lo reconoce como un bloque
  HTML de tipo 7, no como HTML en línea dentro de un párrafo. Ese caso es
  del intérprete de bloques que construyen HU-03 a HU-05, no de este.

Estado: activa
