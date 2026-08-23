# Contexto

> Estado: borrador
> Última actualización: 2026-08-20
> Modo: new-project

## Problema

Para leer un archivo Markdown en el escritorio, la herramienta que hay a mano es
un editor de código o un IDE. Abrirlo cuesta varios segundos, carga extensiones,
indexa la carpeta que lo contiene y reserva cientos de megabytes de memoria, todo
para mostrar un documento de texto que el usuario solo quiere leer. El coste de
la herramienta es desproporcionado respecto a la tarea. Existe además un segundo
problema: cuando hay que consultar dos o tres documentos a la vez —un README, una
guía de contribución, unas notas—, no hay forma ligera de tenerlos todos a mano
sin acumular ventanas o sin cargar el proyecto entero en el editor.

## Usuarios

Hay un único tipo de usuario. No existen roles con permisos distintos ni
objetivos distintos.

- **Lector de documentación técnica** (desarrollador, redactor técnico, o
  cualquiera que se encuentre un `.md` en el disco): necesita ver el documento
  formateado de forma legible en el momento en que hace doble clic sobre él, y
  poder tener varios abiertos a la vez sin que eso implique abrir un proyecto.

El desarrollador del proyecto es también su primer usuario.

## Restricciones

Restricciones afirmadas por el usuario:

- El lenguaje es **Rust** y la interfaz gráfica se construye con **GPUI**. Ambas
  cosas están decididas de antemano y no se reabren.
- El objetivo declarado del proyecto es ser **multiplataforma**, pero la primera
  versión se limita a **Windows**.
- No hay fecha de entrega.
- No hay cliente, supervisor ni evaluador externo que espere entregables o
  ceremonias concretas.
- No se ha declarado ninguna regulación, norma de privacidad ni requisito de
  residencia de datos aplicable.

Consecuencias técnicas verificadas de la restricción de usar GPUI (comprobadas
consultando crates.io y el `README` de `crates/gpui` del repositorio
`zed-industries/zed` el 2026-08-20, no supuestas):

- `gpui` está publicado en crates.io; la última versión encontrada fue la
  **0.2.2**, publicada el 2025-10-22, con licencia Apache-2.0.
- GPUI es **pre-1.0 y está en desarrollo activo**. Su API puede romperse entre
  versiones menores. El proyecto asume ese riesgo por decisión del usuario.
- En **Windows**, `gpui_platform` no requiere activar ninguna *feature*: usa
  Win32 y DirectWrite de forma nativa. En Linux exige `wayland`, `x11` o ambas;
  en macOS exige Xcode instalado y la *feature* `font-kit`.
- La consecuencia práctica para el alcance multiplataforma es que **macOS no se
  puede compilar ni verificar sin una máquina Apple**. Cualquier criterio de
  aceptación relativo a macOS quedaría registrado como `bloqueado`, no como
  `pasa`, mientras no exista ese equipo.

## Equipo

Un solo desarrollador. No hay reparto de trabajo ni paso de revisión por otra
persona.

## Metodología

**Elegida:** Kanban, con límite de trabajo en curso de 1 historia.

**Razón:** el equipo es de una sola persona, así que las ceremonias de Scrum no
tendrían a nadie a quien sincronizar; celebrar planificación, revisión y
retrospectiva con uno mismo es un ritual, no gestión de riesgo. No hay fecha de
entrega ni cliente externo que espere un compromiso por iteración, de modo que
comprometerse a un lote de trabajo no aporta nada. Los requisitos son acotados y
razonablemente estables —es un visor, no un producto en descubrimiento—, con lo
que tampoco hace falta el mecanismo de replanificación periódica que justifica a
Scrum. El límite de 1 responde a que un desarrollador único que abre tres frentes
a la vez no termina ninguno.

**Consecuencias:** el progreso se mide por historias cerradas y verificadas, no
por velocidad ni por puntos; no hay compromiso de iteración que romper. Cada
historia debe ser lo bastante pequeña para terminarse y verificarse de una
sentada. Ninguna historia nueva se empieza mientras haya otra abierta, lo que
incluye no empezar la siguiente mientras la anterior esté pendiente de
verificación.

## Glosario

Estos términos significan lo mismo en todos los demás artefactos del proyecto.

| Término | Significado acordado |
| --- | --- |
| **MDView** | Nombre del proyecto y de la aplicación. |
| **Documento** | El contenido de un archivo `.md` una vez leído del disco. |
| **Renderizar** | Convertir el texto Markdown en su presentación visual con formato. No implica generar HTML. |
| **Pestaña** | Elemento de la interfaz que representa un documento abierto. Una pestaña, un documento. |
| **Pestaña activa** | La única pestaña cuyo documento se está mostrando en ese instante. |
| **Instancia única** | Comportamiento por el cual solo existe un proceso de MDView en ejecución: si se pide abrir otro archivo, ese proceso lo recibe en lugar de arrancar uno nuevo. |
| **Arranque en frío** | Arranque del proceso sin que la aplicación se haya ejecutado antes desde el último inicio del sistema, de modo que las bibliotecas del sistema aún no están en caché. Es la medida que cuenta para el objetivo de rendimiento. |
| **Documento visible** | Instante en que el usuario ve el documento renderizado en pantalla, no el instante en que el proceso arranca ni en que aparece la ventana vacía. |
| **CommonMark** | Especificación base de Markdown. |
| **GFM** | *GitHub Flavored Markdown*: CommonMark más tablas, listas de tareas y texto tachado. Es el nivel de soporte de la primera versión. |
| **GPUI** | Framework de interfaz gráfica acelerada por GPU, escrito en Rust, procedente del editor Zed. |
| **`gpui-component`** | Biblioteca de terceros (longbridge) con componentes construidos sobre GPUI. Candidata, no decidida. |
| **Front matter** | Bloque de metadatos delimitado por líneas `---` al principio de un archivo Markdown, escrito en YAML. |
| **Aviso temporal** | Mensaje breve que aparece sobre la interfaz, informa de algo y desaparece por sí solo, sin exigir que el usuario lo cierre. |
| **Subconjunto de HTML** | Conjunto cerrado de etiquetas HTML que MDView interpreta traduciéndolas a los elementos que ya dibuja para el Markdown. No implica motor de HTML ni soporte de CSS. |
