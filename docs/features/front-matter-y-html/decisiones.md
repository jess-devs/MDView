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
