ARCL (read as "ARCLE") is a programming language which focus on being dynamic and metaprogrammable but keeping memory safety. We want to create a language with a pleasant interpreted language sintax but with the performance and assurances of being compiled.
# Approach
ARCL approch for problem solving is highly supported by functional and object oriented programming. ARCL also provides native support for parallelism. And finally one of the coolest features of ARCL is that almost everything is an [[expressions|expression]]. 
# Naming Conventions
ARCL adopts ``snake_case`` for almost everything. The only exceptions are global constants which use ``ALL_CAPS`` and [[Constructions|non primitive types]] which uses ``PascalCase``.
# Program
An ARCL program is a bunch of ``.arcl`` files (we gonna call them "modules") and folders (we gonna call them "packages") with a entrypoint file called ``main.arcl``. 
A program can be both an executable or a library, the only difference between them is that an executable program was a ``main`` attribution in the ``main`` module (the ``main.arcl`` file, for clear). 
If a package or a module name starts with a ``_`` it makes them local, so they are only accessible for they "brother" packages/modules.
For linking your modules a ``link`` is used. 
An ARCL module is a list of expressions, that maybe acessible from the outside. But not every expression is executed, just ``decl``, ``defn``, ``cons``, ``link``, ``test`` and the ``main`` (if in the ``main.arcl`` of a executable program) expressions are executed.
# APE (ARCL Program Environment)
An ARCL program can grow and need to use external libraries, that's when APE comes in. APE is used to manage a program and it's dependencies
```markdown-tree
my-program
	bin
		...
	lib
		...
	my_package
		...
	tests
		...
	main.ape
	main.arcl
	my_module.arcl
```
- ``bin``: your generated compiled files (executable or library)
- ``lib``: your external libraries
- ``tests/``: the module for your tests
- ``main.ape``: the file which holds information about your program
## CLI
- ``ape init``: setup your folder structure
- ``ape build [release]``: builds your program
- ``ape run``: builds the program in debug mode and runs it
- ``ape lib add <libname> [version]``: adds a new external library
- ``ape lib rem <libname>``: removes an external library
- ``ape lib update [<libname>]``: updates a library or all the libraries
- ``ape test [<testname>]``: runs one or all the tests inside the ``tests`` module
https://bnfplayground.pauliankline.com/?bnf=%3Cprogram%3E%20%3A%3A%3D%20(%3Cws%3E*%20%3Ccommand%3E%20%3Cws%3E*%20%22%3B%22%20%3Cws%3E*)*%0A%3Cws%3E%20%3A%3A%3D%20(%22%20%22%20%7C%20%22%5Ct%22%20%7C%20%22%5Cn%22%20%7C%20%22%5Cr%22%20)%0A%0A%3Ccommand%3E%20%3A%3A%3D%20%3Cdecl%3E%0A%3Cexpression%3E%20%3A%3A%3D%20%3Cident%3E%20%7C%20%3Cinteger%3E%20%7C%20%3Cfloat%3E%20%7C%20%3Cstring%3E%20%7C%20%3Cchar%3E%0A%0A%3Cident%3E%20%3A%3A%3D%20(%22_%22%3F%20(%5BA-Z%5D%20%7C%20%5Ba-z%5D)%20(%5BA-Z%5D%20%7C%20%5Ba-z%5D%20%5B0-9%5D%20%22_%22)*)%20%7C%20%22_%22%0A%3Cinteger%3E%20%3A%3A%3D%20%220%22%20%7C%20(%5B1-9%5D%20%5B0-9%5D*)%0A%3Cfloat%3E%20%3A%3A%3D%20(%3Cinteger%3E%20%22.%22%20%3Cinteger%3E%3F)%20%7C%20(%22.%22%20%3Cinteger%3E)%0A%3Cstring%3E%20%3A%3A%3D%20%22%5C%22%22%20%3Ccharinner%3E*%20%22%5C%22%22%0A%3Cchar%3E%20%3A%3A%3D%20%22%27%22%20%3Ccharinner%3E%3F%20%22%27%22%0A%3Ccharinner%3E%20%3A%3A%3D%20%5BA-Z%5D%20%7C%20%5Ba-z%5D%20%5B0-9%5D%20%7C%20%22!%22%20%7C%20%22%40%22%20%7C%20%22%23%22%20%7C%20%22%24%22%20%7C%20%22%25%22%20%7C%20%22%26%22%20%0A%09%09%09%7C%20%22*%22%20%7C%20%22(%22%20%7C%20%22)%22%20%7C%20%22_%22%20%7C%20%22%2B%22%20%7C%20%22%3D%22%20%7C%20%22-%22%20%7C%20%22%5B%22%20%7C%20%22%7B%22%20%7C%20%22%7D%22%20%0A%20%20%20%20%20%20%20%20%20%20%20%20%7C%20%22%5D%22%20%7C%20%22%2F%22%20%7C%20%22%3F%22%20%7C%20%22%5C%5C%22%20%7C%20%22%7C%22%20%7C%20%22%3A%22%20%7C%20%22%3B%22%20%7C%20%22%3E%22%20%7C%20%22.%22%20%7C%20%22%3C%22%20%0A%20%20%20%20%20%20%20%20%20%20%20%20%7C%20%22%2C%22%20%7C%20%22~%22%20%7C%20%22%5E%22%20%7C%20%22%60%22%20%7C%20%22%27%22%20%7C%20%22%5C%22%22%0A%0A%3Cdecl%3E%20%3A%3A%3D%20%22decl%22%20%3Cws%3E%2B%20%22var%22%3F%20%3Cws%3E%2B%20%3Cident%3E%20%3Cws%3E*%20(%22%3D%22%20%3Cws%3E*%20%3Cexpression%3E)%3F&name=arcl-grammar