# Decisiones — varios-documentos-en-pestanas

> Estado: borrador
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
