Functions are values that receive an input and can return an output. The input and is given as a named compound, while the output is a type. These together form the function signature. Then comes the function body as an expression.
```
<funcexpr> := <namedcompound> <instruction> | <namedcompound> ~> <typename> <instexpr>
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