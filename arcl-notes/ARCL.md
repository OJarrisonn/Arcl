ARCL (read as "ARCLE") is a programming language which focus on being dynamic and metaprogrammable but keeping memory safety. We want to create a language with a pleasant interpreted language sintax but with the performance and assurances of being compiled.
# Approach
ARCL approch for problem solving is highly supported by functional and object oriented programming. ARCL also provides native support for parallelism. And finally one of the coolest features of ARCL is that everything is an [[expressions|expression]] and every function is pure. 
# Naming Conventions
ARCL adopts ``snake_case`` for almost everything. The only exception are global constants which use ``ALL_CAPS`` and [[Constructions|non primitive types]] which uses ``PascalCase``.
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