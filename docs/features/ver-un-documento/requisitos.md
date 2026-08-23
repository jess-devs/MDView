# Requisitos — ver-un-documento

> Estado: borrador
> Última actualización: 2026-08-20
> Modo: new-project

Esta feature cubre lo mínimo que ya sirve: pasar la ruta de un archivo `.md` a la
aplicación y ver ese documento renderizado, con buen aspecto y sin espera
perceptible. Once requisitos, seis de ellos sobre lo que se ve en pantalla.
Todavía no hay pestañas, ni imágenes, ni doble clic en el Explorador.

Cada requisito refina uno de `02-requisitos.md`. Un refinamiento **acota**: aquí
no aparece nada que su requisito padre no tuviera ya. Los términos usados están
en el glosario de `00-contexto.md`.

## Funcionales

### RF-01.1 — Apertura de un archivo desde la línea de comandos

Refina: RF-01
El sistema debe abrir el archivo cuya ruta se le pase como único argumento al
invocarlo. El caso de varias rutas en una misma invocación corresponde a la
feature `varios-documentos-en-pestanas` y aquí no se contempla.

### RF-08.1 — Renderizado de CommonMark

Refina: RF-08
El sistema debe mostrar con formato los encabezados de nivel 1 a 6, los párrafos,
el énfasis, la negrita, las listas ordenadas y no ordenadas con anidamiento, las
citas, las reglas horizontales y el código en línea.

### RF-09.1 — Renderizado de los añadidos de GFM

Refina: RF-09
El sistema debe mostrar con formato las tablas, las listas de tareas y el texto
tachado.

### RF-10.1 — Bloques de código sin coloreado

Refina: RF-10
Los bloques de código deben mostrarse en tipografía monoespaciada, sobre un fondo
distinguible del resto del documento y sin colorear las palabras según el
lenguaje. Una línea más ancha que la ventana se conserva entera y el bloque se
desplaza horizontalmente por sí solo, sin arrastrar al resto del documento.

### RF-16.1 — Desplazamiento del documento

Refina: RF-16
El sistema debe permitir desplazarse verticalmente por un documento más largo que
la ventana.

### RF-17.1 — Archivo que no se puede mostrar

Refina: RF-17
Si el archivo indicado no existe, no se puede leer o no es texto, el sistema no
debe mostrar documento alguno, y debe mostrar un aviso temporal que identifique
el archivo y la causa.

El requisito padre dice «no debe abrir una pestaña», redacción que presupone un
mundo con pestañas. En esta feature no las hay, y el comportamiento observable
es el mismo: no se muestra documento y aparece el aviso. Cuando lleguen las
pestañas, el requisito no cambia de significado.

### RF-18.1 — Tema claro u oscuro

Refina: RF-18
El sistema debe adoptar, al arrancar, el tema claro u oscuro que tenga
configurado Windows.

### RF-19.1 — Arranque sin ningún archivo

Refina: RF-19
Si se invoca la aplicación sin pasarle ninguna ruta, el sistema debe mostrar una
ventana que indique que MDView abre archivos `.md` y por qué medios se le puede
pedir que abra uno.

### RF-20.1 — Ancho del texto al redimensionar la ventana

Refina: RF-20
El texto del documento debe reajustarse al ancho de la ventana cuando esta cambia
de tamaño, hasta un ancho máximo de lectura, a partir del cual la columna de
texto deja de crecer y se mantiene centrada. El texto no queda cortado ni obliga
a desplazarse horizontalmente para leer un párrafo.

## No funcionales

### RNF-01.1 — Tiempo hasta el documento visible, desde la línea de comandos

Refina: RNF-01
Desde que se invoca la aplicación con la ruta de un archivo hasta que su
documento está visible deben transcurrir **menos de 1 segundo**, medido en
arranque en frío y con un archivo de aproximadamente 50 KB.

El requisito padre mide desde el doble clic en el Explorador. Aquí se mide desde
la línea de comandos, que es el único camino que existe en esta feature y que
recorre menos trabajo. **Cumplir esto no demuestra que se cumpla RNF-01**: el
escenario del doble clic se verifica por separado en la feature
`integracion-con-windows`, y un resultado favorable aquí no vale como evidencia
de aquel.

### RNF-03.1 — Plataforma de ejecución

Refina: RNF-03
El sistema debe ejecutarse en Windows 11 de 64 bits.

## Requisitos del sistema que esta feature no cubre

Se listan para que quede claro que la ausencia es deliberada y para que nadie los
dé por hechos al cerrar la feature: RF-02, RF-03, RF-04, RF-05, RF-06, RF-07,
RF-11, RF-12, RF-13, RF-14, RF-15 y RNF-02. Su reparto entre las otras tres
features está en `plan.md`.

Los requisitos RF-19 y RF-20 nacieron en Fase 3, después de escribirse las
historias, y sí pertenecen a esta feature: están refinados arriba.
