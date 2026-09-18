# Decisiones — varios-documentos-en-pestanas

> Estado: cerrada, sus cuatro historias verificadas
> Última actualización: 2026-09-18
> Modo: new-feature

Aquí se escribe toda decisión de esta feature que alguien pueda querer revertir
más adelante, con sus alternativas y sus consecuencias, y se indexa en el mismo
turno en `../../03-arquitectura.md`. La numeración `AD-nn` es continua en todo el
proyecto, y sigue desde AD-24 en `front-matter-y-html/decisiones.md`.

---

## AD-25 — `AppState` con una lista de pestañas; identidad por ruta canonicalizada; `Tab`/`TabBar` de `gpui-component`

Fecha: 2026-09-18

**Contexto.** Hasta `front-matter-y-html`, `AppState` tenía un único
`blocks: Vec<markdown::Block>` para el único documento que MDView sabía
mostrar. RF-01.1 exige varios a la vez, cada uno en su pestaña.

**Decisión — la lista.** `AppState.blocks` se sustituye por
`tabs: Vec<DocumentTab>` (`DocumentTab { path: PathBuf, blocks:
Vec<markdown::Block> }`) y `active_tab: usize`. `app::run` construye la
lista una vez, al arrancar, analizando **todas** las rutas de la línea de
comandos por adelantado —no de forma perezosa—: es la opción más simple, y
`plan.md` ya registró que analizar bajo demanda es el candidato a revisar
si la medida informal de RNF-01 al cerrar la feature muestra que cuesta un
margen sensible, no antes.

**Decisión — identidad de un documento.** `DocumentTab.path` se guarda
**canonicalizada** (`std::fs::canonicalize`) en vez de como llegó por
línea de comandos. RF-04.1 pide que dos rutas que resuelven al mismo
archivo —una relativa, otra absoluta; una con `..` en medio— cuenten como
el mismo documento. Canonicalizar una vez, al cargar, reduce esa
comparación a `PathBuf == PathBuf`; la alternativa —comparar cadenas, o
resolver a mano en cada punto de comparación— repetiría en dos sitios
(carga inicial y HU-04) una lógica que el sistema operativo ya resuelve
correctamente. Si `canonicalize` falla (la ruta ya no existe entre que
`document::load` la leyó y este paso, una carrera de milisegundos, o un
sistema de archivos que no la soporta), se usa la ruta tal cual: RF-04.1 no
se cumple en ese caso límite, pero el documento se sigue mostrando en vez
de fallar por completo.

**Decisión — componente visual.** La barra de pestañas usa `Tab`/`TabBar`
de `gpui_component::tab` (ya en el proyecto por AD-01), no un `div` propio:
ya resuelve el aspecto de pestaña activa/inactiva (CA-02.2) sin escribir
ese estilo a mano. `TabBar::children(iter)` añade una `Tab::new().label(...)`
por documento, con el nombre de archivo —`Path::file_name()`, no la ruta
completa ni el primer encabezado (RF-06.1)— como único contenido por ahora:
cambiar de pestaña (`on_click`) y cerrarla son HU-02 y HU-03.

**Consecuencias.**

- Sin tests nuevos para `app::run`: toca el sistema de archivos
  (`document::load`, `std::fs::canonicalize`) igual que ya hacía antes de
  esta historia, y sigue el mismo criterio de AD-17/AD-19 —esa clase de
  código se verifica por observación, no con un test aislado—. Los cinco
  criterios de HU-01 se comprobaron lanzando el binario de verdad.
- Un solo `pending_notice` para todos los fallos de una invocación: si más
  de una ruta falla, sus mensajes se unen con salto de línea en un único
  aviso, en vez de encolar varios. Ningún criterio de esta historia exige
  más de un fallo a la vez (CA-01.5 prueba exactamente uno), así que no se
  amplió el mecanismo de aviso para ese caso.

Estado: activa

---

## AD-26 — Un clic muta `AppState`: `Entity<DocumentView>::update`, no una función libre

Fecha: 2026-09-18

**Contexto.** Hasta ahora, lo único que un clic sobre texto interactivo
hacía era leer datos (`app::activate_link(url)`, una función libre sin
acceso a `AppState`: le basta con lanzar el navegador). CA-02.1 exige que
pulsar una pestaña **cambie** `AppState.active_tab` y provoque un nuevo
fotograma — la primera vez que un clic necesita mutar estado, no solo
reaccionar a él.

**Investigación, antes de escribir código.** `DocumentView` implementa
`Render` sobre `&mut self`, con `cx: &mut Context<Self>` — es decir, ya es
un `Entity<DocumentView>` por dentro; lo que falta es una forma de alcanzar
ese entity desde dentro de un cierre de clic que se ejecuta más tarde, no
durante `render()`. `Context<T>::entity(&self) -> Entity<T>` (en
`gpui` 0.2.2, `app/context.rs`) da exactamente eso, y `Entity<T>::update(&self,
cx: &mut C, f: impl FnOnce(&mut T, &mut Context<T>))` (`entity_map.rs`) es
el mecanismo estándar de GPUI para mutar un entity desde fuera de su propio
`render()`.

**Decisión.** `DocumentView::render` captura `cx.entity()` una vez, al
principio, y lo pasa a `render_tab_bar`, que lo mueve dentro del cierre de
`TabBar::on_click`. Al pulsar una pestaña, el cierre llama a
`view.update(cx, |view, cx| { view.state.active_tab = ix; cx.notify(); })`:
cambia el campo y pide explícitamente un nuevo fotograma con `cx.notify()`
—sin esa llamada, GPUI no sabe que el estado cambió y no vuelve a
dibujar—. Es el mismo patrón que usará HU-04 para que un enlace a otro
`.md` añada o active una pestaña, así que se registra aquí, la primera vez
que aparece, no se repite la investigación en esa historia.

**Consecuencias.**

- `render_tab_bar` ya no es una función que solo lee: recibe
  `Entity<DocumentView>` por valor (se clona barato, es un handle) y lo
  mueve dentro de un cierre `'static` — la razón por la que no basta con
  `&App`, que no puede sobrevivir más allá de la llamada a `render()`.
- El cierre comprueba `ix < view.state.tabs.len()` antes de asignar: si la
  lista de pestañas cambiara entre que se pintó la barra y que el clic
  llegó (por ejemplo, si HU-03 permitiera cerrar una pestaña por otra vía
  mientras el clic está en vuelo), un índice fuera de rango no entra en
  pánico, simplemente no hace nada.

Estado: activa

---

## AD-27 — Navegación por enlace: `LinkCtx` enhebrado por `render`; `activate_document_link` separado de `activate_link`

Fecha: 2026-09-18

**Contexto.** RF-14.1 exige que un enlace a otro `.md`, resuelto relativo al
directorio del documento activo, abra una pestaña nueva o active la
existente. `render_text` —la única función que construye el `on_click` de
cualquier enlace— hasta ahora solo necesitaba la URL; RF-14.1 le añade dos
necesidades más: el directorio del documento (para resolver la ruta) y
acceso a `AppState` (para mutarlo, con el patrón de `Entity::update` de
AD-26).

**Decisión — cómo llega el contexto hasta `render_text`.** Un `struct
LinkCtx { doc_dir: Option<PathBuf>, view: Entity<DocumentView> }`,
construido una vez en `DocumentView::render` y pasado por referencia junto
a `cx` por toda la cadena (`render_block` → `render_paragraph`/`render_list`/
`render_table`/`render_centered`/... → `render_text`). Alternativa
descartada: un parámetro `doc_dir: Option<&Path>` y otro `view:
&Entity<DocumentView>` sueltos, en vez de una sola estructura — más ruido
en cada firma de función para la misma cantidad de información, sin
ninguna ventaja real.

**Decisión — dónde vive la lógica de «qué hacer con esta URL».**
`app::activate_document_link(url, doc_dir, view, cx)` es una función
**nueva**, no una ampliación de `app::activate_link`: seguirá habiendo
contextos —ninguno todavía, pero la distinción ya existe en el código— sin
documento activo del que depender, para los que `activate_link` solo abre
el navegador sigue siendo el contrato correcto y más simple. La nueva
función delega en la vieja para `http`/`https`, y añade la resolución de
ruta y la llamada a `open_or_activate_tab` para lo demás.

**Decisión — por qué `open_or_activate_tab` no es el mismo bucle que usa
`run`.** El bucle de carga inicial (RF-01.1, HU-01) nunca debe mover
`active_tab` al toparse con una ruta repetida: CA-01.2 exige que la primera
pestaña sea la activa al arrancar, sin importar en qué posición de la lista
de argumentos se repita una ruta más tarde. RF-14.1, en cambio, exige
exactamente lo contrario: activar siempre la pestaña del documento al que
se navega (CA-04.2). Forzar una sola función para los dos casos habría
significado un parámetro booleano —`¿mover active_tab?`— disfrazando dos
comportamientos distintos de una sola función; se quedan separadas.

**Consecuencias.**

- `DocumentView.state` pasa de privado a `pub(crate)`: `activate_document_link`
  necesita alcanzarlo desde `app`, dentro del cierre de
  `Entity::update` (el mismo patrón de AD-26, aplicado desde fuera de
  `render` por primera vez).
- Encontrado por observación, no por lectura de código: `doc_dir` ya venía
  canonicalizado desde `DocumentTab.path` (AD-25), así que el mensaje de
  RF-17 para un enlace roto mostraba el prefijo `\\?\` de rutas extendidas
  de Windows — inconsistente con el mismo aviso cuando la ruta mala viene
  de la línea de comandos, que no está canonicalizada. Se quita ese
  prefijo antes de construir el mensaje.

Estado: activa
