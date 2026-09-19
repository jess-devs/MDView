# Decisiones — multiplataforma

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Aquí se escribe toda decisión de esta feature que alguien pueda querer revertir
más adelante, con sus alternativas y sus consecuencias, y se indexa en el mismo
turno en `../../03-arquitectura.md`. La numeración `AD-nn` es continua en todo el
proyecto, y sigue desde AD-31 en `edicion-con-confirmacion/decisiones.md`.

---

## AD-32 — La dependencia `windows` se limita a `cfg(windows)`, con un hueco explícito para RF-03.1 en Linux

Fecha: 2026-09-18

**Contexto.** `Cargo.toml` declaraba `windows = "0.61.3"` (AD-28) en
`[dependencies]` sin ningún `target.cfg`, así que un `cargo build` en
cualquier plataforma la compilaba igual. Nadie lo había notado porque
nadie había compilado el proyecto fuera de Windows hasta que se probó en
Linux para esta feature — y falló, con errores dentro de una dependencia
transitiva de `windows` (`windows-future`) que solo tiene sentido en
Windows.

**Decisión — mover `windows` a `[target.'cfg(windows)'.dependencies]`.**
Es la forma estándar de Cargo para una dependencia de una sola
plataforma; ninguna alternativa considerada (una *feature* de Cargo
propia para activarla, por ejemplo) sería tan directa ni tan a prueba de
que alguien la use por error en código compartido, porque el propio
compilador ni siquiera ve el símbolo fuera de Windows.

**Decisión — `is_another_instance_running` (RF-03.1, AD-28) queda tras
`#[cfg(windows)]`, con una versión `#[cfg(not(windows))]` que siempre
devuelve `false`.** No se intenta un mecanismo de instancia única para
Linux en esta feature: ninguna historia lo pide, y adivinar uno sin poder
verificarlo en una máquina real repetiría exactamente el error que esta
feature existe para evitar. El hueco queda documentado, no oculto: en
Linux, cada invocación de MDView abre su propia ventana, sin RF-03.1,
hasta que una feature futura decida cómo construirlo ahí.

**Decisión — WSLg no cuenta como la máquina Linux real que
`01-alcance.md` exige.** Se investigó y se documentó en `plan.md` lo que
WSLg permitió observar (arranca con el backend X11, el backend Wayland
falla por una incompatibilidad de protocolo con Weston, la ventana
proyectada queda en un modo de renderizado degradado), pero ningún
resultado de ahí se registra como `pasa` en `04-calidad.md`. Es
información para cuando exista una máquina real, no una sustituta de
ella.

**Consecuencias.**

- El proyecto ahora compila en dos plataformas (`cargo build` verificado
  en Windows y en una VM Linux), pero solo se **ejecuta verificadamente**
  en una: Windows. Eso no cambia hasta tener una máquina Linux real.
- Sin tests nuevos: la separación por `cfg` no tiene comportamiento en
  tiempo de ejecución que probar con `#[cfg(test)]`, y la verificación
  real (que compile) ya la hace `cargo build` en CI o a mano, no un test.

Estado: activa
