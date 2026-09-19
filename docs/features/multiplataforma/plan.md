# Plan — multiplataforma

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

## Forma técnica

**Entorno de prueba: WSL2 + Ubuntu + WSLg, no una máquina Linux real.**
Se instaló porque era lo único disponible en esta sesión (Windows sin
máquina Linux ni Mac). Sirve para probar que el código compila y para un
primer intento de ejecución, pero **no sustituye** a la máquina real que
`01-alcance.md` exige para dar un criterio por `pasa` — WSLg proyecta la
ventana Linux hacia Windows a través de un cliente de Escritorio remoto
interno (`msrdc.exe`), una capa más que una pantalla Linux real no tiene.

**El primer intento de compilar reveló un defecto real, no relacionado
con Linux en sí.** `Cargo.toml` declaraba
`windows = { version = "0.61.3", ... }` en `[dependencies]` sin ningún
`target.cfg`, así que un `cargo build` en Linux intentaba compilarla
igual. Falló dentro de una dependencia transitiva de esa biblioteca
(`windows-future`, con errores de símbolos específicos de Windows que no
existen fuera de esa plataforma) — la prueba de que nadie había
compilado nunca este proyecto fuera de Windows, ni con `--target`, desde
que AD-28 añadió esa dependencia. Corregido moviendo `windows` a
`[target.'cfg(windows)'.dependencies]` y añadiendo `#[cfg(windows)]` a
`is_another_instance_running` y a sus imports en `src/app.rs`, con una
versión `#[cfg(not(windows))]` que devuelve `false` — RF-03.1 queda sin
mecanismo de instancia única en Linux por ahora, un hueco explícito, no
una promesa rota, porque ningún requisito de esta feature pide RF-03.1
en Linux todavía.

**Tras el arreglo, el proyecto entero compila en Linux** (`cargo build`,
`cargo test`: 31/31, las mismas pruebas que en Windows, porque son de
lógica pura sin E/S — AD-17). Ninguna otra dependencia necesitó tocarse:
`gpui = "=0.2.2"` ya trae activadas por defecto las *features* `wayland`
y `x11` (confirmado leyendo su `Cargo.toml`), así que MDView no necesita
declarar nada especial para Linux más allá de lo que ya tenía.

**Al ejecutarlo, dos hallazgos, ninguno resuelto del todo:**

- Con `WAYLAND_DISPLAY` puesto (WSLg lo pone siempre), `gpui` elige su
  backend Wayland y falla al arrancar:
  `UnsupportedVersion` al pedir una versión de `wl_compositor` que el
  compositor Weston de WSLg no ofrece (`gpui-0.2.2/src/platform/linux/
  wayland/client.rs`). No es un defecto de MDView: es una incompatibilidad
  entre `gpui` y la versión de Weston que trae WSLg.
- Quitando `WAYLAND_DISPLAY` para forzar el backend X11
  (`gpui::platform::guess_compositor` cae a X11 si `DISPLAY` está puesto
  y `WAYLAND_DISPLAY` no), el proceso arranca sin *panic* y sin errores en
  su log. La ventana que WSLg proyecta hacia Windows, sin embargo,
  apareció con el título «`[WARN:COPY MODE] top-level window (Ubuntu)`»
  —un aviso del propio WSLg de que está usando una ruta de
  renderizado degradada, no la acelerada por GPU—, y no se pudo
  confirmar que el contenido se viera correctamente.

**Qué no se investiga aquí:** por qué WSLg específicamente entra en modo
degradado (aceleración por GPU vía D3D12 dentro de la VM, `/dev/dxg`,
confirmado presente) es un problema del entorno de prueba, no del
proyecto — no vale la pena perseguirlo cuando la meta real es una máquina
Linux de verdad, donde este problema en concreto no existiría.

## Riesgos

- **El hueco de RF-03.1 en Linux** queda sin resolver hasta que exista una
  feature que decida su mecanismo (probablemente un socket de dominio
  Unix o un *lock file*, ninguno de los dos investigado todavía).
- **Ninguna verificación visual es de fiar hasta tener una máquina real.**
  Cualquier intento futuro en WSLg debe tratarse como esta feature lo
  hizo: información útil, nunca evidencia de un criterio de aceptación.
