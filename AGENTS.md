# MDView

Visor de archivos Markdown para escritorio, escrito en Rust con GPUI. Abre
documentos `.md` renderizados en pestañas, con el objetivo de sustituir la
costumbre de abrir un IDE solo para leer un archivo de texto.

Estado: siete features cerradas (veintiocho historias verificadas), una
octava en curso. Ver la tabla de abajo.

## Documentación

Las decisiones que gobiernan este repositorio están en `docs/`. Léelas antes de
tocar código.

| Archivo | Qué contiene |
| --- | --- |
| `docs/00-contexto.md` | Problema, usuarios, restricciones, metodología y glosario. |
| `docs/01-alcance.md` | Dentro y fuera del alcance, supuestos y preguntas abiertas. |
| `docs/02-requisitos.md` | Requisitos funcionales y no funcionales de todo el sistema. |
| `docs/03-arquitectura.md` | Fronteras entre módulos, convenciones y el índice de todas las decisiones. |
| `docs/04-calidad.md` | Resultado observado de cada criterio de aceptación, historia por historia. |
| `docs/features/ver-un-documento/` | Requisitos, historias, plan y decisiones de la primera feature. |
| `docs/features/enlaces-e-imagenes/` | Ídem de la segunda: enlaces externos e imágenes locales. |
| `docs/features/front-matter-y-html/` | Ídem de la tercera: front matter como tabla de propiedades y un subconjunto de HTML incrustado. |
| `docs/features/varios-documentos-en-pestanas/` | Ídem de la cuarta: varios documentos abiertos a la vez, en pestañas, y navegación entre ellos (RF-14). |
| `docs/features/integracion-con-windows/` | Ídem de la quinta: asociación de `.md`, doble clic e instancia única (solo RF-03.1; RF-02 se aplazó a la sexta). |
| `docs/features/distribucion/` | Ídem de la sexta: instalador, integración en el PATH y versión portable (RF-02.1, RF-21.1, RF-22.1). |
| `docs/features/ver-markdown-en-crudo/` | Ídem de la séptima: alternar entre documento renderizado y texto en crudo (RF-23). |
| `docs/features/edicion-con-confirmacion/` | Ídem de la octava, en curso: edición con confirmación explícita (RF-24), acotada a editar el crudo con `gpui_component::input`. |

Estado: siete features cerradas — `ver-un-documento`, `enlaces-e-imagenes`,
`front-matter-y-html`, `varios-documentos-en-pestanas`,
`integracion-con-windows`, `distribucion` (cerró RF-02, pendiente desde
`varios-documentos-en-pestanas`) y `ver-markdown-en-crudo` (cerró RF-23,
aplazado desde el 2026-08-20). `edicion-con-confirmacion` abierta el
2026-09-18. Una más planeada después: `multiplataforma`, bloqueada en su
verificación hasta que exista una máquina Linux o Mac.

El trabajo de las primeras seis features estaba dividido según
`docs/features/enlaces-e-imagenes/plan.md` (las cinco primeras) y
`docs/features/distribucion/requisitos.md` (la sexta). Las features
siguientes no comparten un plan: cada una es su propia unidad de trabajo.

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
