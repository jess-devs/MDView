# Historias — multiplataforma

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Una historia, un único tipo de usuario: el lector de documentación técnica
de `00-contexto.md`, en un futuro donde también usa Linux — hoy sin
máquina real para comprobarlo.

---

## HU-01 — El código compila en Linux

Cubre: RNF-04.1

**Historia.** Como responsable del proyecto, quiero que el código de
MDView compile en Linux sin código específico de Windows filtrándose
donde no debe, para que portarlo más adelante sea encontrar y resolver
diferencias de comportamiento, no pelear con el propio build.

**Criterios de aceptación.**

1. CA-01.1 — `cargo build` termina sin errores en una máquina/VM Linux
   x86_64.
2. CA-01.2 — `cargo test` pasa igual que en Windows (las pruebas son de
   lógica pura, sin E/S de sistema operativo — AD-17).
3. CA-01.3 — La aplicación se ejecuta en una pantalla Linux real (X11 o
   Wayland) y muestra su ventana correctamente. **Bloqueado**: no existe
   una máquina Linux real en este entorno. Se intentó en una VM de WSL
   con WSLg, sin poder confirmar que la ventana se vea correctamente
   —el compositor de WSLg reportó un estado degradado
   («`[WARN:COPY MODE]`») al proyectar la ventana hacia Windows—, y una
   VM de WSL no es la máquina real que este criterio pide de todos
   modos. Queda `bloqueado` hasta que exista una.

**Estado.** parcialmente verificada: CA-01.1 y CA-01.2 pasan; CA-01.3
bloqueado. La historia no se cierra mientras quede un criterio sin
verificar (`04-calidad.md`), pero tampoco hay más trabajo posible en este
entorno — queda abierta a la espera de una máquina Linux real, no en
progreso activo.

---

## Cobertura

| Requisito | Historia que lo cubre |
| --- | --- |
| RNF-04.1 | HU-01 |
