# Requisitos — multiplataforma

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Novena feature del proyecto. Reconsidera en parte la exclusión de Linux y
macOS de `01-alcance.md`: cubre RNF-04, solo Linux, y solo su compilación —
no la verificación en pantalla, que exige una máquina Linux real. macOS no
se toca aquí: sigue sin ninguna máquina Apple para compilarlo ni
verificarlo, y `00-contexto.md` ya lo señalaba como bloqueante.

## Decisión de alcance que gobierna toda la feature

**No hay máquina Linux real disponible.** Este entorno es Windows. Al
abrirse esta feature se instaló WSL con una distribución Ubuntu, pero una
VM de WSL con WSLg no es la máquina real que `01-alcance.md` exige para
verificar por observación — es la mejor aproximación disponible, no un
sustituto. Por eso esta feature se acota a dejar el código listo para
compilar en Linux, con la verificación en pantalla registrada como
`bloqueada`, nunca como `pasa`, hasta que exista una máquina Linux real.
Construir esto de otra forma —dando por buena una prueba en WSLg como si
fuera la cosa real— rompería la regla más antigua del proyecto.

## Funcionales

Ninguno nuevo: esta feature no añade comportamiento visible al usuario,
prepara el terreno para que exista en Linux el mismo comportamiento que
ya existe en Windows.

## No funcionales

### RNF-04.1 — El código compila en Linux sin código específico de Windows fuera de su límite

Refina: RNF-04
Todo uso de la biblioteca `windows` (el mecanismo de instancia única,
RF-03.1/AD-28) debe quedar limitado a compilaciones para Windows —ni la
dependencia en `Cargo.toml` ni el código que la usa en `src/app.rs` deben
intentar compilarse en Linux. Un `cargo build` en Linux debe terminar sin
errores.

## Requisitos del sistema que esta feature no cubre

- **Verificación visual en Linux** (que la ventana se muestre y se pueda
  usar). Necesita una máquina Linux real; no existe en este entorno. Se
  registrará `bloqueado`, no `pasa`, siguiendo la regla de
  `docs/04-calidad.md`.
- **macOS por completo.** Sin máquina Apple, ni compilar ni verificar es
  posible. No se abre ningún requisito para macOS en esta feature.
