```
<consexpr> := cons <name> <typespec> <typeimpl>
```
# Type Specifications
The specifications are used to set the kind of data your type can store
## Sets
### Integer Interval
Using the ``..`` operator, you can create integer intervals that are open to the right. Using ``..=`` the interval is closed to the right.
```
<intinvexpr> := <expr:int>..<expr:int> | <expr:int>..=<expr:int>
```
```
0..5 // 0, 1, 2, 3, 4
0..=5 // 0, 1, 2, 3, 4, 5
```
### Float Interval
Again using ``..``, ``=..``, ``..=`` and ``=..=`` you can create intervals using floating point numbers.
```
// Mathematical notation
1. .. 2. // (1, 2)
1.5 =.. 3.0 // [1.5, 3)

```
### Enumerations AKA Value Sets
A ``vset`` expression let you create a type and define all the possible values using custom literals
```
<vsetexpr> := vset <values>
<values> := <identifier> | <identifier>, <values>
```
```
vset Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday
```
### Type Sets
A ``tset`` expression let you create a type that can store values of several types one at a time
```
tset int, float, (Str, bool), char?
```
## Objects
Objects in ARCL are just protected named [[Compound Type|compounds]]
```
<objexpr> := < <fields> >
<fields> := (read|write)? <type> <name> | (read|write)? <type> <name>, <fields>
```
## Type Alias
By giving another type's name as the specification you create an alias
```
cons MyInt int;
```
# Type Implementation
Implementations are ways of adding custom behaviour to manipulate the data stored by your type
```
<typeimpl> := impl <sections>
<sections> := <section> | <section>, <sections>
```
## Section
A section is a piece of the implementation. A section can be associated to a single or to multiple associated functions
```
<section> := <secname> <afuncexpr>
		   | <secname> [ <afuncarray> ]
<afuncarray> := <afuncexpr> | <afuncexpr>, <afuncarray>
```
## New Section
The section that defines how a new value of the give type is defined, it can take multiple different functions, but all of them must return the ``Self`` type. The functions defined here will be called when the type is instantiated.
```
new self <namedcompound> -> Self <expr:Self>
```

## Casting Section
A casting is basicly converting a type into another type using some provided function
```
cast T <expr> // This try to cast the <expr> to the type T
```
