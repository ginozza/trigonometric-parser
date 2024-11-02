# Trigonometric Parser
Este proyecto implementa un parser en Rust que analiza y evalúa expresiones matemáticas con funciones trigonométricas, utilizando una gramática LL(1). El parser genera un árbol de sintaxis abstracta (AST) para representar la estructura de las expresiones evaluadas

## Gramática
Se define como la siguiente:

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

## Colaboradores
Este proyecto cuenta con la participacion de:

[<img src="https://github.com/ginozza.png" width="60px;" /><br /><sub><a href="https://github.com/ginozza">ginozza</a></sub>](https://github.com/ginozza)

[<img src="https://github.com/sebastiann1212.png" width="60px;" /><br /><sub><a href="https://github.com/sebastiann1212">sebastiann1212</a></sub>](https://github.com/sebastiann1212)