## Init rust project 

1. cargo init - init a end user application project , which contains binary representation of a file

2. cargo init --lib -  this create a library based project , this cannot get executed directly , we need to create test case to execute the code ,since the code written are reusable code 

## variables , data types

variables are not mutable , that is the values are not changeable
   ### the error : cannot mutate immutable variable `num`

while declaring variables , put _ if you want space 
### the error : Variable `myString` should have snake_case name, e.g. `my_string` 
so we "mut" when variable is declared
   
struct lets you create a list of variable  with different data type in a single form