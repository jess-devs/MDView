# Requisitos — enlaces-e-imagenes

> Estado: borrador
> Última actualización: 2026-08-24
> Modo: new-feature

Esta feature cubre lo que un documento referencia **fuera de sí mismo**: los
enlaces que apuntan a otro sitio y las imágenes que viven en el disco junto al
`.md`. Es la segunda de las features del proyecto y la primera de las dos en
que se partió la que `ver-un-documento/plan.md` llamaba `contenido-enriquecido`;
la otra mitad, `front-matter-y-html`, cubre RF-12 y RF-13 y va después.

Cada requisito refina uno de `02-requisitos.md`. Un refinamiento **acota**: aquí
no aparece nada que su requisito padre no tuviera ya. Los términos usados están
en el glosario de `00-contexto.md`.

## Funcionales

### RF-08.2 — Renderizado de enlaces

Refina: RF-08
El sistema debe mostrar el texto de un enlace dentro del párrafo que lo contiene,
distinguible del texto que lo rodea, sin mostrar su URL en el cuerpo del
documento y sin que el contenido que sigue al enlace deje de mostrarse.

RF-08.1, en `ver-un-documento/requisitos.md`, enumeró los elementos de CommonMark
que aquella feature cubría y dejó fuera los enlaces. Este requisito recoge esa
parte pendiente del mismo padre; no amplía RF-08.

### RF-11.1 — Imágenes por ruta relativa al documento

Refina: RF-11
El sistema debe mostrar las imágenes que el documento referencie mediante una
ruta relativa, resuelta respecto al directorio del archivo `.md` que se está
viendo y no respecto al directorio desde el que se invocó la aplicación.

Cuando la imagen referenciada no se pueda mostrar —porque el archivo no exista,
no se pueda leer o no sea una imagen que el sistema sepa decodificar— debe
mostrarse en su lugar el texto alternativo que el documento le dio, y el resto
del documento debe seguir viéndose. Esto no amplía RF-11: fija qué ocurre en el
caso de fallo, que el padre no decidía.

### RF-15.1 — Apertura de enlaces externos en el navegador

Refina: RF-15
Al activarse un enlace `http` o `https`, el sistema debe hacer que se abra en el
navegador predeterminado del sistema operativo, sin abrirlo dentro de MDView.

Los enlaces que apuntan a otro archivo `.md` son RF-14 y pertenecen a la feature
`varios-documentos-en-pestanas`: aquí no se contemplan.

## No funcionales

### RNF-02.1 — Ausencia de tráfico de red propio al mostrar un documento

Refina: RNF-02
Mientras abre y muestra un documento, incluidas sus imágenes, el proceso de
MDView no debe iniciar ninguna conexión de red. El umbral es cero conexiones
salientes atribuibles al proceso. Pedirle al sistema operativo que abra un enlace
en el navegador, según RF-15.1, no cuenta: la conexión la hace el navegador, que
es otro proceso.

### RNF-01.2 — Tiempo hasta el documento visible, con imágenes

Refina: RNF-01
Con un documento de aproximadamente 50 KB que referencie imágenes locales, desde
que se invoca la aplicación con su ruta hasta que el documento está visible deben
transcurrir **menos de 1,2 segundos**, medido en arranque en frío.

El umbral es el de RNF-01 tras AD-14, no uno nuevo. La composición exacta del
documento de prueba es método de medida y está en `plan.md`, no aquí.

### RNF-03.1 — Plataforma de ejecución

Refina: RNF-03
El sistema debe ejecutarse en Windows 11 de 64 bits.

## Decidido en ausencia del usuario

El usuario delegó la conducción de esta feature el 2026-08-24 para volver unas
horas después. Lo que sigue se decidió sin él y está señalado para que pueda
revisarlo, no para que pase inadvertido.

- **Una imagen con URL `http` o `https` no se descarga.** `01-alcance.md` excluye
  las imágenes alojadas en la web —«implicaría que la aplicación hiciera
  peticiones de red»— y RNF-02 fija cero conexiones. De ahí se sigue que no se
  pide, pero no de dónde se sigue qué se ve en su lugar. Se resuelve como el caso
  de fallo de RF-11.1: se muestra el texto alternativo. [inferido]

## Requisitos del sistema que esta feature no cubre

Se listan para que quede claro que la ausencia es deliberada: RF-02, RF-03,
RF-04, RF-05, RF-06, RF-07, RF-12, RF-13 y RF-14. RF-12 y RF-13 van en
`front-matter-y-html`; el resto, en `varios-documentos-en-pestanas` y en
`integracion-con-windows`, según el reparto de `ver-un-documento/plan.md`.
