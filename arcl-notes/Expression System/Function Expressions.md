Functions are values that receive an input and can return an output. The input and is given as a named compound, while the output is a type. These together form the function signature. Then comes the function body as an expression.
```
<funcexpr> := <namedcompound> <instruction> | <namedcompound> -> <typename> <instexpr>
```
# Associated Functions
Those special functions are used in [[constructions]] for the implementations' sections. The difference is basicly that an associated function is a function preceeded by ``self``, ``&self`` or ``@self``, to mean that the associated function will move the value, just readed it or modify it, respectively.
## Example
```
cons Pos <read int x, read int y> impl [
	// Moves the original value
	new self (int x, int y) ~> Self { self.x = x; self.y = y; self }
];
```
# Returning
A ``retn`` expression will end the function early and set the value of the function to the passed expression
# Pure Functions
A pure function doesn't access nor modify anything outside the function. It means no side-effects and that the output only depends on the inputs
```
pure (int x, int y) -> int x + y
```
# Procedures
Procedures on the other hand access and modify stuff outside the function and can't return values
```
decl a = 10;

decl p = proc (int x) a = x;

p 5;
println a; // 5
```
# Parallel
Parallels can be any other kind of function, but must return an await type. The call of a parallel is non-blocking.
```
decl pl = prll pure (int t, int a) -> ...int {
	(0..t):foldl(0, proc (int acc, int _) acc += a); 
};

decl k = pl (1000000, 1); // k is a  ...int and this line is a non_blocking
decl l = ...k; // this line will await for k to be calculated
```