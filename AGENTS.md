# MDView

Visor de archivos Markdown para escritorio, escrito en Rust con GPUI. Abre
documentos `.md` renderizados en pestañas, con el objetivo de sustituir la
costumbre de abrir un IDE solo para leer un archivo de texto.

Estado: en documentación. Todavía no hay código.

## Documentación

Las decisiones que gobiernan este repositorio están en `docs/`. Léelas antes de
tocar código.

| Archivo | Qué contiene |
| --- | --- |
| `docs/00-contexto.md` | Problema, usuarios, restricciones, metodología y glosario. |
| `docs/01-alcance.md` | Dentro y fuera del alcance, supuestos y preguntas abiertas. |
| `docs/02-requisitos.md` | Requisitos funcionales y no funcionales de todo el sistema. |
| `docs/03-arquitectura.md` | Fronteras entre módulos, convenciones y el índice de todas las decisiones. |
| `docs/features/ver-un-documento/` | Requisitos, historias, plan y decisiones de la primera feature. |

Falta por escribir `docs/04-calidad.md`, que se abre al verificar la primera
historia y recoge el resultado observado de cada criterio. Esta tabla se
amplía en la misma fase en que cada archivo aparece.

El trabajo está dividido en cuatro features. Solo la primera tiene artefactos; el
orden de las cuatro y el reparto de requisitos entre ellas está en
`docs/features/ver-un-documento/plan.md`.

## Reglas

- No se escribe código sin una historia que lo cubra en
  `docs/features/<slug>/historias.md`.
- Toda decisión que alguien pueda querer revertir más adelante se escribe en el
  `decisiones.md` de la feature correspondiente, y se indexa en
  `docs/03-arquitectura.md` en el mismo momento.
- Un criterio de aceptación se verifica **por observación**. Lo que no se pudo
  observar se registra como `bloqueado`, nunca como `pasa`. Que el diseño lo
  garantice o que una biblioteca lo prometa no es evidencia.
- El idioma de trabajo de este proyecto es el español. Los nombres de archivo y
  de directorio no se traducen.

## Método de trabajo

Kanban con límite de trabajo en curso de 1 historia: no se empieza una historia
nueva mientras haya otra abierta, y una historia sigue abierta hasta que sus
criterios están verificados. La razón de esta elección está en la sección
«Metodología» de `docs/00-contexto.md`.
