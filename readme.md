## Init rust project 

1. cargo init - init a end user application project , which contains binary representation of a file

2. cargo init --lib -  this create a library based project , this cannot get executed directly , we need to create test case to execute the code ,since the code written are reusable code 

## variables , data types

string inside '' is a char , limited size
string insede "" is a borrowed string , unlimited string

variables are not mutable , that is the values are not changeable
   ### the error : cannot mutate immutable variable `num`

while declaring variables , put _ if you want space 
### the error : Variable `myString` should have snake_case name, e.g. `my_string` 
so we "mut" when variable is declared

### struct

struct lets you create a list of variable  with different data type in a single form

### enum

enum lets you create enumerate variables for limited and specific usage

  built-in enumns : 
   ### 1. options - lets you return null , None , nil type for a function or any other reference types or some type of values 
   ### 2.result - lets you return OK and Err value , used for error handling in rust

pattern matching is similar to switch-case , but here the statement provided must align with any one of the statement and the return type of all the statement must be same

### Package Management 
  we can add external libraries or in rust called as crates by command :
    cargo add crate_name

# Part -2 
## Collections 
    -> data structures that are used for storing multiple values , and these data collections stored in the heap , since these contains large space so we store more values , push and delete , that is dynamically allocate those memory
  ## Vectors
  ## Hashmap
  ## Iterators 
      types -> iter 
            -> iter_mut
            -> into_iter
  ## Vectors
   
  