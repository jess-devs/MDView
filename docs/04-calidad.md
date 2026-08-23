# Calidad

> Estado: en curso
> Última actualización: 2026-08-23
> Modo: new-project

Registro de verificación de los criterios de aceptación, historia por historia,
con el resultado observado. Un criterio se verifica por observación: lo que no
se pudo observar se registra como `bloqueado`, nunca como `pasa`.

Este archivo se amplía en la misma fase en que cada historia se cierra.

---

## HU-01 — Ver un documento pasándole su ruta

Verificado el 2026-08-23, en esta misma máquina de desarrollo (Windows 11 Pro
10.0.26200, 64 bits), lanzando `target\debug\mdview.exe` con la ruta de
`pruebas/hu-01-documento.md` como argumento y observando la ventana resultante.
Las capturas están en `capturas/`.

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-01.1 | pasa | `capturas/ca-01.1-formato.png`. Los encabezados aparecen en negrita y con tamaño mayor que los párrafos; no se ve ningún carácter de marcado (`#`, `**`) en pantalla. |
| CA-01.2 | pasa | `capturas/ca-01.2-scroll-abajo.png` y `capturas/ca-01.2-scroll-arriba.png`. Con la rueda del ratón sobre la ventana, el contenido se desplaza hacia abajo hasta el final del documento y vuelve hasta el encabezado inicial. |
| CA-01.3 | pasa | La propia ejecución de la verificación ocurre en Windows 11 de 64 bits; la ventana se abre y muestra el documento. |
| CA-01.4 | pasa | `capturas/ca-01.4-ventana-estrecha.png`, ventana redimensionada a 480 px de ancho. Ningún párrafo se corta ni se desborda del borde; el encabezado principal pasa a dos líneas. |
| CA-01.5 | pasa | `capturas/ca-01.5-ventana-ancha.png`, ventana redimensionada a 1880 px de ancho. La columna deja de crecer alrededor de los 720 px de AD-08 y queda centrada, con márgenes visualmente iguales a ambos lados. |

**Nota de proceso.** CA-01.4 falló en el primer intento: el texto no se
reajustaba y se salía de la ventana sin desplazamiento horizontal ni corte
visible (se recortaba contra el borde de la pantalla). La causa fue doble:
los bloques de párrafo/encabezado no tenían un ancho definido propio, y el
ancho de la columna se calculaba con `w_full()` + `max_w()` dentro de un
contenedor flex centrado, lo que no le daba a GPUI un ancho concreto con el
que ajustar el texto antes de dibujarlo. Se corrigió calculando el ancho de la
columna en píxeles a partir de `window.viewport_size()` en cada `render()` y
fijándolo con `.w(px(...))`, en vez de dejar que el layout flexible lo
dedujera. Ver `src/render.rs`.

**Medida informal de arranque (RNF-01, no es verificación).** Con cronómetro
(`Stopwatch` de PowerShell) desde el lanzamiento del proceso hasta que
`MainWindowHandle` deja de ser cero: **~6.7 s** en build de depuración
(`cargo build`, sin optimizar), con la caché de disco ya caliente por
compilaciones previas, no en arranque en frío real. Muy por encima del
objetivo de RNF-01 (<1 s), pero no es comparable a ese objetivo todavía: falta
medir con un build de release y en arranque en frío real (reinicio de
máquina), que es lo que exige HU-06. Se anota aquí solo como alerta temprana,
tal como pide `plan.md`: el tamaño de `gpui-component` (~48 000 líneas) es
candidato sospechoso.

**Estado de la historia:** verificada. Los cinco criterios pasan por
observación directa.
