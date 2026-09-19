# Requisitos — ver-markdown-en-crudo

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Séptima feature del proyecto. Retoma RF-23, aplazado desde el
2026-08-20 (`01-alcance.md`, sección «Aplazado a versiones posteriores»)
por ser barato: el texto original ya está en memoria al cargar el
documento, así que mostrarlo no exige ningún analizador nuevo.

Cada requisito refina uno de `02-requisitos.md`. Un refinamiento **acota**:
aquí no aparece nada que su requisito padre no tuviera ya. Los términos
usados están en el glosario de `00-contexto.md`.

## Funcionales

### RF-23.1 — Alternar entre documento renderizado y en crudo

Refina: RF-23
Cada pestaña debe ofrecer un control para alternar entre ver el documento
renderizado (el comportamiento actual) y verlo como el texto exacto del
archivo `.md`: sin interpretar Markdown, HTML ni front matter. El modo es
propio de cada pestaña — cambiarlo en una no afecta a las demás — y por
defecto una pestaña se abre renderizada.

### RF-23.2 — El crudo se muestra en monoespaciada, línea por línea

Refina: RF-23
El texto en crudo debe mostrarse en tipografía monoespaciada, conservando
cada salto de línea del archivo. Una línea más ancha que la ventana se
conserva entera y se desplaza horizontalmente por sí sola, el mismo
comportamiento que RF-10 ya exige para los bloques de código — no una
regla nueva, la misma aplicada a todo el documento en crudo.

## No funcionales

Ninguno nuevo. RNF-01 no se refina: alternar el modo de una pestaña ya
abierta no es parte del arranque que ese umbral mide.

## Requisitos del sistema que esta feature no cubre

Ninguno: RF-23 se cubre entero entre RF-23.1 y RF-23.2.
