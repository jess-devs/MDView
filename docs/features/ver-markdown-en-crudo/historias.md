# Historias — ver-markdown-en-crudo

> Estado: borrador
> Última actualización: 2026-09-18
> Modo: new-feature

Una historia, un único tipo de usuario: el lector de documentación técnica
de `00-contexto.md`.

---

## HU-01 — Alternar entre el documento renderizado y su texto en crudo

Cubre: RF-23.1, RF-23.2

**Historia.** Como lector de documentación técnica, quiero poder ver el
texto exacto de un `.md` —por ejemplo, para copiar un fragmento de código
que el renderizado ya formateó, o para entender por qué algo no se ve
como esperaba— sin tener que abrir otro programa.

**Criterios de aceptación.** Se comprueban con un documento que tenga
front matter, HTML incrustado, y al menos una línea más ancha que la
ventana.

1. CA-01.1 — Un documento se abre renderizado por defecto, con el
   control de alternar visible.
2. CA-01.2 — Pulsar el control muestra el texto en crudo: idéntico,
   carácter por carácter, al contenido del archivo en disco —incluidos
   los delimitadores `---` del front matter y las etiquetas HTML, sin
   interpretar ninguno.
3. CA-01.3 — El crudo se muestra en tipografía monoespaciada.
4. CA-01.4 — La línea más ancha que la ventana se conserva entera y se
   desplaza horizontalmente por sí sola, sin arrastrar el resto del
   documento ni partir la línea.
5. CA-01.5 — Pulsar el control de nuevo vuelve al documento renderizado.
6. CA-01.6 — Con dos pestañas abiertas, poner una en modo crudo y
   cambiar a la otra la muestra renderizada: el modo es de cada pestaña,
   no global.

**Estado.** pendiente.

---

## Cobertura

| Requisito | Historia que lo cubre |
| --- | --- |
| RF-23.1 | HU-01 |
| RF-23.2 | HU-01 |

El único requisito de esta feature queda cubierto por su única historia.
