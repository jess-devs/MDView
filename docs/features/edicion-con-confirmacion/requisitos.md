# Requisitos — edicion-con-confirmacion

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Octava feature del proyecto. Retoma RF-24, aplazado desde el 2026-08-20
(`01-alcance.md`) por ser, en sus propias palabras, «plausiblemente más
trabajo que toda la primera versión junta». Se acota aquí explícitamente
a una primera entrega concreta, no al editor completo que esa nota
imaginaba — lo que queda fuera se dice, no se omite en silencio.

Cada requisito refina uno de `02-requisitos.md`. Un refinamiento **acota**:
aquí no aparece nada que su requisito padre no tuviera ya. Los términos
usados están en el glosario de `00-contexto.md`.

## Decisión de alcance que gobierna toda la feature

**Editar significa editar el texto Markdown crudo del archivo, no el
árbol renderizado.** No hay edición «WYSIWYG» sobre encabezados, tablas o
negritas ya formateadas: habilitar la edición muestra el mismo texto que
ya muestra RF-23 (`ver-markdown-en-crudo`), pero editable, con el cursor,
la selección y deshacer/rehacer que trae de fábrica el componente de
entrada de `gpui-component` (`gpui_component::input`, confirmado leyendo
su código: multilínea, historial de cambios con deshacer/rehacer,
selección — nada de esto se construye a mano). Guardar escribe ese texto
tal cual en el archivo. Es la misma decisión que `01-alcance.md` ya
apuntaba en su nota de coste condicionado: adoptar `gpui-component`
(AD-01) abarata mucho esta función si se apoya en su componente, y esta
feature apoya en él.

## Funcionales

### RF-24.1 — Habilitar la edición explícitamente, por pestaña

Refina: RF-24
Cada pestaña debe ofrecer un control para pasar a modo edición, separado
del control de RF-23.1. Por defecto una pestaña abre de solo lectura
(RF-24 ya lo exige); pasar a edición muestra el texto crudo del
documento, editable.

### RF-24.2 — Escribir con cursor, selección y deshacer/rehacer

Refina: RF-24
En modo edición, el usuario debe poder escribir con cursor y selección
normales, y deshacer/rehacer sus cambios. Heredado del componente de
entrada de `gpui-component`: esta historia verifica que funciona, no lo
reimplementa.

### RF-24.3 — Guardar los cambios en el archivo

Refina: RF-24
Debe existir una acción explícita para guardar (un botón, y el atajo
habitual del sistema para guardar) que escriba el texto editado en el
archivo original y vuelva a mostrar la pestaña renderizada, reflejando
los cambios.

### RF-24.4 — Indicar que hay cambios sin guardar

Refina: RF-24
Mientras el texto editado difiera del que había al entrar en modo
edición o al guardar por última vez, la pestaña debe indicarlo
visualmente.

### RF-24.5 — Confirmar antes de perder cambios sin guardar (revisa RF-07)

Refina: RF-24, RF-07
Cerrar una pestaña con cambios sin guardar —incluida la última, que hoy
termina la aplicación sin más (RF-07)— debe preguntar antes: guardar,
descartar, o cancelar. `01-alcance.md` ya avisaba que RF-07 necesitaría
revisarse cuando la edición entrara; esto es esa revisión.

## No funcionales

Ninguno nuevo.

## Explícitamente fuera de esta primera entrega

No aplazado en silencio: se nombra aquí para que nadie lo dé por hecho ni
lo confunda con un olvido.

- **Detectar que el archivo cambió en disco mientras se editaba.**
  `01-alcance.md` la señalaba como parte de lo que hace grande a esta
  función. Sin recarga automática en el resto del proyecto (ya excluida
  del alcance general), añadirla solo para el modo edición no se
  justifica en esta primera entrega.
- **Edición del front matter como tabla, o de cualquier otro elemento
  renderizado directamente.** Se edita como texto, según la decisión de
  alcance de arriba.
- **Cualquier atajo de teclado más allá de guardar.** El resto de
  atajos de edición (deshacer, rehacer, cortar, pegar) los da el
  componente de `gpui-component` tal cual vienen; no se añade ninguno
  propio.
