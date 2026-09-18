# Historias — distribucion

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Tres historias, un único tipo de usuario: el lector de documentación técnica
de `00-contexto.md`, en su papel de quien instala MDView por primera vez
—o lo retira— en un equipo Windows 11.

---

## HU-01 — Instalar MDView deja `.md` disponible en el Explorador y `mdview` disponible en la terminal

Cubre: RF-02.1, RF-21.1 (parcial: instalación)

**Historia.** Como lector de documentación técnica, quiero un instalador que
deje MDView listo para abrir `.md` desde el Explorador y desde la terminal,
sin tener que escribir rutas completas ni tocar el registro a mano.

**Criterios de aceptación.** Se comprueban ejecutando el instalador en una
cuenta de usuario estándar (sin permisos de administrador) y observando el
resultado.

1. CA-01.1 — El instalador se ejecuta sin pedir elevación (sin UAC): todo lo
   que registra vive en el ámbito del usuario actual.
2. CA-01.2 — Tras instalar, `mdview.exe` existe en la carpeta de instalación
   bajo el perfil del usuario.
3. CA-01.3 — Clic derecho sobre un `.md` en el Explorador → «Abrir con» →
   MDView aparece en la lista de aplicaciones disponibles.
4. CA-01.4 — Elegir MDView en ese diálogo abre el documento correctamente.
5. CA-01.5 — El instalador no deja MDView como predeterminada por su cuenta:
   antes de que el usuario elija nada en «Abrir con», el doble clic directo
   sobre un `.md` sigue abriendo la aplicación que ya fuera predeterminada
   antes de instalar (o el propio diálogo de selección, si no había
   ninguna).
6. CA-01.6 — Abrir una terminal **nueva** (una ya abierta no hereda el PATH)
   y ejecutar `mdview ruta\a\un\archivo.md` abre ese documento, sin escribir
   la ruta del ejecutable.

**Estado.** pendiente.

---

## HU-02 — Desinstalar MDView retira lo que el instalador dejó, sin rastro

Cubre: RF-21.1 (parcial: desinstalación)

**Historia.** Como lector de documentación técnica, quiero que desinstalar
MDView deje el sistema como estaba antes de instalarlo, sin una entrada de
PATH rota ni una opción de «Abrir con» que ya no funciona.

**Criterios de aceptación.** Se comprueban desinstalando MDView desde el
panel de «Aplicaciones» de Windows, tras haber cumplido HU-01.

1. CA-02.1 — Tras desinstalar, `mdview.exe` ya no existe en la carpeta de
   instalación.
2. CA-02.2 — Una terminal nueva ya no encuentra `mdview` en el PATH.
3. CA-02.3 — «Abrir con» sobre un `.md` ya no ofrece MDView como opción.

**Estado.** pendiente.

---

## HU-03 — Usar la versión portable sin instalar nada

Cubre: RF-22.1

**Historia.** Como lector de documentación técnica, quiero poder llevar
MDView en una memoria USB o una carpeta cualquiera, sin instalador, para
usarlo en un equipo donde no quiero o no puedo instalar nada.

**Criterios de aceptación.** Se comprueban extrayendo el `.zip` portable en
una carpeta nueva, en un equipo donde MDView no está instalado.

1. CA-03.1 — Ejecutar el `.exe` extraído abre MDView con normalidad, con el
   mismo comportamiento que la versión instalada.
2. CA-03.2 — Antes y después de ejecutarlo, el registro de asociaciones de
   `.md` del usuario no cambia, y el PATH de usuario tampoco: la versión
   portable no escribe nada fuera de su propia carpeta.

**Estado.** pendiente.

---

## Cobertura

| Requisito | Historia que lo cubre |
| --- | --- |
| RF-02.1 | HU-01 |
| RF-21.1 | HU-01 (instalación), HU-02 (desinstalación) |
| RF-22.1 | HU-03 |

Los tres requisitos de esta feature quedan cubiertos entre las tres
historias.
