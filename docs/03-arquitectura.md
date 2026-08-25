# Arquitectura

> Estado: borrador
> Última actualización: 2026-08-20
> Modo: new-project

## Visión general

MDView es un único ejecutable de escritorio. Al arrancar lee sus argumentos: si
hay una ruta, carga ese archivo y lo muestra; si no la hay, muestra la ventana de
estado vacío de RF-19. El texto del archivo se analiza como Markdown y se
convierte en un árbol de elementos propio de la aplicación, y ese árbol es lo que
se dibuja en pantalla.

La pieza que decide el aspecto es GPUI, a través de la biblioteca de componentes
`gpui-component`. La pieza que decide el contenido es un analizador de Markdown
propio construido sobre `pulldown-cmark`. Están separadas a propósito: leer un
archivo y entender su Markdown no requiere una tarjeta gráfica, y mantener esa
frontera es lo que permite razonar sobre el contenido sin arrancar una ventana.

## Fronteras

Cuatro módulos. La regla que los ordena es que **GPUI vive en uno solo**.

| Módulo | De qué es dueño | Qué no puede conocer |
| --- | --- | --- |
| `document` | Leer el archivo del disco, decidir si es texto, producir el documento o el error de RF-17. | GPUI, `gpui-component`, y qué aspecto tendrá nada. |
| `markdown` | Convertir el texto en un árbol de elementos propio: encabezados, listas, tablas, bloques de código. | GPUI y `gpui-component`. Tampoco sabe de colores, tamaños ni fuentes. |
| `render` | Traducir el árbol de elementos a componentes en pantalla, y el tema. | El sistema de archivos y la sintaxis de Markdown. Recibe elementos, no texto. |
| `app` | Arranque, argumentos, estado de la aplicación, ventana. | Los detalles de los tres anteriores; los coordina, no los sustituye. |

La consecuencia que importa: si algún día se cambia `gpui-component` por otra
cosa, o se añade Linux, el trabajo cae dentro de `render` y `app`. `document` y
`markdown` no se tocan. Si en cambio se descubre que el árbol de elementos tiene
que conocer detalles de dibujo para funcionar, esa frontera estaba mal puesta y
hay que registrarlo como decisión nueva, no ir agujereándola en silencio.

## Convenciones

- **El idioma del código es el inglés**: nombres de módulos, tipos, funciones y
  comentarios. El idioma de la documentación es el español. Son dos públicos
  distintos: el código lo leen herramientas y bibliotecas de terceros, los
  documentos los lee quien decide.
- **Los errores suben como valores, no como pánicos.** `document` devuelve el
  error de lectura; `app` decide que eso se muestra como el aviso temporal de
  RF-17. Un `panic!` con un archivo que el usuario eligió mal incumpliría
  CA-05.5, que exige que la aplicación no termine de forma abrupta.
- **Las versiones de las dependencias de interfaz se fijan exactas.** Ver AD-03.
- **Nada de red.** RNF-02 no es una recomendación: ninguna dependencia que abra
  conexiones entra en el proyecto sin una decisión registrada que lo justifique.

## Decisiones

Índice de todas las decisiones registradas en el proyecto, de la más antigua a la
más reciente. Cada una vive completa en el `decisiones.md` de su feature. Este
índice se actualiza en el mismo turno en que se escribe la decisión: un índice al
que le faltan entradas es peor que no tener índice.

| Id | Decisión | Dónde vive | Estado |
| --- | --- | --- | --- |
| AD-01 | Adoptar `gpui-component` como capa de componentes de interfaz | `features/ver-un-documento/decisiones.md` | activa |
| AD-02 | Analizar el Markdown con `pulldown-cmark` propio, no con el renderizador de `gpui-component` | `features/ver-un-documento/decisiones.md` | activa |
| AD-03 | Fijar versiones exactas de `gpui` y `gpui-component` | `features/ver-un-documento/decisiones.md` | activa |
| AD-04 | Separar el proyecto en cuatro módulos con GPUI confinado a uno | `features/ver-un-documento/decisiones.md` | activa |
| AD-05 | El solo lectura es un estado explícito del modelo, no la ausencia de edición | `features/ver-un-documento/decisiones.md` | activa |
| AD-06 | Compilar como aplicación de ventanas de Windows, sin consola | `features/ver-un-documento/decisiones.md` | activa |
| AD-07 | Medir el arranque con dos marcas de tiempo volcadas a un fichero, activadas por variable de entorno | `features/ver-un-documento/decisiones.md` | activa |
| AD-08 | Ancho máximo de lectura: 720 px | `features/ver-un-documento/decisiones.md` | activa |
| AD-09 | Valores tipográficos y visuales de HU-02 (tamaños, marcadores, cita, tabla, código en línea) | `features/ver-un-documento/decisiones.md` | activa |
| AD-10 | Scroll horizontal de bloques de código: `ScrollHandle` propio por bloque, no `overflow_x_scrollbar()` | `features/ver-un-documento/decisiones.md` | activa |
| AD-11 | Detectar el tema con `window.appearance()` de GPUI, con resincronía en caliente | `features/ver-un-documento/decisiones.md` | activa |
| AD-12 | Avisos de error con `Notification` de `gpui-component`; `document::LoadError` clasifica, `app` redacta el mensaje | `features/ver-un-documento/decisiones.md` | activa |
| AD-13 | Perfil de release con LTO, `codegen-units = 1` y `strip` (~16% más rápido en caliente, ~2.6× más lento de compilar) | `features/ver-un-documento/decisiones.md` | activa |
| AD-14 | Umbral de RNF-01 renegociado de 1 s a 1,2 s en arranque en frío (CA-06.1 fallaba por 55 ms) | `features/ver-un-documento/decisiones.md` | activa |
| AD-15 | Arreglar la causa compartida de la pérdida de contenido en `parse_blocks` (enlaces, imágenes y bloques HTML) | `features/enlaces-e-imagenes/decisiones.md` | activa |
| AD-16 | Representar el enlace como un campo `url` en `Span`, no como un tipo nuevo | `features/enlaces-e-imagenes/decisiones.md` | activa |
| AD-17 | Primeros tests del proyecto: módulo `#[cfg(test)]` junto al código, sin framework | `features/enlaces-e-imagenes/decisiones.md` | activa |
| AD-18 | Cómo se activa un enlace al pulsarlo: `InteractiveText` con un contador de identificadores por documento | `features/enlaces-e-imagenes/decisiones.md` | activa |
| AD-19 | Con qué se abre un enlace externo: crate `open`, función `that_detached`, sin fijar versión exacta | `features/enlaces-e-imagenes/decisiones.md` | activa |
