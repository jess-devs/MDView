# Plan — edicion-con-confirmacion

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

## Forma técnica

**El editor: `gpui_component::input`, no algo propio.** Se leyó su código
fuente (`gpui-component-0.5.1/src/input/`) antes de decidir, no se asumió
por el nombre del módulo. Expone `InputState` (`multi_line(true)`,
`set_value`, `value()`, `text()` como `Rope`) con cursor, selección,
edición internacional y un historial de deshacer/rehacer ya cableado a
las teclas del sistema (`InputState::undo`/`redo`, en `input.rs`, vía
`on_action`). Es exactamente lo que `01-alcance.md` anticipaba en su nota
de coste condicionado a Fase 4: adoptar `gpui-component` (AD-01) abarata
esta función si se apoya en su componente. Nada de esto se reimplementa.

**Modelo de estado: un enum, no un segundo `bool` junto a `raw_view`.**
`DocumentTab.raw_view: bool` (AD-30) se sustituye por
`view: ViewMode { Rendered, Raw, Editing(Entity<InputState>) }`. Un
`bool` adicional para «editando» permitiría el estado imposible
«`raw_view = true` y editando a la vez», que no significa nada —un enum
lo hace irrepresentable, no solo indocumentado. `Editing` carga su
propio `Entity<InputState>` (necesita `Window`/`Context` para crearse,
así que se construye en el `on_click` del botón «Editar», no antes).

**Guardar.** Un botón junto al de «Editar»/«Ver crudo», visible solo en
`ViewMode::Editing`, que: lee `input_state.value()`, escribe ese texto en
`tab.path` (`std::fs::write`, igual de directo que `document::load` lee),
vuelve a analizarlo con `markdown::parse` para refrescar `tab.blocks`
—el mismo camino que ya recorre abrir el archivo—, actualiza `tab.raw`
con el nuevo texto, y pone `view` de vuelta en `ViewMode::Rendered`
(RF-24.3: guardar muestra el renderizado). El atajo de teclado del
sistema para guardar se resuelve con un `KeyBinding`/`actions!` de GPUI
sobre el mismo handler, siguiendo el propio patrón de `gpui-component`
para acciones (`Undo`/`Redo` en `input.rs`).

**Cambios sin guardar.** No se guarda un `bool` aparte que pueda
desincronizarse: la marca se deriva comparando `input_state.value()`
contra `tab.raw` en cada `render` — si difieren, hay cambios sin
guardar. Es literalmente la comparación que CA-03.3 pide («deshacer
hasta volver al original quita la marca»): con un `bool` propio habría
que decidir cuándo ponerlo a `false` aparte de guardar; comparando
contra `tab.raw`, deshacer todo ya lo dispara solo.

**El diálogo de confirmación.** `gpui_component::dialog::Dialog`, vía
`Window::open_dialog` (confirmado en `root.rs`: `WindowExt::open_dialog`,
misma familia que `push_notification`, que el proyecto ya usa desde
AD-12). Tres botones —guardar, descartar, cancelar— en su `footer`, cada
uno cerrando el diálogo y, si corresponde, completando el cierre de la
pestaña que `close_tab` ya sabía hacer.

**RF-07 revisado.** `close_tab` (`app.rs`) no decide más si cerrar
termina la aplicación: antes de llamarlo, `render_close_tab_button`
comprueba si esa pestaña tiene cambios sin guardar. Si no los tiene, el
camino es exactamente el de hoy. Si los tiene, se abre el diálogo primero
y `close_tab` solo se invoca desde sus botones — incluido el caso de la
última pestaña, que hoy termina la app: ese `cx.quit()` pasa a ocurrir
dentro de «Guardar» o «Descartar» del diálogo, nunca antes de que el
usuario responda.

## Riesgos

- **`InputState::value()` como fuente de verdad de lo editado.** Si el
  componente normaliza saltos de línea o espacios de forma que
  `value() != texto tecleado`, guardar reescribiría el archivo con un
  formato distinto al que el usuario ve. No hay indicio de esto en el
  código leído, pero CA-02.1 lo comprobará contra el archivo real, no
  contra lo que se asuma.
- **`Rope` vs `String`.** `text()` devuelve un `ropey::Rope`; convertirlo
  a `String` para `std::fs::write` y para comparar contra `tab.raw` es
  una asignación por guardado/frame, aceptable para el tamaño de archivo
  que este proyecto ya asume (RNF-01 mide con ~50 KB).
