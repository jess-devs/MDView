# Plan — enlaces-e-imagenes

> Estado: borrador
> Última actualización: 2026-08-24
> Modo: new-feature

## Cómo se aprobó esta feature

El usuario aprobó el 2026-08-24 partir en dos la feature que
`ver-un-documento/plan.md` llamaba `contenido-enriquecido`, y a continuación
delegó la conducción del trabajo para volver unas horas después. Los gates de
las fases siguientes —requisitos, historias, este plan y las decisiones que
salgan al construir— se han dado por aprobados en su ausencia. Lo que se decidió
sin él está señalado: la sección «Decidido en ausencia del usuario» de
`requisitos.md`, y esta misma sección.

## Orden de las features del proyecto

Actualiza la lista de `ver-un-documento/plan.md`, que preveía cuatro features y
ahora son cinco:

1. **`ver-un-documento`** — cerrada, sus siete historias verificadas.
2. **`enlaces-e-imagenes`** — esta. RF-08.2, RF-11.1, RF-15.1, RNF-01.2 y
   RNF-02.1.
3. **`front-matter-y-html`** — RF-12 y RF-13. La otra mitad de lo que iba a ser
   `contenido-enriquecido`.
4. **`varios-documentos-en-pestanas`** — varias rutas a la vez, cambiar y cerrar
   pestañas, y RF-14: los enlaces a otros `.md`.
5. **`integracion-con-windows`** — asociación de `.md`, doble clic e instancia
   única. Sigue siendo la última porque necesita que las pestañas existan.

**Por qué se partió en dos.** Contando las historias que exigían RF-11, RF-12,
RF-13, RF-15 y RNF-02 salían ocho o nueve, por encima del techo de seis que
marca la lista de validación. El corte separa lo que el documento referencia
fuera de sí mismo —enlaces e imágenes, que necesitan el disco y el navegador— de
lo que es marcado que el analizador todavía no entiende —front matter y HTML,
que es trabajo dentro de `markdown` y sin entrada/salida nueva.

**Por qué esta va primero.** RF-13 exige mostrar `a` e `img` «con el formato
equivalente al del elemento Markdown correspondiente». El equivalente Markdown
tiene que existir antes de que se pueda exigir equivalencia con él.

## Orden de las historias

1. **HU-01 — Leer un documento con enlaces sin perder nada.** Primera porque
   arregla una pérdida de contenido que hoy ocurre (ver «Defecto observado») y
   porque hasta que el árbol de elementos tenga enlaces no hay nada que pulsar.
2. **HU-02 — Abrir un enlace externo en el navegador.** Inmediatamente después:
   necesita que HU-01 haya puesto los enlaces en el documento.
3. **HU-03 — Ver las imágenes que el documento referencia.** No depende de las
   dos anteriores, pero se coloca después porque introduce lo único de esta
   feature que vuelve a tocar el disco, y conviene no mezclarlo con el trabajo
   de enlaces.
4. **HU-04 — Saber que MDView no se conecta a nada.** Después de HU-03 porque las
   imágenes son lo único de esta feature que podría llegar a pedir algo por red:
   antes de que existan, el criterio se cumpliría por no haber nada que
   comprobar.
5. **HU-05 — Seguir viendo el documento sin esperar.** La última, por el mismo
   motivo que HU-06 en `ver-un-documento`: mide la feature terminada. Medir antes
   daría una cifra de algo que aún no es el producto.

## Dependencias

| Historia | Depende de | Motivo |
| --- | --- | --- |
| HU-01 | Ninguna | Trabaja sobre el documento que `ver-un-documento` ya sabe mostrar. |
| HU-02 | HU-01 | No se puede activar un enlace que el documento no contiene. |
| HU-03 | Ninguna | Podría ir la primera; se coloca tras HU-02 por afinidad de trabajo, no por dependencia. |
| HU-04 | HU-03 | CA-04.1 y CA-04.2 exigen un documento con una imagen por URL, que solo tiene sentido cuando las imágenes se muestran. |
| HU-05 | HU-03 | Mide el tiempo con las imágenes ya cargándose. |

Con el límite de trabajo en curso de 1 acordado en `00-contexto.md`, las
historias se hacen de una en una aunque dos de ellas no dependan de nada.

## Defecto observado antes de empezar

Comprobado el 2026-08-24 ejecutando `src/markdown.rs` aislado —el módulo es puro
y no depende de GPUI— sobre dos entradas:

| Entrada | Lo que se obtiene |
| --- | --- |
| `# uno`, un bloque `<div>`, `# dos` | solo se muestra «uno» |
| `un [enlace](url) y ![alt](ruta) fin` | se muestran «un » y «enlace», en dos párrafos |

Un enlace hace desaparecer el resto del párrafo y del documento, y lo parte en
dos párrafos. Un bloque HTML hace desaparecer el resto del documento. La causa es
la misma en los dos casos y está en una sola línea: `parse_blocks` corta al ver
cualquier `End`, y los `End` de `Link`, `Image` y `HtmlBlock` no los consume
nadie, porque no hay ninguna rama que abra esas etiquetas.

Esto **no contradice** lo verificado en `ver-un-documento`: los diez criterios de
HU-02 enumeran los elementos que se comprobaron y ninguno menciona enlaces ni
HTML, así que el documento de prueba de aquella historia no los llevaba. El
`pasa` registrado sigue siendo honesto. Lo que hay es un hueco, no un error.

**Consecuencia para quien implemente HU-01:** el arreglo va en la causa
compartida, no en un caso especial para enlaces. Los bloques HTML dejarán
probablemente de truncar el documento como efecto colateral, pero eso **no se
verifica aquí**: RF-13 pertenece a `front-matter-y-html` y allí se observará.

## Método de observación de HU-04

CA-04.1 no se puede comprobar mirando la ventana. El método es:

1. Preparar el documento de prueba con al menos un enlace `http`, un enlace
   `https` y una imagen referenciada por una URL `http`.
2. Lanzar MDView con ese documento y quedarse con el identificador del proceso.
3. Desde el arranque y hasta unos segundos después de que el documento esté
   visible, muestrear repetidamente las conexiones y los extremos abiertos que el
   sistema atribuye a ese proceso, con `Get-NetTCPConnection -OwningProcess` y
   `Get-NetUDPEndpoint -OwningProcess`.
4. El criterio pasa si ningún muestreo devuelve nada.

**Límite del método, que hay que escribir junto al resultado:** es un muestreo,
no una captura continua. Una conexión que se abriera y se cerrara entre dos
muestras no se vería. Da confianza razonable, no demostración. Si al ejecutarlo
se juzga que ese límite es demasiado grande para llamarlo `pasa`, el resultado es
`bloqueado` y se dice qué haría falta —una captura de paquetes filtrada por
proceso— en lugar de rebajar el criterio.

CA-04.3 se observa repitiendo el muestreo sobre el proceso del navegador después
de pulsar el enlace: sirve para demostrar que el método **sabe ver** tráfico
cuando lo hay. Un método que no distingue nada de nada no prueba nada.

## Documento de prueba de HU-05

El mismo de `ver-un-documento` —`pruebas/documento-50kb.md`, regenerable, unos
50 KB— con al menos tres imágenes locales intercaladas, de tamaño parecido al de
una captura de pantalla real (del orden de 200 KB cada una), no miniaturas. Se
regenera para medir y se borra después, como se hizo en HU-06.

El método de medición es el de `ver-un-documento/plan.md`: instrumentación con
`MDVIEW_TIMING` según AD-07, ejecutable del perfil de AD-13, reinicio del equipo
antes de cada una de las tres medidas, y se anotan las tres, incluida la peor.

## Riesgos

- **El margen de RNF-01 es de 145 ms.** Las tres medidas de CA-06.1 fueron 1055,
  755 y 965 ms contra un umbral de 1,2 s. Decodificar imágenes antes del primer
  fotograma puede consumir ese margen entero. Mitigación: tomar la medida
  informal en cada historia, como se hizo desde HU-01 en `ver-un-documento`, para
  no descubrirlo en HU-05 con la feature entera construida. Si HU-05 falla, la
  decisión no es volver a subir el umbral: es decidir si las imágenes pueden
  cargarse después del primer fotograma, y registrarlo.
- **`document` no conserva la ruta del archivo.** Hoy `document::load` devuelve
  solo el texto. RF-11.1 exige resolver las rutas relativas respecto al
  directorio del `.md`, así que esa ruta tiene que llegar hasta donde se
  resuelvan las imágenes. Dónde vive es una decisión que hay que registrar al
  implementar HU-03, no un detalle: toca la frontera entre `document`,
  `markdown` y `render` descrita en `03-arquitectura.md`.
- **Ninguna dependencia nueva entra sin decisión registrada.** RF-15.1 apuntaba
  al crate `open`, ya anunciado al final de `ver-un-documento/decisiones.md`, y
  la decodificación de imágenes puede pedir otra. `03-arquitectura.md` dice que
  nada que abra conexiones entra sin justificarlo; una biblioteca de imágenes que
  sepa cargar por URL cumple la letra y rompe el espíritu de RNF-02.1. Hay que
  mirarlo antes de añadirla, no después.
- **AD-02 sigue sin comprobarse.** `ver-un-documento/plan.md` avisó de que se
  eligió analizar el Markdown con `pulldown-cmark` propio sin mirar cuánto se
  puede moldear el renderizador de `gpui-component`. Aquel aviso apuntaba a
  RF-12 y RF-13, que ahora viven en `front-matter-y-html`. El riesgo viaja con
  ellos: se atiende allí, no aquí.
