# c-expression-validator

This project objective is to be able to validate C program expressions using a pushdown automaton in order to explore the lexical analyzer component of a compiler. 

The formal definitions of the expressions considered are:
- \<assignation\> := \<variable\>=\<expression\>
- \<variable\> := \<letter\>\<char\>
- \<letter\> := [A-Z]|[a-z]
- \<expression\> := \<apperture\>\<letter\>\<number\>\operand\>\<closer\>\<final\>
- \<apperture\> := (|lambda
- \<number\> := [0-9]
- \<closer\> := )|lambda
- \<final\> := ;

## Grammar definition
Lang={C expressions}
AP = {{q0,q1,q2,q3,q4,q5,q6},{A-Z,a- z,0-9,+,-,*,/,%,_.}.{Z,0,8,q0,Z,96}
<img width="634" alt="Screenshot 2023-03-06 at 12 42 58" src="https://user-images.githubusercontent.com/55115748/223201766-86fbb464-a77e-4fd2-8d81-9fb4458c3ab5.png">

## Execution example
<img width="577" alt="image" src="https://user-images.githubusercontent.com/55115748/223202477-2d48072d-5a00-43b6-928a-fa96769d754a.png">

