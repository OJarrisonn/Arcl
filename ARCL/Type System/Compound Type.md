One of ARCL's most important features is the compound type, basicly everything uses compounds.
```
<compoundexpr> := ( <exprs> )
<exprs> := <expr> | <expr>, <exprs>
```
# Named Compounds
Instead of using numbers to access values of a compound, one may give names to each element of the compound.
```
decl (int age, float height, Str name) person = (26, 1.8, "James");
println `Person is {person.age} years old and it's called {person.name}`;
// Person is 26 years old and it's called James
```
# Protected Named Compounds AKA Objects
Those compounds have special descritors before the type which set the access level for each field of the named compound.
## ``read``
It means that the field value can be obtained from the outside, but can't be written
## ``write``
It means that the field value can be obtained and written by the outside of the type
## ``priv``
It means that only the compound can access it's value

```
decl var <read int number, write bool fact, char c> my_var = <18, false, 'h'>;
println `{my_var.number} {my_var.fact}`;
my_var.fact = true;
// println `{my_var.c}`; // Forbidden, c is private
```
If it isn't clear, objects were meant to be used just in [[constructions]].
# Compound Coersion
ARCL can make some convertions in compile-time on compounds:
## Single Coersion
Any type ``T`` can be coersed to ``(T)`` and vice-versa. That's why ARCL accept both ``println "Hello World"`` and ``println("Hello World")``.
## Named Coersion
A named compound can be coersed from an unnamed compound if the types of each field match
## Object Coersion
An object can be coersed from an unnamed compound in a similar way done with named coersion
```
decl <read int n1, int n2, write int n3> bar = (13, 71, 52);
```
# Compound Operations
## Accessing
In order to get a specific value of a compound, use the `.` operator and the position of the element for unnamed compounds or by the field name in named compounds. But named compounds do not support the use of the position;
```
decl int my_num = ('v', true, 12, "banana").2;
```
## Concatenation
Using the ``:`` operator it's possible to concatenate compounds into new compounds. For example:
```
decl (int, bool, char, float) my_compound = (32, true) : ('g', 4.2);
```
## Spliting
Using the ``\`` operator you can split a compound into two parts
```
decl ((char, Str), (int, int, bool)) my_compound = ('s', "word", 5, 9, false)\2;
```
## Dimension
Using the `| |` operator you can obtain the dimension of the compound
```
assert(|('b', 5, true)| == 3);
```