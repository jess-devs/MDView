# Plan — integracion-con-windows

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

## Orden de las features del proyecto

Sin cambios respecto a `enlaces-e-imagenes/plan.md`, quinta y última:

1. `ver-un-documento` — cerrada.
2. `enlaces-e-imagenes` — cerrada.
3. `front-matter-y-html` — cerrada.
4. `varios-documentos-en-pestanas` — cerrada.
5. **`integracion-con-windows`** — esta, solo RF-03.1. RF-02 queda pendiente
   de la feature de distribución (ver `requisitos.md`).

## Orden de las historias

1. **HU-01 — Ruta nueva.** Primera porque construye el mecanismo entero
   —detección de instancia, transporte de la ruta, activación de la
   ventana—; HU-02 y HU-03 solo prueban variaciones de la entrada sobre el
   mismo mecanismo.
2. **HU-02 — Ruta ya abierta.** Reutiliza `open_or_activate_tab` de
   `varios-documentos-en-pestanas` (AD-27): no hay mecanismo nuevo, solo
   confirmar que la segunda invocación pasa por el mismo camino que ya
   activa una pestaña existente en vez de duplicarla.
3. **HU-03 — Sin ninguna ruta.** Última porque es el caso más simple: ni
   pestaña nueva ni activación de una existente, solo traer la ventana al
   frente.

## Dependencias

| Historia | Depende de | Motivo |
| --- | --- | --- |
| HU-01 | Ninguna (de esta feature) | Construye el mecanismo. Reutiliza `open_or_activate_tab` de `varios-documentos-en-pestanas`, ya cerrada. |
| HU-02 | HU-01 | Necesita el mecanismo de transporte que construye HU-01; solo cambia qué ruta se manda. |
| HU-03 | HU-01 | Misma razón; el caso «sin ruta» es un mensaje vacío sobre el mismo transporte. |

## Forma técnica, investigada antes de escribir código

**GPUI no resuelve esto en Windows.** `App::on_reopen` existe, pero su propia
documentación dice que es de macOS (el dock relanza el proceso via
LaunchServices); `App::on_open_urls` es del mismo mundo de integración de
escritorio. Un `.exe` de Win32 corriente —MDView no es un paquete
MSIX/UWP— no tiene ninguna coalescencia de instancias por parte del
sistema operativo: cada invocación es un proceso nuevo e independiente
hasta que el programa mismo decide lo contrario. Comprobado leyendo
`gpui` 0.2.2, no asumido.

**Detección de instancia: Mutex con nombre, vía el crate `windows`.**
`windows 0.61.3` ya es una dependencia transitiva de `gpui` —aparece en
`Cargo.lock` con esa versión exacta—, así que añadirlo como dependencia
directa de MDView con esa misma versión no duplica nada que no estuviera
ya compilado. `CreateMutexW` con un nombre fijo (`"MDView-Instancia-Unica"`)
es el mecanismo estándar de Win32 para esto: si ya existe un mutex con ese
nombre, la llamada devuelve éxito pero `GetLastError` marca
`ERROR_ALREADY_EXISTS`, la señal de que otro proceso lo creó primero. El
handle debe mantenerse abierto mientras el proceso viva —si se cierra, el
sistema libera el mutex y una tercera invocación no vería la segunda como
existente—, así que se filtra deliberadamente (`std::mem::forget`), no un
descuido.

**Transporte de la petición: un archivo bajo `%TEMP%`, sondeado.** Se
consideró `WM_COPYDATA` —el mecanismo clásico de Win32 para pasar datos
entre ventanas— y se descartó: exige localizar la ventana de GPUI con
`FindWindowW` y, sobre todo, interceptar su bucle de mensajes nativo para
recibir el `WM_COPYDATA`, algo que `gpui` no expone como gancho público (se
buscó; no hay nada parecido a «subclasear esta ventana» en su API). Construir
una ventana Win32 propia, sin GPUI, solo para recibir ese mensaje, es más
mecanismo del que este alcance necesita. En su lugar: la segunda invocación
escribe sus rutas —o nada, si no traía ninguna— en un archivo de nombre fijo
bajo el directorio temporal del usuario, escribiendo primero a un archivo
temporal y renombrándolo (`std::fs::rename`, atómico en el mismo volumen),
para que el proceso que ya está en marcha nunca lea una escritura a medias.
Ese proceso sondea ese archivo con un temporizador de `BackgroundExecutor`
(`cx.background_executor().timer(...)`, ya usado en el proyecto — ver
`app::record_timing`) dentro de la misma tarea asíncrona que abre la
ventana; si lo encuentra, lo borra y reacciona.

**Reaccionar a una petición: el mismo patrón de `Entity::update` que
AD-26/AD-27, más `WindowHandle::activate_window`.** `cx.open_window(...)`
ya devuelve un `WindowHandle<Root>`; capturado junto al `Entity<DocumentView>`
que ya se construye antes de envolverlo en `Root`, la tarea de sondeo
puede, en la misma actualización, llamar a `window.activate_window()`
—método público de `Window`, confirmado en el código de `gpui`— y mutar
`AppState` con `open_or_activate_tab` (RF-03.1 con ruta) o no hacer nada
más (RF-03.1 sin ruta), exactamente el patrón de mutación por clic que ya
existe en el proyecto, aplicado ahora a un evento que no es un clic.

**Consecuencia para HU-03 (sin ruta).** La petición vacía todavía necesita
mandarse —de lo contrario la segunda invocación no tiene nada que decirle a
la primera y su ventana nunca se activaría—, así que «sin ruta» se
serializa como un archivo vacío existente, distinguible de «no hay
petición» (el archivo no existe).

## Riesgos

- **Latencia del sondeo.** Un temporizador de ~200 ms significa que, en el
  peor caso, pasar el primer plano y la pestaña nueva tarda hasta ese
  margen desde que la segunda invocación termina. Ningún criterio de
  aceptación fija un tiempo máximo; si en la verificación por observación
  se nota una espera molesta, el intervalo es lo primero que se ajusta, no
  el mecanismo entero.
- **El archivo de petición sobrevive a un cierre inesperado.** Si el
  proceso que debía leerlo termina antes de hacerlo (cierre forzado,
  cuelgue), el archivo queda huérfano y una futura primera invocación lo
  encontraría al arrancar. Se borra también al iniciar (antes de crear el
  mutex), para no procesar una petición que no era para esta ejecución.
- **Ninguna dependencia nueva sin decisión registrada.** `windows 0.61.3`
  se justifica arriba: ya está compilada, es la vía estándar de Win32 para
  esto, y ninguna alternativa evaluada es más simple. Se registra como
  decisión al implementar HU-01, no aquí.

## Documentos de prueba

Dos documentos pequeños, reutilizando el patrón ya establecido
(`pruebas/instancia-doc-a.md`, `-b.md`), para las tres historias.
