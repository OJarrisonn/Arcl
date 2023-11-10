```
<consexpr> := cons <name> :<generics>?: <typespec> <typeimpl>
```
# Generics
If generic types are needed for your type, pass them between ":" before the type specification

```
cons Pair : T1, T2 : (T1 fst, T2 scd);
```
## Tagged Generics
You also may wish that your generics aren't so generic and that they have some tag
```
T <#tag1, tag2, tag3, ...#>
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
<vsetexpr> := vset <compound> | vset <named_compound>
```
```
vset (Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday)
vset (int )
```
## Objects
Objects in ARCL are just protected named [[Compound Type|compounds]]
```
<objexpr> := < <fields> >
<fields> := (read|write)? <type> <name> | <fields>, (read|write)? <type> <name>
```
## Type Alias
By giving another type's name as the specification you create an alias
```
cons MyInt int;
```
# Type Implementation
Implementations are ways of adding custom behaviour to manipulate the data stored by your type
```
<typeimpl> := impl <methods>
<methods> := <method> | { <method>;* }
```
## Section
A section is a piece of the implementation. A section can be associated to a single or to multiple associated functions
```
<method> := <methname> <afuncexpr>
		   | <methname> <compound:afuncexpr>
```
## New Method
The method that defines how a new value of the give type is defined, it can take multiple different functions, but all of them must return the ``Self`` type. The functions defined here will be called when the type is instantiated.
```
new self <namedcompound> -> Self <expr:Self>
```

## Casting Method
A casting is basicly converting a type into another type using some provided function
```
cast T <expr> // This try to cast the <expr> to the type T
```
## Adding Tags
For a type adquire some tag, it must implement the required functions by that tag
```
impl <#tag, tag, ...#> { ... }
```
# Calling Methods
To call a method for an expression, a variable or a constant, just call it using ":"
```
cons Pair : T : (T fst, T scd) impl {
	cast &self () -> [T; 2] [self.fst, self.scd];
	swap @self () -> () self = (self.scd, self.fst);
}

main {
	decl var Pair p = (35, 42);
	decl [int; 2] a = p:cast();
	print `{p} -> {a} `;
	
	p:swap();
	println `-> {p}`;
	 
	//Saída: (35, 42) -> [35, 42] -> (42, 35)
}
```