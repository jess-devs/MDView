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

---

## HU-07 — Entender qué es esto si lo abro sin un archivo

Verificado el 2026-08-23. Sin datos de prueba: el propio caso es invocar
`mdview.exe` sin argumentos.

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-07.1 | pasa | `capturas/ca-07.1-07.2-07.3-estado-vacio.png`. La ventana aparece con contenido visible, no queda en blanco ni deja de aparecer nada. |
| CA-07.2 | pasa | Misma captura: «MDView muestra archivos Markdown (.md) con formato, sin necesidad de abrir un editor de código.». |
| CA-07.3 | pasa | Misma captura: «Para abrir uno, indícale su ruta al iniciarlo: `mdview ruta\al\archivo.md`», una forma concreta y hoy realmente disponible (RF-02, doble clic, pertenece a la feature `integracion-con-windows` y no existe todavía; no se prometió). |

**Nota de proceso.** Se distinguió explícitamente "sin argumento" (este
caso, `AppState.no_path_given`) de "argumento que falla" (HU-05,
`pending_notice`): antes de este cambio ambos casos dejaban `blocks` vacío
de la misma forma, y mostrar el texto de bienvenida detrás de un aviso de
error de HU-05 habría sido una respuesta que no venía a cuento —el usuario sí
indicó una ruta, solo que no funcionó—. Los dos estados ahora son
independientes en `AppState` y no se pisan.

**Estado de la historia:** verificada. Los tres criterios pasan por
observación directa.

---

## HU-06 — Ver el documento sin esperar (fase 1: preparación)

**Esto NO es la verificación de CA-06.1.** Esa verificación exige tres
arranques en frío con reinicio de máquina entre cada uno, coordinados con el
usuario, y queda pendiente como fase 2. Lo que sigue es la instrumentación
lista, el perfil de `release` decidido (AD-13) y una medida informal en
caliente que sirve de referencia y de alerta temprana, no de criterio de
aceptación. HU-06 sigue **en curso**, no verificada.

**Instrumentación `MDVIEW_TIMING` (AD-07).** Comprobada el 2026-08-24:
lanzando `target\debug\mdview.exe` con la variable de entorno apuntando a un
fichero, aparece una línea con las dos marcas y la diferencia:

```
entrada_ms=1787543747526 primer_frame_ms=1787543748831 diferencia_ms=1305
```

Sin la variable definida, se comprobó (lanzando y comprobando que el fichero
no aparece) que no se crea ningún fichero — coste cero confirmado por
observación, no solo por lectura del código.

**Archivo de prueba.** `pruebas/documento-50kb.md`, generado, 50 032 bytes.
Dato de prueba de esta historia: se borra al cerrarla, junto con los
ficheros de medidas que genere la fase 2.

**Medida informal en caliente, release con el perfil de AD-13 (2026-08-24).**
Tres lanzamientos consecutivos de `target\release\mdview.exe` con
`pruebas/documento-50kb.md`, caché de disco ya caliente, sin reiniciar la
máquina:

1. 642 ms
2. 599 ms
3. 588 ms

Media ~610 ms. Antes de aplicar AD-13 (perfil de `release` por omisión,
mismo método): 744 ms, 779 ms, 658 ms, media ~727 ms. La comparación completa
y la decisión de adoptar el perfil están en AD-13.

**Lectura de esta cifra.** ~610 ms en caliente dista de lo que exige RNF-01
(<1 s) en el peor caso —arranque en frío—, que es justo lo que esta medida
no mide. Con caché caliente casi todo el trabajo de carga de páginas y
resolución de símbolos ya está hecho por el sistema operativo; un arranque en
frío tras reiniciar la máquina no tiene ese margen. Sigue siendo el mismo
riesgo que ya señalaba `plan.md` desde la Fase 4 por el tamaño de
`gpui-component`: esta cifra es un dato a favor, no una resolución del
riesgo.

**Instrucción exacta para la fase 2 (arranque en frío, tras cada reinicio):**

```powershell
$env:MDVIEW_TIMING = "C:\ruta\que\se\quiera\timing.txt"
& "E:\Code\Rust\MDView\target\release\mdview.exe" "E:\Code\Rust\MDView\pruebas\documento-50kb.md"
```

Repetir tres veces, cada una tras un reinicio completo de la máquina (no
basta con cerrar y volver a abrir: el objetivo es que el sistema operativo
no tenga nada de `mdview.exe` ni de sus bibliotecas en caché de páginas).
Leer después `C:\ruta\que\se\quiera\timing.txt`: cada línea trae
`diferencia_ms`, que es la cifra que exige CA-06.1. El fichero de medidas y
`pruebas/documento-50kb.md` se borran al cerrar HU-06.

**Estado de la historia:** en curso. La fase 1 (instrumentación, perfil,
medida informal) está completa; CA-06.1 sigue sin verificar.

---

## HU-06 — Ver el documento sin esperar (fase 2: verificación de CA-06.1)

Verificado el 2026-08-24 (madrugada), coordinado con el usuario. Método: el
de `plan.md`, con una condición **más estricta** que la mínima documentada —
en lugar de un solo reinicio antes de la primera medición con las tres
medidas consecutivas, se reinició la máquina por completo **antes de cada
una** de las tres, y cada reinicio se comprobó real leyendo
`LastBootUpTime` antes de medir. Cada medición: `MDVIEW_TIMING` apuntando a
`pruebas/timing.txt`, `target\release\mdview.exe` (perfil de AD-13) con
`pruebas/documento-50kb.md`, documento visible confirmado en pantalla.

Las tres líneas del fichero de medidas, copiadas aquí antes de borrarlo:

```
entrada_ms=1787558783978 primer_frame_ms=1787558785034 diferencia_ms=1055
entrada_ms=1787558959579 primer_frame_ms=1787558960335 diferencia_ms=755
entrada_ms=1787559074968 primer_frame_ms=1787559075933 diferencia_ms=965
```

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-06.1 — <1 s hasta documento visible en los tres arranques en frío | **no pasa** | 1055 ms, 755 ms, 965 ms: el primero supera 1 s |

(Resultado contra el umbral vigente el 2026-08-24. El umbral cambió después: ver la fase 3 al final de este documento y AD-14.)

**Lectura.** Dos de tres arranques cumplen y el que falla lo hace por 55 ms
(5,5 %). El perfil de AD-13 acercó la cifra al objetivo pero, como su propia
consecuencia anticipaba, no basta por sí solo: queda pendiente trabajo de
optimización con nombre (los candidatos anotados en AD-13: reducir qué se
carga de `gpui-component`, o revisar AD-01) o, alternativamente, renegociar
el umbral de RNF-01 — esa elección es del usuario y no se toma aquí. No se
registra como `pasa` por estar cerca: la letra del criterio exige menos de
1 s **en los tres**.

**Estado de la historia** (al cerrar la fase 2; lo continua la fase 3):
verificación ejecutada, CA-06.1 **no pasa**. La
historia queda abierta a la espera de la decisión sobre el pendiente. Los
datos de prueba (`pruebas/documento-50kb.md` y `pruebas/timing.txt`) se
borran ya: las cifras están copiadas arriba y el archivo de 50 KB es
regenerable; si la optimización pendiente exige repetir la medición, se
regeneran según la instrucción de la fase 1.

---

## HU-06 — Ver el documento sin esperar (fase 3: cierre tras AD-14)

Sin medición nueva. La fase 2 queda arriba tal como se escribió, con su
resultado contra el umbral que estaba vigente entonces; esta sección no la
reescribe, la continúa.

Puesta la decisión al usuario —optimizar más o renegociar el umbral—, eligió
renegociar. AD-14 lleva RNF-01 y RNF-01.1 de «menos de 1 s» a «menos de
1,2 s» en arranque en frío, y CA-06.1 se reformula con esa cifra. El
razonamiento y las alternativas descartadas están en la decisión; aquí solo
el resultado.

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-06.1 — <1,2 s hasta documento visible en los tres arranques en frío | **pasa** | Las mismas tres medidas del 2026-08-24: 1055 ms, 755 ms, 965 ms, todas por debajo de 1,2 s |

No se remide porque no hace falta: las medidas existentes se tomaron en
condiciones **más estrictas** que las que el criterio pide (reinicio de
máquina completo y comprobado antes de cada una de las tres, no solo antes
de la primera), y el criterio nuevo es más laxo que aquel contra el que se
tomaron. Un dato que pasa un listón alto pasa el bajo.

**Estado de la historia:** verificada. Con ella, las siete historias de la
feature `ver-un-documento` quedan verificadas.

---

## HU-01 (enlaces-e-imagenes) — Leer un documento con enlaces sin perder nada

Verificado el 2026-08-24. La construcción y la observación en pantalla las hizo
la sesión que implementó la historia; esta sesión revisó el código, ejecutó los
tests por su cuenta y comprobó los criterios contra las capturas. Las capturas
están en `pruebas/` (no se versionan, ver `.gitignore`).

Documento de prueba: `pruebas/hu-01-enlaces.md`, un párrafo con la forma «texto
antes, enlace, texto después» seguido de un encabezado y otro párrafo.

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-01.1 — Las tres partes del párrafo, en orden y en un mismo párrafo | pasa | `ca-01-captura.png`: «Este párrafo tiene texto antes, luego un enlace de ejemplo y texto después que debe seguir viéndose.», en una sola línea de párrafo |
| CA-01.2 — Texto del enlace distinguible del que lo rodea | pasa | `ca-01-zoom.png`: «enlace de ejemplo» va subrayado; el resto del párrafo no |
| CA-01.3 — La URL no aparece en el cuerpo del documento | pasa | En la captura no aparece `https://example.com/ruta` por ninguna parte |
| CA-01.4 — El encabezado posterior y lo que le sigue se siguen mostrando | pasa | `ca-01-captura.png`: «Encabezado posterior» y su párrafo se ven bajo el enlace |
| RNF-03.1 — Windows 11 de 64 bits | pasa | La aplicación se ejecutó y se observó en esa plataforma |

**Prueba de regresión.** `cargo test` ejecuta los dos casos que `plan.md`
documentó como defecto observado. Ejecutados por esta sesión:

```
test markdown::tests::html_block_does_not_truncate_the_document ... ok
test markdown::tests::link_and_image_stay_in_one_paragraph_with_the_rest_of_the_text ... ok
test result: ok. 2 passed; 0 failed
```

Son los primeros tests del proyecto (AD-17). No sustituyen a la observación: la
comprobación de los cuatro criterios es la de la tabla, no la de arriba.

**Dos cosas que no son criterio y conviene no perder.**

- El color del enlace se fija con `theme.link`, pero en el tema oscuro esa
  cifra es casi indistinguible del texto normal: lo que hace visible el enlace
  en la captura es el subrayado. CA-01.2 admite «por color, por subrayado o por
  ambos», así que pasa. Si algún día se quita el subrayado, el criterio dejaría
  de pasar sin que nadie tocara el color.
- El bloque HTML ya no trunca el documento, y el test lo cubre. **No se declara
  verificado**: RF-13 pertenece a `front-matter-y-html` y allí se observará. Es
  el efecto colateral que AD-15 anticipaba.

**Medida informal de arranque (no es verificación).** 1941 ms sobre el binario
de **depuración**, que es el único que existe ahora mismo. No es comparable con
los ~610 ms en caliente de AD-13 ni con las cifras de CA-06.1, que se tomaron
sobre el perfil de release con LTO. Como señal de desvío para RNF-01.2 no sirve:
la próxima medida informal debe tomarse sobre release o no tomarse.

**Estado de la historia:** verificada. Los cuatro criterios pasan por
observación.

---

## HU-02 (enlaces-e-imagenes) — Abrir un enlace externo en el navegador

Verificado el 2026-09-18, en la misma máquina de desarrollo. `cargo test`
(2 casos, los mismos de HU-01) sigue en verde tras el cambio. Compilado
`target\debug\mdview.exe` con el fix de CA-02.5 ya incluido.

Documentos de prueba: `pruebas/hu-02-enlaces.md` (un enlace `https` y uno
`http` en párrafos, con relleno para desplazar) y `pruebas/hu-02-tabla.md`
(un enlace dentro de una celda de tabla, una celda sin enlace, y un párrafo
final).

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-02.1 — Un enlace `https` hace que el navegador muestre esa dirección | pasa | Verificado en la sesión que implementó la historia (2026-08-24): `pruebas/ca-02-antes.png` y `pruebas/ca-02-tras-click.png`, captura antes y después del clic con el navegador visible de fondo. |
| CA-02.2 — Un enlace `http` se comporta igual | pasa | Mismo mecanismo que CA-02.1 (`app::activate_link` no distingue el esquema); la captura de 2026-08-24 incluye ambos enlaces en el mismo documento. |
| CA-02.3 — La dirección no se abre dentro de la ventana de MDView | pasa | `pruebas/ca-02-tras-click.png`: MDView sigue mostrando el mismo documento en la misma posición; el navegador aparece como ventana aparte. Reconfirmado esta sesión: tras el clic, captura de pantalla de MDView sin cambios de contenido ni de scroll. |
| CA-02.4 — MDView sigue respondiendo tras abrir el enlace | pasa | Esta sesión: `Get-Process mdview` devuelve `Responding: True` inmediatamente después del clic, y el documento admite scroll con normalidad a continuación. |
| CA-02.5 — Un enlace `https` en celda de tabla se abre igual que uno en párrafo | pasa | Esta sesión, en dos tiempos. Primero sin acceso al navegador: se lanzó `pruebas/hu-02-tabla.md`, el enlace de la celda se veía distinguible (subrayado, sin URL en el cuerpo) y el clic sobre su rango de texto no cerró ni colgó MDView (`Get-Process mdview` → `Responding: True`), pero el resultado del clic —si abría algo— quedó sin observar. El usuario concedió después acceso de solo lectura a Dia (su navegador predeterminado real) y se repitió la prueba: al pulsar «ejemplo en tabla», Dia abrió una pestaña nueva («Example Domain») cuya barra de direcciones, ampliada, muestra `https://example.com/tabla` — exactamente la URL de la celda. Vuelta a MDView: mismo documento, misma posición, sin cambios. |

**Prueba de regresión.** `cargo test` sigue en verde (2/2), sin cambios
respecto a HU-01: esta historia no tocó `markdown.rs`.

**Nota de proceso — la corrección venía de una sesión anterior, sin
comprometer.** El código de `src/render.rs` y la decisión en
`decisiones.md` que cierran CA-02.5 ya estaban escritos en el árbol de
trabajo al empezar esta sesión, sin commit. Esta sesión los revisó línea por
línea, corrió `cargo test`, compiló y verificó los cinco criterios por
observación — no escribió el fix.

**Nota de proceso — el primer intento de observar CA-02.5 se bloqueó, y eso
fue correcto, no un fallo.** El acceso de pantalla al navegador se denegó
dos veces antes de que el usuario, presente en la conversación, lo concediera
explícitamente a la tercera. Entre medias este archivo registró el criterio
como `bloqueado`, no como `pasa` apoyado en que el código reutiliza el mismo
camino que CA-02.1 — que es una garantía de diseño, no una observación. La
comprobación real, una vez concedido el acceso, tardó menos de un minuto y
confirmó exactamente lo que el diseño prometía; pero el orden importa: la
confirmación vino de mirar la pantalla, no de leer el código.

**Estado de la historia:** verificada. Los cinco criterios pasan por
observación directa.

---

## HU-03 (enlaces-e-imagenes) — Ver las imágenes que el documento referencia

Verificado el 2026-09-18, en la misma máquina de desarrollo. `cargo test`
(5 casos: los 2 de HU-01/HU-02 más 3 propios de esta historia sobre
`Inline`/`ImageRef`) en verde. Compilado `target\debug\mdview.exe`.

Documento de prueba: `pruebas/hu-03-imagenes/documento.md`, con una imagen
normal (`img/foto.png`, 400×250, generada con `System.Drawing` de .NET), una
imagen más ancha que la columna de lectura (`img/ancha.png`, 2200×300), una
ruta que no existe (`img/no-existe.png`) y un archivo con extensión `.png`
que en realidad es texto plano (`img/no-es-imagen.png`), seguidas de un
encabezado y un párrafo final. Lanzado con `-WorkingDirectory
D:\Code\Rust\MDView`, un directorio distinto al que contiene el documento
(`pruebas\hu-03-imagenes`), para que CA-03.2 se comprobara de verdad y no
por coincidencia.

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-03.1 — Imagen con ruta relativa mostrada en su punto del documento | pasa | Captura de esta sesión: `foto.png` aparece entre los dos párrafos donde el documento la referencia, no al principio ni al final. |
| CA-03.2 — Sigue mostrándose invocando MDView desde otro directorio | pasa | La captura es precisamente de ese lanzamiento: proceso iniciado con `-WorkingDirectory D:\Code\Rust\MDView`, documento en `pruebas\hu-03-imagenes\documento.md`; ambas imágenes válidas se ven. |
| CA-03.3 — Ruta que no corresponde a ningún archivo muestra su texto alternativo, y el resto del documento sigue | pasa | Captura: bajo «Imágenes que fallan», el texto «esta ruta no existe» aparece en el lugar de `img/no-existe.png`; el encabezado «Final del documento» y su párrafo se siguen mostrando debajo. |
| CA-03.4 — Archivo existente no decodificable muestra su texto alternativo, sin terminar ni dejar de responder | pasa | Misma captura: «este archivo no es una imagen decodificable» se muestra en el lugar de `img/no-es-imagen.png` (un `.png` que es texto plano). `Get-Process mdview` → `Responding: True` tras el scroll. |
| CA-03.5 — Imagen más ancha que la columna se ajusta a ese ancho, sin desbordar ni pedir scroll horizontal | pasa | Captura y zoom sobre la franja de `ancha.png`: la imagen de 2200×300 se ve completa dentro del ancho de la ventana, proporción mantenida, sin tocar los bordes ni aparecer barra de scroll horizontal. |

**Prueba de regresión.** `cargo test`: 5/5 en verde, incluyendo los 2 casos
de HU-01/HU-02 (`markdown.rs` no perdió su comportamiento previo con enlaces
y bloques HTML).

**Nota de proceso — decisión de arquitectura nueva.** Mostrar una imagen de
verdad (no su texto alternativo haciendo de texto, que era el comportamiento
heredado de HU-01) exigió cambiar `Block::Paragraph` de `Vec<Span>` a
`Vec<Inline>`, resolver la ruta relativa al directorio del documento dentro
de `markdown::parse`, y decidir cómo se dibuja una imagen mezclada con texto
en el mismo párrafo (GPUI no permite intercalar una imagen de verdad dentro
de un `StyledText`). El razonamiento completo, con las alternativas
descartadas, está en AD-20.

**Estado de la historia:** verificada. Los cinco criterios pasan por
observación directa.

---

## HU-04 (enlaces-e-imagenes) — Saber que MDView no se conecta a nada

Verificado el 2026-09-18, en la misma máquina de desarrollo, con el método
de `plan.md`. Documento de prueba: `pruebas/hu-04-red/documento.md`, con un
enlace `https`, uno `http`, y una imagen referenciada por URL `http`
(`http://example.com/no-deberia-pedirse.png`), seguidos de un encabezado y un
párrafo final.

**Método.** Lanzado `target\debug\mdview.exe` con `Start-Process -PassThru`
para capturar el PID. Desde el lanzamiento y durante 6 segundos —de sobra
para que el documento esté visible—, muestreo cada 80 ms (unas 75 muestras)
de `Get-NetTCPConnection -OwningProcess <pid>` y
`Get-NetUDPEndpoint -OwningProcess <pid>`.

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-04.1 — Ninguna conexión TCP ni extremo UDP atribuible al proceso, desde el arranque hasta el documento visible | pasa | 0 muestras con datos de 75 muestreadas a lo largo de 6 s. |
| CA-04.2 — La imagen por URL muestra su texto alternativo, no un hueco ni un error | pasa | Captura: «imagen remota que nunca se descarga» aparece en el lugar de la imagen; el encabezado y párrafo posteriores se siguen mostrando. |
| CA-04.3 — Al abrir un enlace externo (HU-02) sí aparece tráfico, pero del navegador, no de MDView | pasa | Tras pulsar el enlace `https`: `Get-NetTCPConnection -OwningProcess <pid de mdview>` sigue devolviendo 0; `Get-NetTCPConnection` sobre los procesos de Dia (el navegador) devuelve 5 conexiones `Established` en el puerto 443. `Get-Process mdview` → `Responding: True` después del clic. |

**Límite del método, como pide `plan.md` anotar junto al resultado.** Es un
muestreo cada 80 ms, no una captura continua: una conexión que se abriera y
cerrara entera entre dos muestras no se vería. Da confianza razonable sobre
75 muestras en 6 s, no una demostración. El propio CA-04.3 —el método sí ve
tráfico cuando lo hay, sobre el navegador— es lo que respalda que la
ausencia de tráfico de MDView en CA-04.1 no es que el método esté ciego.

**Nota de proceso.** Ninguna dependencia nueva entró para esta historia:
`markdown`/`render` no hacen peticiones de red por construcción (AD-20 ya lo
estableció para las imágenes; los enlaces siguen sin pasar por nada más que
`open::that_detached`, ver AD-19). No hizo falta ningún cambio de código
para HU-04, solo el documento de prueba y la medición.

**Estado de la historia:** verificada. Los tres criterios pasan por
observación directa.

---

## HU-05 (enlaces-e-imagenes) — Seguir viendo el documento sin esperar

Verificado el 2026-09-18, coordinado con el usuario: exige tres arranques en
frío con reinicio completo de la máquina antes de cada uno, igual que
`ver-un-documento/HU-06`.

**Documento de prueba.** `pruebas/hu-05-arranque/documento.md`, 50,0 KB,
generado con relleno (el mismo patrón que `documento-50kb.md` en
`ver-un-documento`), con tres imágenes locales intercaladas
(`img/captura1.png`, `.../captura2.png`, `.../captura3.png`), 334–356 KB
cada una —del orden de una captura de pantalla real, no miniaturas, como
pide `plan.md`—, generadas con `System.Drawing` de .NET con ruido por
píxel para que el PNG no comprima a casi nada como haría un color plano.

**Automatización.** Para no requerir intervención manual en cada medida, se
montó `C:\...\Escritorio\MDView-HU05\medir.bat`: en cada ejecución comprueba
`(Get-Date) - LastBootUpTime` y se niega a medir si pasaron más de 10
minutos desde el arranque (para no colar una medida en caliente como si
fuera en frío), lanza `mdview.exe` con `MDVIEW_TIMING` apuntando a
`timing.txt`, espera a que aparezca la línea, la copia a `resultado.txt`
—en modo añadir, una línea por medida, nunca se sobrescriben entre sí— y
pregunta si reiniciar para la siguiente. Sin registro en el arranque de
Windows: el usuario vuelve a hacer doble clic tras cada reinicio. Probado
por esta sesión antes de entregarlo (lanzamiento, captura de
`diferencia_ms`, escritura en `resultado.txt`, sin la propia comprobación
de arranque en frío, imposible de simular sin reiniciar de verdad); un
primer intento con `echo 0>archivo.txt` escribía el archivo vacío —`cmd`
interpreta `0>`/`1>` como redirección de un descriptor de archivo en vez de
como el texto «0» o «1»—, corregido añadiendo un espacio antes de cada `>`.

**Las tres medidas, copiadas de `resultado.txt` antes de borrarlo:**

```
Medida 1: diferencia_ms=666   segundos_desde_arranque=115
Medida 2: diferencia_ms=699   segundos_desde_arranque=40
Medida 3: diferencia_ms=723   segundos_desde_arranque=48
```

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-05.1 — <1,2 s hasta documento visible en los tres arranques en frío | pasa | 666, 699 y 723 ms: las tres por debajo de 1,2 s, con margen de al menos 477 ms en la peor (muy por encima del margen de 145 ms que dejaba CA-06.1 en `ver-un-documento` sin imágenes). |

**Lectura.** El riesgo que `plan.md` señalaba desde el principio —«decodificar
imágenes antes del primer fotograma puede consumir el margen entero»— no se
materializó: `gpui::img()` carga de forma asíncrona (AD-20) y no bloquea el
primer fotograma, así que las tres imágenes de ~340 KB cada una no movieron
la aguja frente a los ~610–755 ms que ya daba HU-06 sin ninguna imagen.

**Nota de proceso — Smart App Control bloqueaba el build de release.**
Antes de poder medir nada hizo falta compilar `target\release\mdview.exe`
por primera vez en esta máquina, y **Smart App Control** de Windows
bloqueaba a `rustc.exe` cargar las DLL de macros recién compiladas
(`paste-....dll`) y, en un segundo intento en otra carpeta, el script de
build de `tree-sitter-json`: confirmado en
`Microsoft-Windows-CodeIntegrity/Operational` (sucesos 3077/3118), no por
inferencia. No es un problema de este proyecto ni de sus dependencias —es
la política de la máquina bloqueando cualquier binario nuevo sin firmar—,
así que no hay nada que registrar como decisión de arquitectura. El usuario
desactivó Smart App Control (Configuración → Seguridad de Windows;
irreversible sin reinstalar el sistema) para poder seguir. Queda anotado
aquí porque quien repita esta medición en una máquina con Smart App Control
activo se topará con lo mismo.

**Datos de prueba borrados al cerrar la historia:**
`pruebas/hu-05-arranque/` (documento e imágenes, regenerable) y la carpeta
`MDView-HU05` del escritorio (script, copia de `mdview.exe` y
`resultado.txt`, ya copiado arriba).

**Estado de la historia:** verificada. El único criterio pasa por
observación directa, en las condiciones más estrictas de `plan.md` (tres
reinicios completos, uno antes de cada medida).

---

## HU-01 (front-matter-y-html) — Ver el front matter como una tabla de propiedades

Verificado el 2026-09-18, en la misma máquina de desarrollo. `cargo test`
(9 casos: los 5 de `enlaces-e-imagenes` más 4 propios de esta historia sobre
`extract_front_matter`) en verde. Compilado `target\debug\mdview.exe`.

Documentos de prueba: `pruebas/hu-01-front-matter/documento.md` (front
matter de cuatro propiedades, una con dos puntos en el valor — una hora—,
seguido de un encabezado y un párrafo) y
`pruebas/hu-01-front-matter/no-es-front-matter.md` (un párrafo, luego un
`---`/`clave: valor`/`---` que no está en la primera línea del documento).

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-01.1 — Front matter al principio se muestra como tabla de dos columnas, antes de cualquier otro elemento | pasa | Captura: `titulo`/`autor`/`fecha`/`hora` en una tabla, en la parte alta de la ventana. |
| CA-01.2 — Ni `---` ni `clave:` aparecen como texto | pasa | Misma captura: no se ve ningún `---` ni ninguna sintaxis de dos puntos pegada al nombre; solo nombre y valor en sus columnas. |
| CA-01.3 — Las propiedades mantienen el orden del documento | pasa | Misma captura: `titulo`, `autor`, `fecha`, `hora`, en ese orden, igual que en el archivo. |
| CA-01.4 — El encabezado y el párrafo posteriores se siguen mostrando | pasa | Misma captura: «Encabezado tras el front matter» y su párrafo, debajo de la tabla. |
| CA-01.5 — Un `---` que no está en la primera línea no se trata como front matter | pasa | Captura de `no-es-front-matter.md`: sin tabla de propiedades; el primer `---` se ve como regla horizontal bajo el párrafo, y «clave: valor» seguido de `---` sin línea en blanco se muestra como encabezado —un encabezado *setext* de CommonMark, comportamiento del analizador ya existente, no de esta historia. |

**Prueba de regresión.** `cargo test`: 9/9 en verde, incluyendo los 5 casos
previos de `enlaces-e-imagenes` (el cambio no tocó `Inline`/`ImageRef`, y los
tests lo confirman en verde sin haberlos revisado línea a línea de nuevo).

**Nota de proceso — por qué no se usó
`Options::ENABLE_YAML_STYLE_METADATA_BLOCKS` de `pulldown-cmark`.**
Investigado en `plan.md` antes de escribir código: esa opción reconoce un
bloque `---`/`---` en cualquier límite de bloque del documento, no solo en
el primero — confirmado leyendo `scan_metadata_block` en el código fuente
de `pulldown-cmark` 0.13.4, sin ninguna comprobación de posición. RF-12.1
solo cuenta la primera línea, así que se escribió `extract_front_matter`
como un paso previo sobre el texto crudo, antes de pasarlo a
`pulldown_cmark::Parser`. El propio CA-01.5 es la verificación de que esta
elección era necesaria: con la opción de la biblioteca sola, ese documento
de prueba habría mostrado una tabla donde no debía.

**Estado de la historia:** verificada. Los cinco criterios pasan por
observación directa.

---

## HU-02 (front-matter-y-html) — Ver texto de HTML incrustado con el formato equivalente

Verificado el 2026-09-18, en la misma máquina de desarrollo. `cargo test`
(21 casos: los 9 anteriores más 7 de `html::parse_tag` y 5 de la
interpretación de etiquetas en línea) en verde. Compilado
`target\debug\mdview.exe`.

Documento de prueba: `pruebas/hu-02-html-en-linea/documento.md`, un párrafo
que mezcla las ocho etiquetas de esta historia con texto Markdown normal
alrededor, y `img/foto.png` (200×120, generada con `System.Drawing`) para el
`<img>`.

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-02.1 — `<b>`/`<strong>` en negrita | pasa | Captura: «negrita en b» y «negrita en strong» en negrita, igual que el resto del texto en negrita del documento. |
| CA-02.2 — `<i>`/`<em>` en cursiva | pasa | Misma captura: «cursiva en i» y «cursiva en em» en cursiva. |
| CA-02.3 — `<code>` con tipografía monoespaciada y fondo | pasa | Misma captura: «codigo en linea» en monoespaciada con fondo distinguible, igual que el código en línea Markdown ya verificado en HU-02 de `ver-un-documento`. |
| CA-02.4 — `<a href>` distinguible y clicable como un enlace Markdown | **pasa la parte visual; bloqueada la apertura del navegador** | Captura: «enlace desde HTML» subrayado, igual que un enlace Markdown. El clic no se pudo repetir esta vez: `textinputhost.exe` (proceso de Windows para teclado táctil/IME) se reporta al frente y bloquea el clic sobre la ventana de MDView, en todos los intentos —incluido forzar el foco con `SetForegroundWindow` y relanzar la app—, sin relación con el código de esta historia. El mecanismo de clic en sí (`Span.url` → `InteractiveText::on_click` → `app::activate_link`) es el mismo, sin cambios, que CA-02.1 a CA-02.5 de HU-02 en `enlaces-e-imagenes` ya verificaron por observación; lo único nuevo aquí es que el `href` se extrae de un atributo HTML en vez de la sintaxis `[texto](url)`, y eso sí está probado (`inline_html_a_carries_the_href_as_url`). Aun así, este archivo no da un criterio por `pasa` sin haberlo visto: queda pendiente repetir el clic antes de cerrar la feature. |
| CA-02.5 — `<img>` se muestra como la imagen, resuelta contra el directorio del documento | pasa | Captura: la imagen (óvalo verde) se ve en el punto donde está escrita. |
| CA-02.6 — `<br>` produce un salto de línea | pasa | Captura: «y un salto» termina una línea y «de linea, seguido...» empieza la siguiente, dentro del mismo bloque de texto. |
| CA-02.7 — Una etiqueta no listada muestra su texto sin su marcado | pasa | Captura: «texto de span» se ve igual que el texto que lo rodea, sin ningún indicio de la etiqueta `<span class="x">` que lo envolvía. |

**Prueba de regresión.** `cargo test`: 21/21 en verde, incluyendo los 9
casos de HU-01 y de `enlaces-e-imagenes`.

**Nota de proceso — nuevo módulo `html`.** `src/html.rs` interpreta el texto
crudo que `pulldown_cmark` entrega para HTML incrustado
(`Event::InlineHtml`/`Event::Html`) en una sola función, `parse_tag`, que
reconoce un `<tag>`/`</tag>` con sus atributos, sin construir un árbol: siete
tests propios, sin depender de `markdown` ni de GPUI. `markdown::parse_paragraph_inline`
reacciona a una etiqueta reconocida reutilizando exactamente los mismos
campos que ya manejaba para Markdown (`style`, `link`, `Inline::Image`); una
etiqueta no reconocida, abierta o cerrada, no hace nada, lo que por
construcción dispensa el marcado sin tocar el texto que hay dentro (CA-02.7).
El diseño completo, con las alternativas consideradas, está en AD-22.

**Nota de proceso — el trabajo en curso no rompe el límite de una historia
a la vez por descuido.** `historias.md` deja HU-02 «en curso», no
verificada, precisamente por CA-02.4. Seguir con HU-03 a continuación es una
desviación consciente del método de `00-contexto.md` bajo la delegación de
esta sesión, no una que se esconda: queda escrita aquí y en `historias.md`
para que quien la lea después sepa que CA-02.4 se retomó, no que se ignoró.

**Estado de la historia:** en curso. Seis de los siete criterios pasan por
observación directa; CA-02.4 queda con la apertura del navegador bloqueada
por el entorno, a la espera de repetir el clic.

---

## HU-03 (front-matter-y-html) — Ver bloques de HTML incrustado con el formato equivalente

Verificado el 2026-09-18, en la misma máquina de desarrollo. `cargo test`
(27 casos: los 21 anteriores más 6 propios de esta historia) en verde.
Compilado `target\debug\mdview.exe`.

Documento de prueba: `pruebas/hu-03-bloques-html/documento.md`, con las
cinco etiquetas (`<p>`, `<ul>`/`<li>` de dos elementos, `<hr>`,
`<div align="center">`) entre un encabezado «Antes» y uno «Despues».

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-03.1 — `<p>` como párrafo | pasa | Captura: «Un parrafo escrito en HTML.» se ve como cualquier otro párrafo del documento. |
| CA-03.2 — `<ul>`/`<li>` como lista no ordenada de dos elementos | pasa | Misma captura: «Elemento uno» y «Elemento dos», cada uno con su marcador `•`. |
| CA-03.3 — `<hr>` como regla horizontal | pasa | Misma captura: línea fina que recorre el ancho de la columna, bajo la lista. |
| CA-03.4 — `<div align="center">` con su contenido centrado | pasa | Misma captura: «Texto centrado en HTML» centrado horizontalmente en la columna, a diferencia del resto del texto, alineado a la izquierda. |
| CA-03.5 — El contenido antes y después se sigue mostrando | pasa | Misma captura: «Antes» y «Despues» visibles, ninguno de los cinco bloques HTML trunca el documento. |

**Prueba de regresión.** `cargo test`: 27/27 en verde, incluyendo los 21
casos previos.

**Nota de proceso — cómo se investigó antes de escribir código.** Antes de
diseñar `parse_html_block_tokens`, se sondeó con un test desechable cómo
entrega `pulldown-cmark` 0.13.4 el contenido de un `Tag::HtmlBlock`:
`<p>Hola <b>mundo</b></p>` llega como **una sola cadena por línea de
código fuente** (`Event::Html`), etiquetas y texto mezclados, a diferencia
del HTML en línea de HU-02, que llega una etiqueta por evento. De ahí que
`html::tokenize` —nuevo, en el mismo módulo que `parse_tag`— haga falta:
recorre esa cadena buscando `<`/`>` y separa texto de etiquetas a mano.
También se confirmó que varias líneas HTML seguidas sin línea en blanco
entre ellas comparten un único `Tag::HtmlBlock` —de ahí que
`parse_html_block_tokens` recorra en bucle, sin asumir una etiqueta de
nivel superior por bloque (cubierto por
`several_html_block_level_tags_in_a_row_all_produce_their_own_block`).

**Nota de proceso — nuevo `Block::Centered`.** `<div align="center">` no
tiene equivalente en CommonMark —es el primer `Block` de este proyecto sin
contrapartida Markdown—, así que RF-13.1 no puede pedir «el mismo formato
que su equivalente»: aquí se decidió que equivale a centrar su contenido
horizontalmente. `render::render_centered` reutiliza la misma separación de
texto/imagen que `render_paragraph`, con `.text_center()` en vez de
`.w_full()` en cada bloque de texto —`w_full()` no deja nada que centrar,
ocupa toda la columna igual—.

**Estado de la historia:** verificada. Los cinco criterios pasan por
observación directa.

---

## HU-04 (front-matter-y-html) — Ver una tabla de HTML incrustada con el formato equivalente

Verificado el 2026-09-18, en la misma máquina de desarrollo. `cargo test`
(28 casos: los 27 anteriores más 1 propio de esta historia) en verde.
Compilado `target\debug\mdview.exe`.

Documento de prueba: `pruebas/hu-04-tabla-html/documento.md`, una tabla
HTML de cabecera (`<th>`) y dos filas (`<td>`), entre dos encabezados.

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-04.1 — La tabla se muestra como rejilla, cabecera distinguible | pasa | Captura: fila «Nombre»/«Valor» con fondo gris y negrita, igual que la cabecera de una tabla Markdown. |
| CA-04.2 — Cada celda en su fila y columna correctas | pasa | Misma captura: «uno»/«1» y «dos»/«2», cada valor bajo su columna. |
| CA-04.3 — El contenido antes y después se sigue mostrando | pasa | Misma captura: «Antes» y «Despues» visibles. |

**Prueba de regresión.** `cargo test`: 28/28 en verde.

**Nota de proceso.** `parse_html_table` reutiliza `collect_html_inline` de
HU-02/HU-03 para el contenido de cada celda, aplanado a `Span` con la misma
función que ya usaban las celdas de tabla Markdown
(`flatten_inlines_to_spans`, factorizada de `parse_inline_spans` en este
mismo cambio): una tabla HTML y una tabla Markdown terminan en el mismo
`Block::Table`, sin código nuevo en `render`. La cabecera se detecta por
que todas las celdas de la primera fila sean `<th>`; una tabla con más de
una fila de cabecera —ningún documento de prueba la tiene— trataría la
segunda como fila de cuerpo.

**Estado de la historia:** verificada. Los tres criterios pasan por
observación directa.

---

## HU-05 (front-matter-y-html) — Ver una sección plegable de HTML incrustada

Verificado el 2026-09-18, en la misma máquina de desarrollo. `cargo test`
(31 casos: los 28 anteriores más 3 propios de esta historia) en verde.
Compilado `target\debug\mdview.exe`.

Documento de prueba: `pruebas/hu-05-details/documento.md`, un `<details>`
con `<summary>` y un párrafo dentro, entre dos párrafos normales.

| Criterio | Resultado | Evidencia |
| --- | --- | --- |
| CA-05.1 — El `<summary>` se distingue visualmente del contenido | pasa | Captura: «Ver mas detalles» en negrita, el párrafo que sigue en peso normal. |
| CA-05.2 — El contenido de `<details>` es legible sin ninguna acción del usuario | pasa | Misma captura: el párrafo de contenido se ve de inmediato, sin haber pulsado nada — la decisión de AD-24, confirmada por observación, no solo por diseño. |
| CA-05.3 — El párrafo anterior y el posterior se siguen mostrando | pasa | Misma captura: «Parrafo antes del details.» y «Parrafo despues del details.» visibles. |

**Prueba de regresión.** `cargo test`: 31/31 en verde.

**Nota de proceso.** `parse_html_blocks_until` (HU-03/HU-04) ganó un
parámetro `stop_name` para esta historia: `<details>` es la primera
etiqueta cuyo contenido es, a su vez, una secuencia de bloques —no texto en
línea, no una lista de `<li>`—, así que se resuelve llamando a la misma
función otra vez en vez de escribir un bucle nuevo. El diseño completo,
con las alternativas de cómo mostrar `details`/`summary`, está en AD-24:
la decisión —siempre visible, sin plegado— ya la había anotado `plan.md`
como opción por defecto antes de escribir código; aquí solo se confirma y
se verifica por observación.

**Estado de la historia:** verificada. Los tres criterios pasan por
observación directa.

Con esta historia, las cuatro de RF-13.1 (HU-02 a HU-05) están construidas;
queda pendiente el matiz de CA-02.4 anotado en HU-02, antes de cerrar la
feature.
