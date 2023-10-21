The main [[Expressions|expression]] is the entry point in the ``main`` module and it's unique in an ARCL program. It means that it's forbiden to have anything called ``main`` in any other module and only 1 ``main`` is allowed in the ``main`` module.

```
<mainexpr> :=
| main <expr> // A simple main
| main ! <expr> // A main that can fail
| main args -> <expr> // A main that receive some input but can't fail
| main args -> ! <expr> A main that receive some input and can fail
```
# Examples
## Hello World
The common way
```
main {
	println ("Hello World!");
};
```
Or the minimal way
```
main println "Hello World";
```