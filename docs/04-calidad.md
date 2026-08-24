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

**Medida informal de arranque, build de release (2026-08-23, sigue sin ser
verificación).** Mismo método (`cargo build --release`, luego `Stopwatch`
desde el lanzamiento del proceso hasta `MainWindowHandle` no nulo), mismo
documento de prueba. Tres lanzamientos consecutivos, sin reiniciar la
máquina:

1. **~3.8 s** — primer lanzamiento tras compilar, con el binario recién
   escrito a disco y sin caché de disco propia todavía.
2. **~1.1 s** — segundo lanzamiento, caché ya caliente.
3. **~0.9 s** — tercer lanzamiento, caché caliente.

El build de release es entre 2 y 6 veces más rápido que el de depuración
(esperable: optimizado, sin símbolos de depuración), pero **el riesgo de
RNF-01 no queda descartado**: incluso con caché caliente el segundo
lanzamiento sigue por encima de 1 s, y el primero (el más parecido a lo que
vería un usuario que abre el archivo por primera vez en una sesión, o tras
reiniciar) lo cuadruplica. Ninguna de las tres cifras es la medición oficial
de HU-06 —falta el arranque en frío real con reinicio de máquina y el
instrumentado de AD-07 que mide hasta el documento visible, no hasta que
existe ventana—, pero la tendencia es clara: **el margen sobre 1 s es
estrecho o inexistente incluso en el mejor caso medido informalmente**, así
que el riesgo de tamaño de `gpui-component` sobre RNF-01 sigue vivo y debe
vigilarse en HU-06, no darse por resuelto por pasar a release.

**Estado de la historia:** verificada. Los cinco criterios pasan por
observación directa.

---

## HU-02 — Leer un documento con el formato que el autor escribió

Verificado el 2026-08-23, en la misma máquina, lanzando
`target\debug\mdview.exe` con `pruebas/documento-completo.md` (compartido con
HU-03; contiene también los elementos de CA-03.x). Capturas en `capturas/`.

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-02.1 | pasa | `capturas/ca-02.1-02.2-encabezados-enfasis.png`. H1 a H6 visibles, los seis con tamaño distinto y decreciente (AD-09). |
| CA-02.2 | pasa | Misma captura: "texto en cursiva" en itálica, "texto en negrita" en negrita, "negrita y cursiva a la vez" en ambas, todo distinguible del texto normal alrededor. |
| CA-02.3 | pasa | `capturas/ca-02.3-02.5-02.7-02.9-listas-cita-codigo.png`. Lista no ordenada de dos niveles: el segundo nivel ("Elemento anidado A/B") sangrado respecto al primero, cada elemento con su viñeta `•`. |
| CA-02.4 | pasa | Misma captura, "Lista ordenada": 1, 2, 3 correlativos. |
| CA-02.5 | pasa | Misma captura, sección "Cita": margen izquierdo mayor que el texto normal y marca vertical a la izquierda. |
| CA-02.6 | pasa | `capturas/ca-02.6-02.8-02.10-hr-tabla-tachado.png`, sección "Regla horizontal": línea que recorre el ancho de la columna entre "Antes de la regla." y "Después de la regla.". |
| CA-02.7 | pasa | `capturas/ca-02.3-02.5-02.7-02.9-listas-cita-codigo.png`, "Código en línea": `pulldown_cmark::Parser::new_ext` en tipografía monoespaciada con fondo propio, distinta de la del párrafo. |
| CA-02.8 | pasa | `capturas/ca-02.6-02.8-02.10-hr-tabla-tachado.png`, sección "Tabla": rejilla de 3×3, fila de cabecera con fondo distinguible, columnas alineadas entre filas. |
| CA-02.9 | pasa | `capturas/ca-02.3-02.5-02.7-02.9-listas-cita-codigo.png`, "Lista de tareas": casillas `☐` vacías y una `☑` marcada, visualmente distintas. |
| CA-02.10 | pasa | `capturas/ca-02.6-02.8-02.10-hr-tabla-tachado.png`, sección "Tachado": "una parte tachada" con línea que la atraviesa. |

También se comprobó que el reajuste de ancho de HU-01 (CA-01.4) sigue
funcionando con el nuevo renderizador: estrechando la ventana a 500px, todos
los elementos —listas, cita, código en línea— se reajustan sin desbordar el
borde ni cortar palabras. No es un criterio de esta historia, pero era el
riesgo de regresión más obvio de cambiar el mecanismo de texto.

**Notas de proceso — dos bugs reales encontrados y corregidos antes de
verificar:**

1. **Los encabezados no variaban de tamaño.** `gpui::TextRun` (y el `Font`
   que contiene) no llevan campo de tamaño de fuente — solo familia, peso y
   estilo. El tamaño de fuente en GPUI es una propiedad ambiental del `div`
   contenedor (`.text_size()`), no algo que se pueda variar por tramo dentro
   de un `StyledText`. La primera versión intentaba fijar el tamaño dentro de
   un `TextStyle` construido a mano por bloque, que `to_run()` simplemente
   ignora. Se corrigió devolviendo `.text_size()` al `div` que envuelve cada
   bloque (igual que en HU-01) y dejando que `render_spans` solo controle
   familia/peso/estilo/tachado por tramo. El mismo problema afectaba al peso
   (negrita) de los encabezados y de la fila de cabecera de la tabla: se
   corrigió igual, fijando el peso base en el `TextStyle` de todo el bloque en
   vez de en el `div`.
2. **El texto tachado no se veía.** `StrikethroughStyle::default()` tiene
   `thickness: 0px`, así que la línea existía pero con grosor cero —
   invisible. Se corrigió fijando `thickness: px(1.0)` explícitamente.

Ambos quedaron confirmados por observación tras el arreglo, no solo inferidos
del código.

**Hallazgo para HU-04.** Sin haber implementado todavía la detección de tema
(RF-18), la aplicación ya arranca en modo oscuro en esta máquina y lo hace de
forma consistente: `gpui_component::init(cx)` debe estar leyendo el tema de
Windows por su cuenta. Cuando se aborde HU-04 hay que comprobar si esto ya
resuelve RF-18 o si solo es una coincidencia del tema por defecto de
`gpui-component`.

**Estado de la historia:** verificada. Los diez criterios pasan por
observación directa.

---

## HU-03 — Leer bloques de código sin que estorben

Verificado el 2026-08-23, en la misma máquina, con `pruebas/documento-completo.md`
ampliado con un segundo bloque de código (Python, con su propia línea larga
distinta) para poder comprobar que dos bloques desplazan de forma
independiente — no solo que uno se desplaza sin mover el resto del documento.
Capturas en `capturas/`.

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-03.1 | pasa | `capturas/ca-03.1-03.2-03.3-03.4-bloques-codigo.png`. Ambos bloques en tipografía monoespaciada. |
| CA-03.2 | pasa | Misma captura: fondo gris de los bloques, distinguible del fondo oscuro del documento. |
| CA-03.3 | pasa | Misma captura: todo el texto del mismo color; ninguna palabra clave (`fn`, `def`, `return`) aparece coloreada distinto. |
| CA-03.4 | pasa | Misma captura: los saltos de línea del bloque y la sangría de 4 espacios de `parametro_uno + ...` se ven tal como están en el archivo. |
| CA-03.5 | pasa | `capturas/ca-03.5-03.6-scroll-horizontal-independiente.png`. El primer bloque, desplazado hasta el final de su línea (`-> i32 {`, con la barra de scroll horizontal pegada al extremo derecho); la línea no aparece partida en ningún momento del desplazamiento. |
| CA-03.6 | pasa | Misma captura: con el primer bloque desplazado a mitad de su línea, el segundo bloque y los párrafos que los rodean (`Un segundo bloque de código...`, la sección `Tabla`) siguen exactamente en su sitio. |

**Notas de proceso — un bug real de identidad de estado, encontrado al
probar con dos bloques de código, no con uno:**

`gpui-component` ofrece `overflow_x_scrollbar()` como atajo para añadir
scroll horizontal y su barra visual de una vez. Con un único bloque de código
en el documento funcionaba. Al añadir un segundo bloque para comprobar
CA-03.6 con más rigor del que pide su letra literal, se detectó que
**desplazar un bloque desplazaba también al otro**: `overflow_x_scrollbar()`
deriva el identificador de su estado con `Location::caller()` — el punto del
código fuente donde se llama, el mismo para cada iteración de un bucle —, así
que todos los bloques de código de un documento comparten sin querer una sola
posición de scroll.

Se corrigió sustituyendo el atajo por un `ScrollHandle` propio por bloque
(clave `("code-block-scroll", índice)`), replicando a mano la estructura
interna que usa `gpui-component` para su propio `Scrollable` (contenedor
`.relative()` → área de scroll `.flex_row().overflow_x_scroll()` → barra de
scroll como **hermana**, no hija, del área de scroll). El primer intento
anidó la barra de scroll dentro del área con overflow, siguiendo la lectura
más obvia de la API pública; no se veía ni respondía a ningún intento de
interacción. Registrado como AD-10.

**Hallazgo de interacción, no de diseño.** La rueda del ratón vertical simple
no mueve un bloque que solo tiene overflow horizontal — hace falta rueda
horizontal real (Shift+rueda, gesto de trackpad) o arrastrar la barra. Esto
retrasó la verificación (varios intentos de simular rueda vertical no
hicieron nada) pero no es un defecto de la aplicación: es el comportamiento
esperable de GPUI para una región que no tiene overflow vertical. Anotado en
AD-10 para que nadie lo confunda con un bug si lo redescubre.

**Estado de la historia:** verificada. Los seis criterios pasan por
observación directa.

---

## HU-04 — Ver el documento con el tema del sistema

Verificado el 2026-08-23. **Requirió cambiar el tema de Windows.** Tema
original de la máquina antes de tocar nada: **oscuro**
(`AppsUseLightTheme=0`, `SystemUsesLightTheme=0` en
`HKCU\Software\Microsoft\Windows\CurrentVersion\Themes\Personalize`,
comprobado antes del primer cambio). Se cambió a claro para CA-04.1, y se
devolvió a oscuro al terminar — confirmado leyendo esas mismas claves después
de restaurarlas y con una captura del propio MDView ya de vuelta en oscuro.
**El tema de Windows de esta máquina quedó exactamente como estaba antes de
empezar.**

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-04.1 | pasa | `capturas/ca-04.1-04.3-claro-encabezados.png` y las otras dos `ca-04.1-...png`. Con Windows en modo claro, fondo claro y texto oscuro. |
| CA-04.2 | pasa | `capturas/ca-04.2-04.3-oscuro-encabezados.png` y las otras dos `ca-04.2-...png`. Con Windows en modo oscuro, fondo oscuro y texto claro. |
| CA-04.3 | pasa | Las seis capturas anteriores cubren todos los elementos verificados en HU-02 y HU-03 (encabezados 1-6, énfasis/negrita, listas anidadas, lista ordenada, lista de tareas, cita, regla horizontal, código en línea, bloque de código, tabla, tachado) en ambos temas. Ningún texto se confunde con su fondo en ninguno de los dos. |

**Nota de proceso — la sospecha de HU-02 se confirmó, con matiz.** Se leyó el
código de `gpui-component`: `gpui_component::init(cx)` ya sincroniza el tema
una vez con `cx.window_appearance()` (una API de plataforma de GPUI, no una
lectura manual del registro), lo que explica que la app ya arrancara en
oscuro en HU-02 sin código propio de tema. Pero esa sincronización ocurre
**antes de que exista la ventana** y no se repite si el usuario cambia el
tema de Windows con la app ya abierta. Se añadió una resincronización contra
la ventana real al abrirla, más una suscripción a cambios de tema en
caliente (`window.observe_window_appearance`). Registrado en AD-11.

**Verificación extra, más allá de la letra de los CA.** Con MDView ya
abierto (sin reiniciarlo), se cambió el tema de Windows de oscuro a claro y
se comprobó que la ventana ya abierta se actualizó sola, sin relanzar la
aplicación: `capturas/ca-04.1-04.3-claro-listas-cita-cambio-en-vivo.png` es
precisamente esa captura, tomada inmediatamente después del cambio en
caliente. No lo pedía ningún criterio, pero confirma que la suscripción de
AD-11 funciona de verdad y no solo al arrancar.

**Estado de la historia:** verificada. Los tres criterios pasan por
observación directa.

---

## HU-05 — Enterarme de que el archivo no se puede mostrar

Verificado el 2026-08-23. Datos de prueba, todos en `pruebas/` y borrados al
cerrar la historia: una ruta inexistente (sin archivo que crear), una imagen
PNG de 10×10 guardada con extensión `.md`, y un archivo de texto con una
regla de acceso `icacls /deny` para el usuario actual. La regla de acceso se
retiró (`icacls /remove:d`) y se comprobó que el archivo quedó con los
mismos permisos heredados que tenía antes de aplicarla.

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-05.1 | pasa | `capturas/ca-05.1-05.5-no-existe.png`. Aviso: «No se encontró «...\no-existe.md».», sin mostrar documento. |
| CA-05.2 | pasa | `capturas/ca-05.2-05.5-no-es-texto.png`. Aviso: «...\imagen-como-md.md» no es un archivo de texto.», sin mostrar documento. |
| CA-05.3 | pasa | `capturas/ca-05.3-05.5-sin-permiso.png`. Aviso: «No se pudo leer «...\sin-permiso.md»: no hay permiso para leerlo.», sin mostrar documento. |
| CA-05.4 | pasa | `capturas/ca-05.4-aviso-desaparecido.png`, tomada más de 5 s después de la anterior sin tocar nada: el aviso ya no está. |
| CA-05.5 | pasa | En los tres casos, `Get-Process` reportó `Responding: True` justo después de cada lanzamiento; la aplicación no terminó ni dejó de responder en ningún caso. |

**Nota de proceso.** El aviso no aparecía en el primer intento, aunque el
código compilaba y no fallaba: `Root` (el componente raíz que ya se usaba
desde HU-01) no pinta las notificaciones por sí solo —`impl Render for
Root` no incluye esa capa—. Hubo que componerla a mano llamando a
`Root::render_notification_layer(window, cx)` desde `DocumentView::render` y
añadir su resultado como hijo. Sin ese paso, `window.push_notification(...)`
actualiza el estado pero nada lo pinta. Registrado en AD-12, junto con la
clasificación de errores (`document::LoadError`) y la elección de mensaje.

**Estado de la historia:** verificada. Los cinco criterios pasan por
observación directa.
