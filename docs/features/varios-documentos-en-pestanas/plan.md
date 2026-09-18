# Plan — varios-documentos-en-pestanas

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

## Orden de las features del proyecto

Sin cambios respecto a `enlaces-e-imagenes/plan.md`:

1. `ver-un-documento` — cerrada.
2. `enlaces-e-imagenes` — cerrada.
3. `front-matter-y-html` — cerrada.
4. **`varios-documentos-en-pestanas`** — esta.
5. `integracion-con-windows` — la última; necesita que las pestañas existan.

## Orden de las historias

1. **HU-01 — Abrir varios documentos a la vez.** Primera porque es el cambio
   de fondo del que dependen las otras tres: `AppState` deja de tener un
   documento y pasa a tener una lista. Sin esto no hay nada que cambiar,
   cerrar, ni a lo que navegar.
2. **HU-02 — Cambiar de pestaña.** Segunda porque es la operación más simple
   sobre la lista que HU-01 ya construyó: mover el índice de la pestaña
   activa, sin tocar la lista en sí.
3. **HU-03 — Cerrar una pestaña.** Tercera porque sí toca la lista —quitar un
   elemento— y tiene un caso especial (RF-07.1) que HU-02 no tenía.
4. **HU-04 — Navegar a otro documento desde un enlace.** Última porque, a
   diferencia de las tres anteriores —todas manipulan `AppState` desde fuera,
   por una acción del lector sobre la barra de pestañas—, esta necesita que
   el propio documento (un clic dentro de `InteractiveText`) mute
   `AppState` para añadir o activar una pestaña. Eso es una forma de
   comunicación entre `render` y `app` que no existe todavía en el proyecto
   —hasta ahora `render_text` solo llamaba a `app::activate_link`, una
   función libre sin acceso al estado—, así que conviene investigarla con
   las otras tres historias ya construidas y probadas, no como la primera
   pieza de la feature.

## Dependencias

| Historia | Depende de | Motivo |
| --- | --- | --- |
| HU-01 | Ninguna | Es el cambio de `AppState` del que dependen las demás. |
| HU-02 | HU-01 | Necesita una lista de pestañas para poder cambiar entre ellas. |
| HU-03 | HU-01 | Misma razón; no depende de HU-02. |
| HU-04 | HU-01, HU-03 | Necesita poder añadir una pestaña (HU-01) y, para CA-04.2, que ya exista el concepto de activar una pestaña existente en vez de crear otra — que HU-02 ya prueba para el caso manual, y HU-04 reutiliza para el automático. |

Con el límite de trabajo en curso de 1, las historias se hacen de una en una.

## Forma técnica

### `AppState`: de un documento a una lista

`AppState` pasa de `blocks: Vec<markdown::Block>` a:

```rust
pub struct DocumentTab {
    pub path: PathBuf,   // canonicalizada (std::fs::canonicalize), para RF-04.1
    pub blocks: Vec<markdown::Block>,
}

pub struct AppState {
    pub tabs: Vec<DocumentTab>,
    pub active_tab: usize,
    // pending_notice, no_path_given, start, timing_path: sin cambios de tipo
}
```

`path` se guarda **canonicalizada** —`std::fs::canonicalize`, que resuelve
`.`/`..` y symlinks— para que RF-04.1 («dos rutas que resuelven al mismo
archivo cuentan como la misma») se reduzca a comparar dos `PathBuf` por
igualdad, en vez de comparar cadenas o reimplementar la resolución a mano.

### Componente de pestañas: `Tab`/`TabBar` de `gpui-component`

`gpui-component` 0.5.1 trae un módulo `tab` (`Tab`, `TabBar`) que ya resuelve
el aspecto visual de una barra de pestañas —variantes, tamaño, pestaña
activa distinguible (CA-02.2)—, coherente con AD-01 (adoptar
`gpui-component` como capa de componentes). Se investigó su API antes de
escribir código: `TabBar::new(id).selected_index(ix).on_click(closure)` con
hijos `Tab::new().label(...)`; el cierre de una pestaña concreta no tiene un
método dedicado (no hay `closable()`/`on_close()`), así que HU-03 construye
el botón de cerrar a mano con `Tab::suffix(...)`. El diseño exacto —incluida
la comprobación de que un clic en el sufijo no dispara también el `on_click`
de la pestaña entera— se registra como decisión al implementar HU-03, no
aquí: es una comprobación de comportamiento, no de lectura de código.

### El caso pendiente de investigar: HU-04

Hasta ahora, `render::render_text` conecta el clic de un enlace a
`app::activate_link(url)`, una función libre sin acceso a `AppState`: le
basta con lanzar el navegador. RF-14.1 exige que ese mismo clic, cuando el
destino es un `.md`, **modifique** `AppState` —añada una pestaña, o cambie
cuál está activa— y provoque un nuevo fotograma. Eso es un patrón que el
proyecto no ha necesitado hasta ahora (todo lo demás que muta `AppState` lo
hace `app::run` una vez, al arrancar) y hay que comprobar contra la API real
de GPUI —lo más probable es que la maneje `Entity<DocumentView>::update` o
un `cx.listener` capturado en `DocumentView::render`, no `render_text` como
función libre— antes de diseñar la firma final. Se investiga y se registra
al implementar HU-04, siguiendo el mismo método que `front-matter-y-html`
usó para el modelo de eventos de `pulldown-cmark` antes de escribir
`html.rs`.

## Riesgos

- **Qué pestaña queda activa al cerrar la activa (CA-03.2).** No hay ninguna
  restricción del usuario ni de `01-alcance.md` sobre cuál. Por defecto se
  activa la que queda en el mismo índice de la lista tras quitar la cerrada
  —la que estaba inmediatamente a su derecha, o la nueva última si se cerró
  la última—, el mismo criterio que usan la mayoría de navegadores y
  editores. Se registra como decisión al implementar HU-03, no se deja
  implícito en el código.
- **`MDVIEW_TIMING` (AD-07) con varias pestañas.** La instrumentación mide
  hasta el primer fotograma que contiene «el documento» visible; con varias
  pestañas, eso sigue siendo la primera —la activa al arrancar—, no las
  demás, que ni siquiera se han analizado todavía si se decide analizarlas
  de forma perezosa (ver el punto siguiente). No hace falta cambiar
  `record_timing`: sigue disparándose una vez, en el primer fotograma,
  exactamente igual que con un solo documento.
- **¿Se analizan las pestañas no activas al arrancar, o de forma
  perezosa?** RF-01.1 no lo dice. Analizar las tres (u N) rutas al arrancar
  es más simple y es lo que se construye por defecto en HU-01; si la medida
  informal de RNF-01 al cerrar la feature muestra que N documentos grandes
  cuestan un margen sensible sobre el umbral de 1,2 s, la alternativa
  —analizar solo la pestaña activa, y las demás la primera vez que se
  activan— es el candidato a registrar entonces, no antes: no hay ningún
  documento de prueba de esta feature lo bastante grande para que el riesgo
  sea hoy más que teórico.
- **Ninguna dependencia nueva entra sin decisión registrada.** Mismo
  criterio que las features anteriores. `gpui-component::tab` ya es parte de
  la dependencia existente; nada de esta feature debería necesitar más.

## Documentos de prueba

Tres documentos pequeños e independientes para HU-01/HU-02/HU-03
(`pruebas/pestanas-doc-a.md`, `-b.md`, `-c.md`, cada uno con contenido
distinguible a simple vista), más un documento con enlaces para HU-04. Todos
en `pruebas/`, no versionados, siguiendo la convención ya establecida.
