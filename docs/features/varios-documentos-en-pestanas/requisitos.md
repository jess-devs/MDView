# Requisitos — varios-documentos-en-pestanas

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Cuarta feature del proyecto. Cubre tener varios documentos abiertos a la vez,
en pestañas, y navegar entre ellos con enlaces a otros `.md` (RF-14). No cubre
RF-02 (asociación de `.md` en el Explorador) ni RF-03 (instancia única): esas
dos van en `integracion-con-windows`, la última feature, porque el
comportamiento de «instancia única» —una petición de apertura la atiende el
proceso que ya existe— solo tiene sentido observarlo una vez que abrir desde
el Explorador dispara un proceso nuevo por cada doble clic, que es justo lo
que esa feature construye.

Cada requisito refina uno de `02-requisitos.md`. Un refinamiento **acota**:
aquí no aparece nada que su requisito padre no tuviera ya. Los términos
usados están en el glosario de `00-contexto.md`.

## Funcionales

### RF-01.1 — Apertura de varias rutas en una sola invocación

Refina: RF-01
El sistema debe admitir varias rutas de archivo `.md` como argumentos de una
sola invocación por línea de comandos, y abrir cada una en su propia pestaña.
La primera pestaña —la del primer argumento— queda como pestaña activa.

### RF-04.1 — No duplicar una pestaña ya abierta

Refina: RF-04
Si una acción dentro de la sesión en curso —activar un enlace a otro `.md`
(RF-14), o que la lista de rutas de la invocación repita una ruta— pide
abrir un archivo que ya tiene una pestaña abierta, esa pestaña pasa a ser la
activa y no se crea una segunda para el mismo documento. Dos rutas distintas
que apuntan al mismo archivo en disco —una relativa y otra absoluta, por
ejemplo— cuentan como la misma ruta si resuelven al mismo archivo.

### RF-05.1 — Cambiar y cerrar pestañas

Refina: RF-05
El sistema debe permitir, con cualquier número de pestañas abiertas: activar
una pestaña distinta de la activa, y cerrar una pestaña concreta —activa o
no— sin afectar a las demás.

### RF-06.1 — La pestaña muestra el nombre del archivo

Refina: RF-06
Cada pestaña debe mostrar el nombre del archivo que contiene —el nombre tal
como aparece en el sistema de archivos, extensión incluida, no una ruta
completa ni el primer encabezado del documento (supuesto 5 de
`01-alcance.md`, confirmado aquí como parte del alcance de esta feature).

### RF-07.1 — Cerrar la última pestaña termina la aplicación

Refina: RF-07
Al cerrarse la pestaña activa cuando es la única abierta, la aplicación debe
terminar, igual que si se cerrara la ventana.

### RF-14.1 — Enlace a otro documento abre una pestaña nueva

Refina: RF-14
Al activarse un enlace cuyo destino, resuelto como ruta relativa al
directorio del documento que lo contiene, señale a un archivo con extensión
`.md`, el sistema debe abrir ese archivo en una pestaña nueva y activarla —o
activar su pestaña existente, si ya estaba abierto (RF-04.1). Un enlace cuyo
destino resuelto no exista, o no sea legible, o no sea texto, se trata como
cualquier apertura fallida: el aviso temporal de RF-17, sin pestaña nueva.

## No funcionales

### RNF-03.1 — Plataforma de ejecución

Refina: RNF-03
El sistema debe ejecutarse en Windows 11 de 64 bits.

## Requisitos del sistema que esta feature no cubre

RF-02 y RF-03 van en `integracion-con-windows`, por la razón dada arriba.
RF-15.1, RF-11.1, RF-12.1 y RF-13.1 ya están cubiertos por las dos features
anteriores y no se tocan aquí. RNF-01 y RNF-02 no se refinan: abrir varios
documentos no añade E/S de red, y el riesgo de tiempo de arranque de RNF-01
no cambia por tener más de una pestaña disponible —abrir la primera sigue
siendo lo único que ocurre antes del primer fotograma—; se vigila con una
medida informal al cerrar la feature, no como criterio de aceptación, igual
que se hizo en `front-matter-y-html`.
