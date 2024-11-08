# Gramática de expresiones trigonométricas

## Producciones
1. `<Expresión> -> <Término> <Expresión'>`
2. `<Expresión'> -> + <Término> <Expresión'>`
3. `<Expresión'> -> - <Término> <Expresión'>`
4. `<Expresión'> -> ε`
5. `<Término> -> <FactorConSigno> <Término'>`
6. `<Término'> -> * <FactorConSigno> <Término'>`
7. `<Término'> -> / <FactorConSigno> <Término'>`
8. `<Término'> -> ε`
9. `<FactorConSigno> -> - <Factor>`
10. `<FactorConSigno> -> <Factor>`
11. `<Factor> -> <FuncionTrig> <Potencia>`
12. `<Factor> -> ( <Expresión> ) <Potencia>`
13. `<Factor> -> <VAR>`
14. `<Factor> -> <NUM> <Potencia>`
15. `<Potencia> -> ^ <NUM>`
16. `<Potencia> -> ε`
17. `<FuncionTrig> -> <FUNTRI> ( <Argumento> )`
18. `<FUNTRI> -> SEN | COS | TAN | SEC | ... | ARCCSCH`
19. `<Argumento> -> <FuncionTrig>`
20. `<Argumento> -> <VAR>`
21. `<Argumento> -> <NUM>`
22. `<Argumento> -> <Potencia>`

### Nota
- `<VAR>` representa cadenas formadas por letras (`a-z`, `A-Z`) con cerradura de Kleene (`+`).
- `<NUM>` representa cadenas de dígitos (`0-9`) con cerradura de Kleene (`+`).

---

## Primeros
- `Pri(<Expresión>) = {-, SEN, COS, TAN, SEC, ..., ARCCSCH, (, a, ..., Z+, 0, ..., 9+}`
- `Pri(<Expresión'>) = {+, -}`
- `Pri(<Término>) = {-, SEN, COS, TAN, SEC, ..., ARCCSCH, (, a, ..., Z+, 0, ..., 9+}`
- `Pri(<Término'>) = {*, /}`
- `Pri(<FactorConSigno>) = {-, SEN, COS, TAN, SEC, ..., ARCCSCH, (, a, ..., Z+, 0, ..., 9+}`
- `Pri(<Factor>) = {SEN, COS, TAN, SEC, ..., ARCCSCH, (, a, ..., Z+, 0, ..., 9+}`
- `Pri(<Potencia>) = {^}`
- `Pri(<FuncionTrig>) = {SEN, COS, TAN, SEC, ..., ARCCSCH}`
- `Pri(<FUNTRI>) = {SEN, COS, TAN, SEC, ..., ARCCSCH}`
- `Pri(<Argumento>) = {SEN, COS, TAN, SEC, ..., ARCCSCH, (, a, ..., Z+, 0, ..., 9+, ^}`
- `Pri(<VAR>) = {a, ..., Z+}`
- `Pri(<NUM>) = {0, ..., 9+}`

---

## Siguientes
- `Sig(<Expresión>) = {-, ), -|}`
- `Sig(<Expresión'>) = {-, ), -|}`
- `Sig(<Término>) = {-, ), -|}`
- `Sig(<Término'>) = {-, ), -|}`
- `Sig(<FactorConSigno>) = {-, ), -|}`
- `Sig(<Factor>) = {-, ), -|}`
- `Sig(<Potencia>) = {-, ), -|}`
- `Sig(<FuncionTrig>) = {-, ), -|}`
- `Sig(<FUNTRI>) = {(}`
- `Sig(<Argumento>) = {)}`
- `Sig(<VAR>) = {-, ), -|}`
- `Sig(<NUM>) = {-, ), -|}`

---

## Selección
- `Sel(1) = {SEN, COS, TAN, SEC, ARCCSCH, (, a, ..., Z+, 0, ..., 9+, -}`
- `Sel(2) = {+}`
- `Sel(3) = {-}`
- `Sel(4) = {), -|}`
- `Sel(5) = {SEN, COS, TAN, SEC, ARCCSCH, (, a, ..., Z+, 0, ..., 9+, -}`
- `Sel(6) = {*}`
- `Sel(7) = {/}`
- `Sel(8) = {), -|}`
- `Sel(9) = {-}`
- `Sel(10) = {SEN, COS, TAN, SEC, ARCCSCH, (, a, ..., Z+, 0, ..., 9+}`
- `Sel(11) = {SEN, COS, TAN, SEC, ARCCSCH}`
- `Sel(12) = {(}`
- `Sel(13) = {a, ..., Z+}`
- `Sel(14) = {0, ..., 9+}`
- `Sel(15) = {^}`
- `Sel(16) = {), -|}`
- `Sel(17) = {SEN, COS, TAN, SEC, ARCCSCH}`
- `Sel(18) = {SEN, COS, TAN, SEC, ARCCSCH}`
- `Sel(19) = {SEN, COS, TAN, SEC, ARCCSCH}`
- `Sel(20) = {a, ..., Z+}`
- `Sel(21) = {0, ..., 9+}`
- `Sel(22) = {^, )}`
- `Sel(23) = {a, ..., Z+}`
- `Sel(24) = {0, ..., 9+}`

---

## Conjuntos
- **S.E** ={+,-,*,/,(,),^,FUNTRI,VAR,NUM,-|}
- **S.P** ={<Expresión>,<Expresión’> ,<Término>,<Término’>,<FactorConSigno>, <Factor>, <Potencia>, <FuncionTrig>, <FUNTRI>, <Argumento>, <VAR>, <NUM>,(,),}
- **C.I.P** ={ <Expresión> }

---

## Transiciones
- `#0: Desapile, avance`
- `#1: Reemplace (<Expresión'> <Término>), retenga`
- `#2: Reemplace (<Expresión'> <Término>), avance`
- `#3: Reemplace (<Expresión'> <Término>), avance`
- `#4: Desapile, retenga`
- `#5: Reemplace (<Término'> <FactorConSigno>), retenga`
- `#6: Reemplace (<Término'> <FactorConSigno>), avance`
- `#7: Reemplace (<Término'> <FactorConSigno>), avance`
- `#8: Desapile, retenga`
- `#9: Reemplace (<Factor>), avance`
- `#10: Reemplace (<Factor>), retenga`
- `#11: Reemplace (<Potencia> <FuncionTrig>), retenga`
- `#12: Reemplace (<Potencia> <Expresión>), avance`
- `#13: Reemplace (<VAR>), retenga`
- `#14: Reemplace (<Potencia> <NUM>), retenga`
- `#15: Reemplace (<NUM>), avance`
- `#16: Desapile, retenga`
- `#17: Reemplace (<FuncionTrig>), retenga`
- `#18: Desapile, avance`
- `#19: Reemplace (<FuncionTrig>), retenga`
- `#20: Reemplace (<VAR>), retenga`
- `#21: Reemplace (<NUM>), retenga`
- `#22: Reemplace (<Potencia>), retenga`
- `#23: Desapile, avance`
- `#24: Desapile, avance`
