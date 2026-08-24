# Encabezado de nivel 1

## Encabezado de nivel 2

### Encabezado de nivel 3

#### Encabezado de nivel 4

##### Encabezado de nivel 5

###### Encabezado de nivel 6

Este párrafo tiene texto normal, *texto en cursiva*, **texto en negrita** y
también ***negrita y cursiva a la vez***. Después de esto sigue texto normal
otra vez, para comprobar que se distingue de lo anterior.

## Listas

Lista no ordenada de dos niveles:

- Elemento uno del primer nivel
- Elemento dos del primer nivel
  - Elemento anidado A
  - Elemento anidado B
- Elemento tres del primer nivel

Lista ordenada:

1. Primer paso
2. Segundo paso
3. Tercer paso

Lista de tareas:

- [ ] Tarea sin marcar
- [x] Tarea marcada
- [ ] Otra tarea sin marcar

## Cita

> Esta es una cita. Debe verse con un margen izquierdo mayor que el del texto
> normal y con una marca vertical a su izquierda.

## Regla horizontal

Antes de la regla.

---

Después de la regla.

## Código

Código en línea: usa `pulldown_cmark::Parser::new_ext` para analizar el texto.

Bloque de código, con una línea deliberadamente más ancha que cualquier ventana
razonable, para HU-03:

```rust
fn linea_muy_larga_para_probar_el_desplazamiento_horizontal(parametro_uno: i32, parametro_dos: i32, parametro_tres: i32, parametro_cuatro: i32) -> i32 {
    parametro_uno + parametro_dos + parametro_tres + parametro_cuatro
}
```

## Tabla

| Columna A | Columna B | Columna C |
| --- | --- | --- |
| uno | dos | tres |
| cuatro | cinco | seis |

## Tachado

Este texto tiene ~~una parte tachada~~ en medio de la frase.

## Cierre

Fin del documento de prueba, usado por HU-02 y por HU-03.
