# Requisitos — integracion-con-windows

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Quinta y última feature del proyecto. Cubre el comportamiento de instancia
única (RF-03): una segunda invocación de MDView, con el proceso ya en
marcha, se atiende desde ese mismo proceso en vez de arrancar uno nuevo.

**RF-02 (asociación de `.md` en el Explorador) no se cubre aquí.**
`01-alcance.md` dejaba abierto de qué mecanismo depende —instalador,
auto-registro, o registro explícito bajo demanda— y señalaba que la
respuesta depende de si existe instalador, que es una feature de
distribución todavía sin abrir. Preguntado directamente el 2026-09-18, el
usuario eligió esperar al instalador: RF-02 queda pendiente, y esta feature
se cierra solo con RF-03. Cuando se abra la feature de distribución, RF-02
se retoma con esa decisión ya tomada, no antes.

Cada requisito refina uno de `02-requisitos.md`. Un refinamiento **acota**:
aquí no aparece nada que su requisito padre no tuviera ya. Los términos
usados están en el glosario de `00-contexto.md`.

## Funcionales

### RF-03.1 — Instancia única con comportamiento de navegador

Refina: RF-03
Al invocar MDView con al menos una ruta mientras ya hay un proceso de MDView
en ejecución, esa nueva invocación no debe crear una segunda ventana: el
proceso ya en marcha recibe la ruta, la muestra en una pestaña nueva —o
activa la pestaña existente si ese archivo ya estaba abierto, RF-04.1— que
queda como pestaña activa, y su ventana pasa a primer plano. Los documentos
que ya estaban abiertos en ese proceso permanecen en sus pestañas, sin
cambios. La nueva invocación termina por sí sola, sin abrir ninguna
ventana propia.

Invocar MDView sin ninguna ruta, con un proceso ya en marcha, también debe
traer su ventana a primer plano —hay algo que atender: mostrarla—, sin
alterar qué pestaña está activa ni los documentos abiertos.

## No funcionales

### RNF-03.1 — Plataforma de ejecución

Refina: RNF-03
El sistema debe ejecutarse en Windows 11 de 64 bits.

## Requisitos del sistema que esta feature no cubre

RF-02, según lo explicado arriba. RNF-01 no se refina: la segunda
invocación —la que se limita a avisar al proceso ya en marcha y terminar—
no muestra ningún documento por sí misma, así que el umbral de «documento
visible» no le aplica; y la primera invocación, la que de verdad arranca y
muestra el documento, no cambia su camino de arranque por esta historia.
Se mide de forma informal al cerrar la feature, igual que en las
anteriores, no como criterio de aceptación.
