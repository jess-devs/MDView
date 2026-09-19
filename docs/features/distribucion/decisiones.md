# Decisiones — distribucion

> Estado: cerrada, sus tres historias verificadas
> Última actualización: 2026-09-18
> Modo: new-feature

Aquí se escribe toda decisión de esta feature que alguien pueda querer revertir
más adelante, con sus alternativas y sus consecuencias, y se indexa en el mismo
turno en `../../03-arquitectura.md`. La numeración `AD-nn` es continua en todo el
proyecto, y sigue desde AD-28 en `integracion-con-windows/decisiones.md`.

---

## AD-29 — Empaquetado: Inno Setup con registro en `HKCU`, sin forzar la aplicación predeterminada

Fecha: 2026-09-18

**Contexto.** RF-02.1 y RF-21.1 exigen un instalador que registre MDView
como opción de «Abrir con» para `.md` y lo añada al PATH del usuario, ambos
sin privilegios de administrador. `01-alcance.md` ya había verificado que,
desde Windows 8, ningún instalador puede fijar la aplicación predeterminada
por registro: esa clave (`UserChoice`) lleva un hash que solo el diálogo de
Windows puede escribir válidamente.

**Decisión — herramienta: Inno Setup, no WiX ni NSIS.** Se investigaron las
tres antes de escribir el script (`plan.md`). WiX genera `.msi` y exige XML
verboso para tareas simples; NSIS es igual de capaz pero con una sintaxis
más críptica para lo que aquí hace falta. Inno Setup tiene, de fábrica,
ejemplos documentados para exactamente los dos problemas de esta feature:
modificar el PATH del usuario (`Path.iss`) y registrar/desregistrar claves
de forma simétrica con `Flags: uninsdeletekey`. Es la opción con menos
piezas nuevas que aprender para el alcance de una feature de empaquetado,
no de un instalador con actualizaciones automáticas ni firma de código.

**Decisión — asociación sin forzar predeterminada: ProgID propio en
`HKCU\Software\Classes`, añadido a `OpenWithProgIds`.** No se usa
`SetUserFTA` ni ninguna variante que calcule el hash de `UserChoice`:
depende de un algoritmo no documentado que Microsoft introdujo protección
adicional contra en 2025 (`UserChoiceLatest`), y construir sobre eso sería
apostar por algo que la próxima actualización de Windows puede romper. En
su lugar, el instalador solo declara el ProgID y lo añade a la lista de
«Abrir con» de la extensión; la clave `UserChoice` no se toca, y el clic
final para hacerla predeterminada queda, como debe, en manos del usuario.

**Decisión — PATH en `HKCU\Environment`, con difusión de
`WM_SETTINGCHANGE`.** Se escribe el PATH de usuario, no el de máquina
—consistente con no requerir administrador—, comprobando primero que la
carpeta no esté ya en la cadena, para que una reinstalación no la duplique.
Se difunde `WM_SETTINGCHANGE` tras escribir para que Explorer refresque su
entorno; una terminal abierta después de instalar, pero antes de que
Explorer procese ese aviso, podría no ver el cambio — riesgo ya anotado en
`plan.md` y la razón de que CA-01.6 pida una terminal nueva.

**Decisión — versión portable sin Inno Setup.** Es el mismo
`mdview.exe` de `cargo build --release`, comprimido en un `.zip` sin ningún
paso de instalación. No comparte código ni script con el instalador: no
hay nada que desregistrar porque no registra nada.

**Consecuencias.**

- El instalador y el desinstalador quedan atados a Inno Setup: portar el
  proyecto a Linux o macOS no reutiliza nada de esta decisión, cada
  plataforma necesita su propio mecanismo de empaquetado (ya anotado como
  motivo de que `distribucion` sea una feature aparte, en `01-alcance.md`).
- MDView nunca podrá prometer «doble clic funciona nada más instalar» sin
  que el usuario haga un clic adicional en «Abrir con» o en la configuración
  de aplicaciones predeterminadas de Windows. Es una limitación del sistema
  operativo, no del diseño, y ya está documentada como tal en
  `01-alcance.md` y en RF-02.1.
- Sin tests nuevos: todo el mecanismo es registro y PATH de Windows, el
  mismo criterio que AD-17/AD-19/AD-28 ya aplicaban a E/S del sistema
  operativo. Se verifica por observación, en una cuenta sin administrador,
  no con un test que simule el registro.

Estado: activa
