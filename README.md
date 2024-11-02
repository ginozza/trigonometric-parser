# Trigonometric Parser
Este proyecto implementa un parser en Rust que analiza y evalúa expresiones matemáticas con funciones trigonométricas, utilizando una gramática LL(1). El parser genera un árbol de sintaxis abstracta (AST) para representar la estructura de las expresiones evaluadas

## Gramática
Se define como la siguiente:

```Expresion -> Termino Expresion'
Expresion' -> + Termino Expresion' | - Termino Expresion' | ε
Termino -> FactorConSigno Termino'
Termino' -> * FactorConSigno Termino' | / FactorConSigno Termino' | ε
FactorConSigno -> - Factor | Factor
Factor -> FuncionTrig Potencia | ( Expresion ) Potencia | VAR | NUM Potencia
Potencia -> ^ NUM | ε
FuncionTrig -> SEN ( Argumento ) | COS ( Argumento ) | TAN ( Argumento ) | COT ( Argumento ) |
               SEC ( Argumento ) | CSC ( Argumento ) |
               ARCSEN ( Argumento ) | ARCCOS ( Argumento ) | ARCTAN ( Argumento ) | ARCCOT ( Argumento ) |
               ARCSEC ( Argumento ) | ARCCSC ( Argumento ) |
               SINH ( Argumento ) | COSH ( Argumento ) | TANH ( Argumento ) | COTH ( Argumento ) |
               SECH ( Argumento ) | CSCH ( Argumento ) |
               ARSINH ( Argumento ) | ARCOSH ( Argumento ) | ARTANH ( Argumento ) | ARCOTH ( Argumento ) |
               ARSECH ( Argumento ) | ARCCSCH ( Argumento )
Argumento -> Expresion | VAR | NUM Potencia
VAR -> a | ... | Z+
NUM -> 0 | ... | 9+
```

## Colaboradores
Este proyecto cuenta con la participación de:
<div style="display: flex; justify-content: center; align-items: center;">
  <a href="https://github.com/ginozza">
    <img src="https://github.com/ginozza.png" alt="Juan Simancas" style="border-radius: 20%; width: 50px; height: 50px; margin-right: 10px;" />
  </a>
  <a href="https://github.com/sebastiann1212">
    <img src="https://github.com/sebastiann1212.png" alt="Juan Sebastian Martinez" style="border-radius: 20%; width: 50px; height: 50px;" />
  </a>
</div>

