# Trigonometric Parser
Este proyecto implementa un analizador sintáctico (parser) en Rust para evaluar expresiones matemáticas que incluyen funciones trigonométricas. Utiliza una gramática LL(1) para procesar las expresiones y generar un Árbol de Sintaxis Abstracta (AST) que representa su estructura.

## Descripción General
El parser es capaz de manejar expresiones matemáticas con operadores aritméticos básicos, funciones trigonométricas, y potencias. La salida del parser es un árbol de sintaxis abstracta (AST) que se puede usar para evaluar la expresión, generar código o realizar transformaciones adicionales.

## Gramática
La gramática de este parser está definida en las siguientes reglas, que cubren las operaciones matemáticas estándar, funciones trigonométricas, y potencias:

```
1.	<Expresion> -> <Termino> <Expresion>
2.	<Expresion> -> +<Termino> <Expresion>  
3.	<Expresion> ->  - <Termino>  <Expresion> 
4.	<Expresion> ->   ε
5.	<Termino> -> <FactorConSigno>  <Termino>  
6.	<Termino> -> * <FactorConSigno>  <Termino>  
7.	<Termino> -> / <FactorConSigno>  <Termino>  
8.	<Termino> ->  ε
9.	<FactorConSigno> -> - <Factor>
10.	<FactorConSigno> ->  <Factor>
11.	<Factor> -> <FuncionTrig>  <Potencia>  
12.	<Factor> -> ( <Expresion>  ) <Potencia>  
13.	<Factor> ->  <VAR> 
14.	<Factor> ->  <NUM> <Potencia>
15.	<Potencia> -> ^ <NUM> 
16.	<Potencia> -> ε
17.	<FuncionTrig> -> SEN ( <Argumento>) 
18.	<FuncionTrig> -> COS ( <Argumento>)
19.	<FuncionTrig> -> TAN ( <Argumento>) 
20.	<FuncionTrig> -> COT ( <Argumento>) 
21.	<FuncionTrig> ->SEC ( <Argumento>) 
22.	<FuncionTrig> ->CSC ( <Argumento>) 
23.	<FuncionTrig> -> ARCSEN ( <Argumento>) 
24.	<FuncionTrig> -> ARCCOS ( <Argumento>) 
25.	<FuncionTrig> -> ARCTAN ( <Argumento>) 
26.	<FuncionTrig> -> ARCCOT ( <Argumento>) 
27.	<FuncionTrig> ->ARCSEC ( Argumento ) 
28.	<FuncionTrig> -> ARCCSC ( <Argumento>) 
29.	<FuncionTrig> ->SINH ( <Argumento>) 
30.	<FuncionTrig> ->COSH ( <Argumento>) 
31.	<FuncionTrig> ->TANH ( <Argumento>) 
32.	<FuncionTrig> -> COTH ( Argumento ) 
33.	<FuncionTrig> ->SECH ( <Argumento>) 
34.	<FuncionTrig> -> CSCH ( <Argumento>) 
35.	<FuncionTrig> ->ARSINH ( <Argumento>)
36.	<FuncionTrig> ->ARCOSH ( <Argumento>)
37.	<FuncionTrig> ->ARTANH ( <Argumento>)
38.	<FuncionTrig> ->ARCOTH ( Argumento )
39.	<FuncionTrig> ->ARSECH ( <Argumento>) 
40.	<FuncionTrig> ->ARCCSCH ( <Argumento>)
41.	<Argumento> -> Expresion 
42.	<Argumento> -> <VAR> 
43.	<Argumento> -> <NUM> 
44.	<Argumento> -> <Potencia>
45.	<VAR> -> a-Z+
46.	<NUM> -> 0-9+
```

### Explicación de las reglas
- Expresiones (Expresion): Estas reglas definen las operaciones de suma y resta que se pueden realizar sobre términos, y cómo los términos se descomponen en factores.

- Operadores (Termino): Las operaciones de multiplicación y división se manejan en esta sección, y también se permite la recursión.

- Funciones trigonométricas (FuncionTrig): La gramática soporta funciones trigonométricas estándar (como seno, coseno, tangente) y sus inversas (como arcseno, arcocoseno), incluyendo funciones hiperbólicas.

- Argumentos: Los argumentos de las funciones trigonométricas pueden ser expresiones completas, variables o números.

## Colaboradores
Este proyecto es desarrollado y mantenido por:

[<img src="https://github.com/ginozza.png" width="60px;" /><br /><sub><a href="https://github.com/ginozza">ginozza</a></sub>](https://github.com/ginozza)

[<img src="https://github.com/sebastiann1212.png" width="60px;" /><br /><sub><a href="https://github.com/sebastiann1212">sebastiann1212</a></sub>](https://github.com/sebastiann1212)

## Uso
1. Clonar el repositorio:
```bash
git clone https://github.com/ginozza/trigonometric_functions_grammar.git
cd trigonometric_functions_grammar
```

2. Compilar el proyecto:
```bash
cargo build
```

3. Ejecutar el parser y proporcionar la expresión matemática:
```bash
cargo run
```

4. Al ejecutar el comando anterior, el programa te pedirá que ingreses una expresión matemática para analizar. Escribe la expresión y presiona Enter. Por ejemplo:
```scss
sin(45)^2 + cos(xy)
```

El parser generará un Árbol de Sintaxis Abstracta (AST) representando la estructura de la expresión matemática.

## Contribuciones
Si deseas mejorar, corregir o añadir nuevas características, sigue estos pasos:

1. Haz un fork del repositorio
Dirígete al repositorio y haz un fork para tener tu propia copia del proyecto.

2. Crea una nueva rama
Trabaja en una nueva rama para tus cambios, evitando realizar modificaciones directamente en la rama principal (main o master):

```bash
git checkout -b feature-branch
```

3. Realiza tus cambios y haz un commit
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

4. Realiza un push a tu rama
Una vez que hayas hecho los cambios y commits necesarios, realiza un push a tu rama:

```bash
git push origin feature-branch
```

5. Abre un pull request
Ve al repositorio original y abre un pull request explicando brevemente qué cambios realizaste y por qué. Incluye detalles sobre la funcionalidad nueva o el error que has corregido.

## Licencia

Este proyecto está licenciado bajo la **Licencia MIT**. Consulta el archivo [LICENSE](https://github.com/ginozza/trigonometric-parser/blob/main/LICENSE) para más detalles. Fue desarrollado como parte de un **proyecto de Diseño de Compiladores** en la **Universidad del Magdalena**.
