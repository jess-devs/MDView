# Requisitos — distribucion

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Sexta feature del proyecto, y la que faltaba para que RF-02 pudiera
retomarse: `integracion-con-windows` lo dejó pendiente el 2026-09-18 porque
el mecanismo de asociación de `.md` depende de si existe un instalador
(`01-alcance.md`, pregunta cerrada de esa misma fecha). Cubre RF-02, RF-21 y
RF-22.

Cada requisito refina uno de `02-requisitos.md`. Un refinamiento **acota**:
aquí no aparece nada que su requisito padre no tuviera ya. Los términos
usados están en el glosario de `00-contexto.md`.

## Funcionales

### RF-02.1 — Asociación de `.md` como opción del Explorador

Refina: RF-02
El instalador debe registrar MDView, en el ámbito del usuario actual
(`HKCU`, sin necesitar privilegios de administrador), como una aplicación
capaz de abrir archivos `.md`, de modo que aparezca en el diálogo «Abrir
con» del Explorador. El instalador no debe intentar dejarla como
predeterminada: desde Windows 8, esa elección está protegida por un hash
que solo el propio diálogo de Windows puede escribir (verificado, ver
`01-alcance.md`), y cualquier intento de forzarla por registro queda
invalidado por el sistema o roto por la próxima actualización. La
aplicación predeterminada la fija el usuario, con su propio clic, en el
diálogo de Windows — MDView solo se ofrece como opción disponible.

### RF-21.1 — Instalador con integración en el PATH

Refina: RF-21
El instalador debe copiar `mdview.exe` a una carpeta fija bajo el perfil
del usuario y añadir esa carpeta al PATH de usuario (no al del sistema),
de modo que abrir una terminal nueva y escribir `mdview archivo.md` lo
ejecute sin indicar la ruta completa. Debe ofrecer también una opción de
desinstalación que retire el ejecutable, la asociación de `.md` que él
mismo registró y la entrada añadida al PATH.

### RF-22.1 — Versión portable

Refina: RF-22
Debe existir un empaquetado alternativo al instalador: un `.zip` que
contenga `mdview.exe` listo para ejecutarse tal cual, sin instalación, sin
tocar el registro ni el PATH. Extraerlo en cualquier carpeta —incluida una
memoria USB— y ejecutar el `.exe` debe abrir MDView con el mismo
comportamiento que la versión instalada.

## No funcionales

Ninguno nuevo. RNF-01, RNF-02 y RNF-03 ya cubren el ejecutable que esta
feature empaqueta; empaquetarlo no cambia su arranque, su tráfico de red ni
su plataforma.

## Requisitos del sistema que esta feature no cubre

Ninguno: con RF-02.1, RF-21.1 y RF-22.1, los tres requisitos que le
correspondían a `distribucion` quedan cubiertos.
