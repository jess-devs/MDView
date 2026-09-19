# Plan — ver-markdown-en-crudo

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

## Forma técnica

**Dónde vive el texto crudo.** `document::load` ya devuelve el `String`
completo del archivo; hoy `app::run`/`open_or_activate_tab` lo pasan a
`markdown::parse` y lo descartan. `DocumentTab` gana un campo `raw:
String` con ese mismo texto, y un campo `raw_view: bool` — el modo de esa
pestaña, `false` por defecto (RF-23.1: renderizado por defecto). Ningún
cambio en `document` ni en `markdown`: el texto ya estaba completo, solo
faltaba no tirarlo.

**El control para alternar.** Un `Button` de `gpui-component`, con la
misma familia de componentes que ya usan `render_tab_bar` y
`render_close_tab_button` (AD-25). Vive en la misma fila que la barra de
pestañas — un `div().flex()` con la `TabBar` a un lado y el botón al
otro—, no dentro de cada `Tab`: es un control sobre la pestaña activa, no
una propiedad visual de la pestaña en sí, y ponerlo una vez evita
siluetear un botón por pestaña que solo el activo necesita.

**Mutar el modo de la pestaña activa.** Mismo patrón que ya establecieron
AD-26 (clic muta `AppState` vía `Entity<DocumentView>::update`) y AD-27:
el botón captura un `Entity<DocumentView>` y en su `on_click` invierte
`raw_view` de `state.tabs[state.active_tab]`. Sin función libre nueva en
`app.rs`: es una sola línea, no hay lógica que valga la pena separar del
sitio donde se usa (a diferencia de `close_tab`/`open_or_activate_tab`,
que sí encapsulan invariantes — este toggle no tiene ninguno).

**Cómo se ve el crudo.** Se reutiliza el mecanismo exacto de
`Block::CodeBlock` (AD-10): un `ScrollHandle` propio, tipografía
`mono_font_family`, `overflow_x_scroll` para que una línea ancha se
desplace en vez de partirse (RF-23.2, RF-10 ya establecía esta regla para
código). La diferencia es que aquí el contenido es `tab.raw` entero, no
un bloque — un único elemento de texto monoespaciado con el archivo tal
cual, en vez de iterar `tab.blocks`. `DocumentView::render` decide cuál
de las dos ramas construir según `tab.raw_view`, antes de entrar al bucle
que hoy itera `tab.blocks`.

**Qué no cambia.** `markdown::parse` sigue analizando el documento al
cargarlo, en modo renderizado o no: no tiene sentido diferir el análisis
a que el usuario nunca pida ver el crudo, y complicaría el código para un
ahorro que RNF-01 no necesita (el análisis ya ocurre dentro del
presupuesto de arranque en frío).

## Riesgos

- **Documentos muy largos en crudo.** Un único elemento de texto con todo
  el archivo dentro podría ser más lento de diseñar que la lista de
  bloques equivalente. No hay una historia que fije un umbral de tamaño;
  si se nota, es el primer punto a revisar, no una razón para no
  construirlo así ahora.
