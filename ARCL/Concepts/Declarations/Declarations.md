Declarations are the way one uses declare the informations one's using in the program.
```cpp
decl var a = 10;
decl var float b = 5.5;
decl var name = "John Doe";
```

Declarations must follow this grammar
```cpp
decl [read|write|mov] [var] [typename] <identifier> [(=|->) <expression>];
```

- ```decl```: the keyword indicating that a declaration in on going
- ```read```: used only in struct fields. Means that the field can be read from the outside
- ```write```: used only in struct fields. Means that the field can be both read and written from the outside
- ```mov```: Means that the box information ownership can be moved to other boxes through a ``return`` statement or passed to a funciton argument.
- ``var``: means that the box value is variable, so it can be changed after the declaration
- ``typename``: the type of the box
- ``identifier``: an identifier to be the box name