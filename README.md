# Trigonometric Parser
Este proyecto implementa un analizador sintáctico (parser) en Rust para evaluar expresiones matemáticas que incluyen funciones trigonométricas. Utiliza una gramática LL(1) para procesar las expresiones y generar un Árbol de Sintaxis Abstracta (AST) que representa su estructura.

## Descripción General
El parser es capaz de manejar expresiones matemáticas con operadores aritméticos básicos, funciones trigonométricas, y potencias. La salida del parser es un árbol de sintaxis abstracta (AST) que se puede usar para evaluar la expresión, generar código o realizar transformaciones adicionales.

## Gramática
La gramática de este parser está documentada en detalle en el archivo [gramatica.md](https://github.com/ginozza/trigonometric-parser/blob/main/gramatica.md).

## Colaboradores
Este proyecto es desarrollado y mantenido por:

[<img src="https://github.com/ginozza.png" width="60px;" /><br /><sub><a href="https://github.com/ginozza">ginozza</a></sub>](https://github.com/ginozza)

[<img src="https://github.com/sebastiann1212.png" width="60px;" /><br /><sub><a href="https://github.com/sebastiann1212">sebastiann1212</a></sub>](https://github.com/sebastiann1212)

## Uso
1. **Clonar el repositorio:**
```bash
git clone https://github.com/ginozza/trigonometric_functions_grammar.git
cd trigonometric_functions_grammar
```

2. **Compilar el proyecto:**
```bash
cargo build
```

3. **Ejecutar el parser y proporcionar la expresión matemática:**
```bash
cargo run
```

4. **Al ejecutar el comando anterior, el programa te pedirá que ingreses una expresión matemática para analizar. Escribe la expresión y presiona Enter. Por ejemplo:**
```scss
sin(45)^2 + cos(xy)
```

El parser generará un Árbol de Sintaxis Abstracta (AST) representando la estructura de la expresión matemática.

## Contribuciones
Si deseas mejorar, corregir o añadir nuevas características, sigue estos pasos:

1. **Haz un fork del repositorio**  

   Dirígete al repositorio y haz un fork para tener tu propia copia del proyecto.

2. **Crea una nueva rama**  

   Trabaja en una nueva rama para tus cambios, evitando realizar modificaciones directamente en la rama principal (main o master).

```bash
git checkout -b feature-branch
```

3. **Realiza tus cambios y haz un commit**

    Haz tus modificaciones y asegúrate de que todo funcione correctamente. Los mensajes de commit deben seguir el enfoque de Conventional Commits para mantener consistencia y claridad en el historial de cambios.

El formato de los mensajes de commit es el siguiente:

- `tipo: mensaje`
    - Ejemplo de tipos:
        - `feat`: para nuevas características.
        - `fix`: para correcciones de errores.
        - `docs`: para actualizaciones de la documentación.
        - `style`: para cambios de estilo (espaciado, formato, etc.).
        - `refactor`: para cambios de código que no afectan la funcionalidad.
        - `test`: para agregar o modificar pruebas.
        - `chore`: para tareas de mantenimiento, como actualizaciones de dependencias.

Ejemplo de un mensaje de commit:

```bash
git commit -am 'feat: add trigonometric function parser'
```

4. **Realiza un push a tu rama** 

    Una vez que hayas hecho los cambios y commits necesarios, realiza un push a tu rama:

```bash
git push origin feature-branch
```

5. **Abre un pull request** 

    Ve al repositorio original y abre un pull request explicando brevemente qué cambios realizaste y por qué. Incluye detalles sobre la funcionalidad nueva o el error que has corregido.

## Licencia

Este proyecto está licenciado bajo la **Licencia MIT**. Consulta el archivo [LICENSE](https://github.com/ginozza/trigonometric-parser/blob/main/LICENSE) para más detalles. Fue desarrollado como parte de un **proyecto de Diseño de Compiladores** en la **Universidad del Magdalena**.
