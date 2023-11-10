```
<declexpr> :=
| decl mov? var? <type>? name (= <expr> | ~> (&|@) identifier | := identifier)?
```
Declarations are the way of binding values to names in arcl. In declarations the type can be ommited, one can define the name to be variable, movable or the name can be unassigned if the type is specified.
```
// creates a constant
decl name = expr;
// creates a variable
decl var name = expr;
```

## Assingments
There are three ways of assign values to names in arcl: assigning, pointing, and deep-copying
### Assing (=)
The assing `=` operator is used to set the value of a name by copying the value if it's copiable or by moving it if it isn't copiable or if it's movable.
If a type is a collection or if it contains a collection so it isn't copiable. Otherwise it is copiable.
```
decl var number = 6;
number = 21;

decl my_list = [1, 2, 3, 4, 5];
decl other_list = my_list; // From this point and beyond, my_list is unassigned
```
### Point (~>)
The point operator is used only for references (both read/write references). It changes to where the current name is pointing.
```
decl age = 26;
decl r_age ~> &age;
// decl w_age ~> @age; -- this is forbbiden both because age is a constant and because we already have a reading reference
```
### Deep-Copy (:=)
By using the deep-copy operator, we completely copy the value from another name. For each piece of data in that value, if it's copiable we simply copy it, if it's a collection we clone it, if it's a reference we create another reference to the same thing.
```
decl my_list = List:int:from([1, 2, 3, 4, 5, 6]);
decl my_second_list := my_list;
```