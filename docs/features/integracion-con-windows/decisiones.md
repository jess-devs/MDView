# Decisiones — integracion-con-windows

> Estado: cerrada, sus tres historias verificadas
> Última actualización: 2026-09-18
> Modo: new-feature

Aquí se escribe toda decisión de esta feature que alguien pueda querer revertir
más adelante, con sus alternativas y sus consecuencias, y se indexa en el mismo
turno en `../../03-arquitectura.md`. La numeración `AD-nn` es continua en todo el
proyecto, y sigue desde AD-27 en `varios-documentos-en-pestanas/decisiones.md`.

---

## AD-28 — Instancia única: Mutex de `windows` para detectar, archivo sondeado para transportar

Fecha: 2026-09-18

**Contexto.** RF-03.1 exige que una segunda invocación de MDView, con un
proceso ya en marcha, se atienda desde ese proceso: la ruta pedida se
añade como pestaña —o activa la existente, RF-04.1— y la ventana pasa a
primer plano, sin abrir una ventana nueva. `plan.md` ya investigó, antes
de escribir código, que ni `App::on_reopen` ni `App::on_open_urls` de
`gpui` resuelven esto en Windows: son de integración de escritorio de
macOS, según su propia documentación, y un `.exe` de Win32 corriente no
tiene coalescencia de instancias por parte del sistema operativo.

**Decisión — detección: un Mutex con nombre, vía el crate `windows`.**
`windows = "0.61.3"`, la misma versión exacta que ya trae `gpui` como
dependencia transitiva —confirmado en `Cargo.lock` antes de añadirlo—, así
que no duplica ninguna compilación. `CreateMutexW` con un nombre fijo es
el mecanismo estándar de Win32: si el nombre ya existe, `GetLastError`
devuelve `ERROR_ALREADY_EXISTS`. El `HANDLE` que devuelve se deja caer sin
cerrarlo a propósito —confirmado leyendo su definición en el crate: no
tiene `impl Drop`, así que no hace falta ni `std::mem::forget`, dejarlo
salir de alcance ya no cierra nada—; cerrarlo liberaría el mutex y una
tercera invocación no vería la segunda como una instancia en marcha.

**Decisión — transporte: un archivo bajo `%TEMP%`, sondeado, no
`WM_COPYDATA`.** Se investigó el mecanismo clásico de Win32 para pasar
datos entre ventanas —`FindWindowW` más `SendMessageW(WM_COPYDATA)`— y se
descartó: exige interceptar el bucle de mensajes nativo de la ventana que
`gpui` ya gestiona, y `gpui` no expone ningún gancho para subclasearla
—se buscó en su código, no hay nada parecido—. Construir una ventana Win32
propia, ajena a `gpui`, solo para recibir ese mensaje, es más mecanismo
del que este alcance necesita. En su lugar, `app::send_instance_request`
escribe las rutas de la segunda invocación en un archivo de nombre fijo,
primero con un nombre temporal y luego renombrado (`std::fs::rename`,
atómico en el mismo volumen) para que nunca se lea a medio escribir; el
proceso ya en marcha lo sondea con `cx.background_executor().timer(...)`
—el mismo mecanismo que ya usa `record_timing`, aplicado aquí en un bucle
en vez de una vez— dentro de la misma tarea asíncrona que abrió la
ventana.

**Decisión — cómo llega la petición hasta `AppState`.** `cx.open_window`
devuelve un `WindowHandle<Root>`; capturado junto a un `Entity<DocumentView>`
construido *antes* de envolverlo en `Root` —`Root` solo expone su vista
hija como `AnyView` opaco, no hay forma de recuperar el tipo concreto
después—, el bucle de sondeo usa `WindowHandle::update` para llamar a
`Window::activate_window()` (confirmado como método público de `gpui`) y,
en la misma actualización, `Entity<DocumentView>::update` para llamar a
`open_or_activate_tab` una vez por ruta pedida — la misma función que
RF-14.1 ya usa (AD-27), y el mismo patrón de mutación por `Entity::update`
de AD-26.

**Consecuencias.**

- Latencia de hasta `INSTANCE_POLL_INTERVAL` (200 ms) entre que la segunda
  invocación termina y la primera reacciona. Ningún criterio fija un
  máximo; es el primer valor a ajustar si se nota, no el mecanismo
  (anotado ya en `plan.md`).
- Un archivo de petición huérfano —el proceso que debía leerlo terminó
  antes— se borra al arrancar, antes de comprobar el Mutex, para que una
  futura primera invocación no la procese por error.
- Sin tests nuevos: todo el mecanismo es E/S real del sistema operativo
  (Mutex, archivo, ventana), el mismo criterio que AD-17/AD-19 ya
  aplicaban a `app::activate_link` y a la carga de rutas. Verificado por
  observación con dos procesos reales, no con un test que simule uno.

Estado: activa
