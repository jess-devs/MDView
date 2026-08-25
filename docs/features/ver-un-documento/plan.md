# Plan — ver-un-documento

> Estado: borrador
> Última actualización: 2026-08-20
> Modo: new-project

## Orden de las features del proyecto

Esta es la primera de cuatro. Las otras tres están nombradas y sin artefactos
hasta que les llegue el turno; sus requisitos ya están repartidos y ese reparto
está en `requisitos.md`.

1. **`ver-un-documento`** — esta. Sin ella no hay nada que observar.
2. **`contenido-enriquecido`** — imágenes locales, front matter, subconjunto de
   HTML, enlaces externos y ausencia de tráfico de red. Va antes que las
   pestañas porque un documento mal renderizado en tres pestañas sigue siendo un
   documento mal renderizado.

   **Partida en dos el 2026-08-24**, porque sus historias pasaban del techo de
   seis: `enlaces-e-imagenes` (RF-11, RF-15, RNF-02) y después
   `front-matter-y-html` (RF-12, RF-13). El motivo del corte y el del orden
   están en `../enlaces-e-imagenes/plan.md`. Las features del proyecto son
   cinco, no cuatro; esa lista está allí.
3. **`varios-documentos-en-pestanas`** — varias rutas a la vez, cambiar y cerrar
   pestañas, y enlaces a otros `.md`.
4. **`integracion-con-windows`** — asociación de `.md`, doble clic e instancia
   única. Va la última porque necesita que las pestañas existan: el
   comportamiento acordado es que el archivo nuevo entre como pestaña activa.

Las funciones aplazadas de `01-alcance.md` darán lugar a dos features más cuando
se decida construirlas: una de **distribución** —instalador, integración en el
PATH y versión portable, que es donde aterriza de verdad el objetivo
multiplataforma— y otra de **edición**, que agrupa la edición con confirmación y
ver el Markdown en crudo, porque comparten la misma maquinaria.

## Orden de las historias

1. **HU-01 — Ver un documento pasándole su ruta.** Primera porque hasta que
   exista no hay ventana en la que observar nada: las demás se verifican mirando
   lo que ella pone en pantalla. HU-07 tampoco depende de nada y podría ir aquí,
   pero HU-01 es la que entrega el valor de la feature.
2. **HU-02 — Leer un documento con el formato que el autor escribió.** El grueso
   del trabajo de la feature. Va inmediatamente después porque es lo que
   convierte una ventana con texto en un visor.
3. **HU-03 — Leer bloques de código sin que estorben.** Después de HU-02 y no
   dentro de ella porque los bloques de código tienen reglas propias, y porque
   separarla mantiene HU-02 en un tamaño verificable de una sentada.
4. **HU-04 — Ver el documento con el tema del sistema.** Después de HU-02 y
   HU-03, porque CA-04.3 exige comprobar la legibilidad de los elementos de
   ambas: antes de que existan, no hay nada que mirar.
5. **HU-05 — Enterarme de que el archivo no se puede mostrar.** Podría ir en
   cualquier momento tras HU-01. Se coloca aquí para no interrumpir el trabajo de
   renderizado, que es continuo.
6. **HU-07 — Entender qué es esto si lo abro sin un archivo.** Junto a HU-05
   porque las dos tratan lo que ocurre cuando no hay documento que mostrar, y
   conviene resolverlas seguidas: comparten la pregunta de qué se ve cuando la
   ventana no tiene contenido.
7. **HU-06 — Ver el documento sin esperar.** La última, porque mide la feature
   terminada. Medir antes daría una cifra de algo que aún no es el producto.

## Dependencias

| Historia | Depende de | Motivo |
| --- | --- | --- |
| HU-02 | HU-01 | Necesita una ventana donde mostrar el documento. |
| HU-03 | HU-01 | Igual que la anterior. |
| HU-04 | HU-02, HU-03 | CA-04.3 comprueba los elementos de ambas en los dos temas. |
| HU-05 | HU-01 | Necesita que exista el camino de apertura para poder fallar en él. |
| HU-07 | Ninguna | Solo necesita que la aplicación sepa abrir una ventana. Podría hacerse la primera; se coloca junto a HU-05 por afinidad, no por dependencia. |
| HU-06 | Todas las anteriores | Mide el tiempo hasta el documento visible con el renderizado ya completo. |

HU-02 y HU-03 no dependen entre sí, y HU-07 no depende de nada. Con el límite de
trabajo en curso de 1 acordado en `00-contexto.md` da igual: se hacen de una en
una de todos modos.

## Método de medición de HU-06

CA-06.1 no se puede comprobar con un cronómetro ni con `Measure-Command`, porque
ninguno de los dos sabe cuándo el documento está **visible**: el proceso puede
haber terminado de arrancar mucho antes de que haya un fotograma pintado con
texto dentro.

El método acordado es instrumentar el ejecutable para que registre dos instantes
—el de entrada al programa y el del primer fotograma que ya contiene el documento
renderizado— y emita la diferencia. El arranque en frío se fuerza reiniciando el
equipo antes de la primera medición; las tres medidas se toman consecutivas y se
anotan **las tres**, incluida la peor, no solo la mejor.

Si al llegar a HU-06 se descubre que instrumentar así no es posible, el resultado
de CA-06.1 es `bloqueado` y se dice qué falta para poder observarlo. No es `pasa`
porque la aplicación parezca rápida.

## Riesgos

- **GPUI es pre-1.0.** Una actualización de versión puede romper la compilación en
  mitad de una historia. Mitigación: fijar la versión exacta en `Cargo.toml` y no
  actualizarla con una historia abierta. Queda como decisión a registrar en Fase 4.
- **HU-06 mide al final.** Si el objetivo de 1 segundo no se cumple, se descubre
  con toda la feature construida. Mitigación: tomar una medida informal desde
  HU-01, sin registrarla como verificación, solo para detectar pronto una
  desviación grande.
- **CA-04.1 y CA-04.2 exigen cambiar el tema de Windows** durante la
  verificación, lo que afecta a todo el escritorio del usuario. Hay que devolver
  el sistema al tema que tenía antes al terminar, y decirlo.
- **CA-05.3 exige un archivo sin permiso de lectura**, que hay que crear con una
  regla de acceso y borrar después. Ver la sección siguiente.
- **El ancho máximo de lectura de CA-01.5 no tiene valor fijado.** El requisito
  RF-20 dice deliberadamente que ese número es una decisión de diseño, no un
  requisito. Hay que elegirlo y registrarlo en `decisiones.md` al implementar
  HU-01; sin él, CA-01.5 se puede observar igual —la columna deja de crecer o no
  lo hace—, pero nadie sabrá después por qué se eligió ese ancho.
- **AD-02 se tomó sin comprobarla.** Se decidió analizar el Markdown con
  `pulldown-cmark` propio en lugar de usar el renderizador de `gpui-component`,
  sin haber mirado cuánto se puede moldear ese renderizador. Antes de empezar
  HU-02 conviene dedicarle un rato: si admitiera las reglas de RF-12 y RF-13,
  AD-02 habría que marcarla superada, no defenderla por antigüedad.
- **El tamaño de `gpui-component` puede pesar en RNF-01.** Son unas 48.000 líneas
  de dependencia y el objetivo es arrancar en menos de un segundo. Mitigación: la
  medida informal temprana desde HU-01 que ya recoge el riesgo anterior, y el
  conjunto mínimo de *features* de Cargo acordado en AD-01.

## Datos de prueba y limpieza

Esta feature necesita crear archivos para poder verificarse. Todos se crean
dentro del propio repositorio, en una carpeta de pruebas, y se borran al cerrar
la historia que los usó:

- Un `.md` con todos los elementos de CA-02.1 a CA-02.10 y de CA-03.1 a CA-03.6.
  Debe incluir un bloque de código con una línea deliberadamente más ancha que la
  ventana, o CA-03.5 y CA-03.6 no se pueden observar.
- Un `.md` de aproximadamente 50 KB para HU-06.
- Una imagen con la extensión cambiada a `.md`, para CA-05.2.
- Un archivo con el permiso de lectura denegado, para CA-05.3. Es el único que
  toca la configuración del sistema y no solo el disco: hay que retirar la regla
  de acceso además de borrar el archivo.
- El fichero de medidas que genera `MDVIEW_TIMING` según AD-07, para HU-06.

El tema de Windows alterado para CA-04.1 y CA-04.2 se devuelve a su valor
original. No es un dato de prueba, pero es estado del sistema que la verificación
modificó, y la regla es la misma: se deja como estaba.
