# Decisiones — edicion-con-confirmacion

> Estado: cerrada, sus cuatro historias verificadas
> Última actualización: 2026-09-18
> Modo: new-feature

Aquí se escribe toda decisión de esta feature que alguien pueda querer revertir
más adelante, con sus alternativas y sus consecuencias, y se indexa en el mismo
turno en `../../03-arquitectura.md`. La numeración `AD-nn` es continua en todo el
proyecto, y sigue desde AD-30 en `ver-markdown-en-crudo/decisiones.md`.

---

## AD-31 — Editar es editar el crudo con `gpui_component::input`; `ViewMode` reemplaza a `raw_view`; RF-07 se revisa con un diálogo, no en silencio

Fecha: 2026-09-18

**Contexto.** RF-24 pide edición con confirmación explícita.
`01-alcance.md` ya advertía que un editor completo es plausiblemente más
trabajo que el resto del proyecto junto; `requisitos.md` de esta feature
acota qué significa «editar» aquí antes de tocar código, no después.

**Decisión — editar el texto crudo, no el árbol renderizado (WYSIWYG).**
Alternativa considerada y descartada: editar directamente sobre los
`Block`/`Inline` ya renderizados, con controles de formato. Se descartó
por desproporcionada frente a lo que cualquier historia pide: nadie
necesita un procesador de texto, necesitan corregir una errata sin salir
de MDView. Editar el crudo reutiliza además, íntegro, el mecanismo de
`ver-markdown-en-crudo` (AD-30): mismo texto, ahora editable.

**Decisión — el editor es `gpui_component::input::InputState`, no algo
propio.** Se leyó su código antes de decidir (`gpui-component-0.5.1/src/
input/`): multilínea, cursor, selección, deshacer/rehacer con historial
propio, ya cableados a las teclas del sistema. Construir esto a mano
sería repetir, peor, lo que la dependencia ya adoptada (AD-01) trae de
fábrica — exactamente el ahorro que `01-alcance.md` dejó anotado como
condicionado a qué se decidiera en la Fase 4.

**Decisión — `ViewMode` reemplaza a `raw_view: bool` (AD-30), pero vive
sin ningún tipo de GPUI dentro.** `enum ViewMode { Rendered, Raw,
Editing }`, sin payload, en `app.rs` junto a `DocumentTab` — dos
booleanos independientes permitirían el estado sin sentido «crudo y
editando a la vez», que un enum hace irrepresentable en vez de solo
indocumentado. El `Entity<InputState>` de la pestaña en edición **no**
vive en `DocumentTab`: eso metería un tipo de componente de interfaz
dentro del módulo que, por AD-04, no debe conocer el aspecto de nada.
Vive en `DocumentView` (`render.rs`), en un
`HashMap<PathBuf, Entity<InputState>>` propio, con la misma clave —la
ruta canonicalizada— que ya identifica una pestaña desde AD-25. Es una
revisión de AD-30, no su abandono: `Raw` sigue siendo exactamente lo que
AD-30 construyó, ahora una variante entre otras.

**Decisión — los cambios sin guardar se detectan comparando, no con un
`bool` de estado.** `InputState::value()` contra `tab.raw`: si difieren,
hay cambios. Un `bool` propio exigiría decidir en cada punto del código
cuándo ponerlo a `false`; la comparación lo resuelve solo, incluido
CA-03.3 (deshacer hasta el original quita la marca sin guardar nada).

**Decisión — RF-07 se revisa con el diálogo de `gpui_component::dialog`,
no reescribiendo `close_tab`.** `close_tab` (`app.rs`) no cambia: sigue
siendo la función que cierra una pestaña y dice si eso vació la lista.
Lo que cambia es cuándo se la llama — `render_close_tab_button` primero
comprueba si hay cambios sin guardar; si los hay, abre un `Dialog`
(`Window::open_dialog`, la misma familia de extensión de `Window` que
`push_notification`, que el proyecto ya usa desde AD-12) y `close_tab`
se invoca solo desde los botones de ese diálogo. La última pestaña —la
que hoy termina la aplicación— no es un caso especial: simplemente es la
vez en que `close_tab` devuelve `true` y `cx.quit()` se llama, ahora
después de que el usuario respondió, no antes.

**Consecuencias.**

- Fuera de esta entrega, dicho expresamente en `requisitos.md`: detectar
  que el archivo cambió en disco durante la edición. Si se construye
  luego, es una feature propia, no una ampliación silenciosa de esta.
- Sin tests nuevos previstos más allá de los que ya existen: la lógica
  nueva (comparar `value()` con `raw`, decidir cuándo abrir el diálogo)
  es E/S y estado de UI, verificado por observación como el resto del
  proyecto (AD-17).

Estado: activa
