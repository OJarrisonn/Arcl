# Code
As everything in ARCL is an expression, so general runnable expressions. Every "command" in ARCL that receives some sort of input expect that to be a single expression, but you may want to more than just a single line of code to be executed in order to executed, and that's where ``{ }`` come in. ``{ }`` are used to group lines of code into a single expression
```
<codeexpr> :=
| { <expr>; <expr>; ...}
```
Also, code expressions support the ``retn <expr>`` expression which is an endpoint of the grouped code and sets the value of this expression. A code expression can have infinite ``retn`` but all of them must return the same type
# Path
Path expressions are used when [[linking]] modules and packages, it uses unix-like path.

Some aliases:
- **Standard library**: ``/``
- **Main package**: ``~``
- **Current package**: ``.``
- **Parent package**: ``..``
- **All**: ``*``
## Examples
```
link /fs; // A module from the standard library
link mlog -> ~/util/my_logger; // A module from the current program
link external/some_module; // A module from some external library
```