# Plan — distribucion

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

## Forma técnica

**Herramienta de empaquetado: Inno Setup.** Se investigaron las tres
opciones habituales en Windows para construir un instalador desde un
binario Rust: WiX (genera `.msi`, requiere XML verboso y una cadena de
herramientas más pesada), NSIS (script propio, maduro pero con una sintaxis
más críptica que Inno Setup para tareas sencillas) e Inno Setup (script
declarativo en secciones `[Files]`/`[Registry]`/`[Icons]`, con soporte
incorporado —de fábrica, sin plugin— para modificar el PATH del usuario vía
su página de ejemplos `Path.iss`). Para un instalador de un único `.exe`
sin dependencias adicionales, Inno Setup es la opción con menos piezas que
aprender y mantener. **No está instalado en la máquina de desarrollo**;
instalarlo es una acción que se le pidió permiso al usuario antes de
ejecutarla, porque implica descargar y ejecutar un instalador de terceros
en su equipo — no algo que esta feature pueda decidir por sí sola.

**Asociación de `.md` sin forzar predeterminada (RF-02.1).** El mecanismo
verificado (`01-alcance.md`) es el que Windows documenta para que una
aplicación aparezca en «Abrir con» sin apropiarse de la extensión:

```
HKCU\Software\Classes\MDView.md\shell\open\command\(Default) = "<ruta>\mdview.exe" "%1"
HKCU\Software\Classes\.md\OpenWithProgIds\MDView.md = (vacío)
```

La primera clave declara un ProgID propio y cómo abrirlo; la segunda añade
ese ProgID a la lista de «Abrir con» de la extensión `.md`, sin tocar la
clave `UserChoice` que decide cuál es la predeterminada — esa la escribe
solo el diálogo de Windows, con el clic del usuario. Todo en `HKCU`, no en
`HKLM`: no hace falta administrador (RF-02.1 lo exige explícitamente), y el
desinstalador puede borrar limpiamente ambas claves con
`Flags: uninsdeletekey` de Inno Setup.

**Integración en el PATH (RF-21.1).** Se escribe en
`HKCU\Environment\Path`, no en el PATH de máquina —de nuevo, sin
administrador—, añadiendo la carpeta de instalación si no está ya presente
(comprobación de subcadena antes de concatenar, para que reinstalar no
duplique la entrada). Tras escribirla, se difunde `WM_SETTINGCHANGE` —el
snippet estándar de la documentación de Inno Setup para esto— para que
Explorer refresque su propio entorno; sin ese aviso, una terminal abierta
*después* de instalar podría heredar el entorno de un Explorer que todavía
no se enteró del cambio. CA-01.6 exige una terminal nueva precisamente por
esto: es el caso que si el aviso fallara, fallaría también.

**Desinstalación (HU-02).** Inno Setup genera el desinstalador solo con
declarar las claves de registro con `Flags: uninsdeletekey`/
`uninsdeletevalue`; retirar la entrada del PATH exige un paso de código
explícito en la sección `[UninstallRun]` o `[Code]`, simétrico al de
instalar: leer `HKCU\Environment\Path`, quitar la subcadena de la carpeta
de instalación, volver a escribir.

**Versión portable (RF-22.1, HU-03).** No usa Inno Setup: es
`cargo build --release` seguido de comprimir `mdview.exe` en un `.zip`, sin
ningún paso de registro ni de PATH. Es, literalmente, el mismo ejecutable
que produce el instalador — la única diferencia es qué lo envuelve. Se
verifica con CA-03.2 que ejecutarlo no deja ningún rastro en el registro ni
en el PATH, precisamente porque no pasa por ningún script de instalación.

**Qué no se investiga aquí:** RF-02 dejó dicho, y esta feature no lo
reabre, que forzar la aplicación predeterminada no es posible sin pasar por
el diálogo de Windows (pregunta cerrada de `01-alcance.md`, verificada el
2026-09-18). No se intenta ningún atajo tipo `SetUserFTA`: depende de un
algoritmo no documentado que Microsoft ya empezó a romper con
`UserChoiceLatest`, y construir sobre él sería la clase de solución fresca
en el próximo *patch* de Windows que el proyecto prefiere evitar.

## Riesgos

- **Inno Setup no está instalado.** Bloquea la construcción del instalador
  hasta que el usuario autorice instalarlo (o lo instale él mismo). No
  bloquea RF-22.1 (portable), que no lo necesita.
- **`WM_SETTINGCHANGE` no es instantáneo en todos los procesos.** Algunos
  programas de terceros cachean su propio entorno y no lo recogen sin
  reiniciar sesión. Por eso CA-01.6 pide una terminal *nueva* del propio
  Windows (`cmd`/PowerShell), no cualquier programa que ya estuviera
  abierto.
- **Cuenta de prueba sin administrador.** Todas las historias de esta
  feature se verifican en una cuenta estándar a propósito, porque RF-02.1 y
  RF-21.1 exigen explícitamente que no haga falta elevación; si alguna
  parte del instalador resulta necesitarla, es una señal de que algo se
  escribió en `HKLM` por error, no una excepción a aceptar.
