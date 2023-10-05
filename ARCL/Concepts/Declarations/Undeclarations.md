Once something is declared, it can be undeclared explicitly or implictly.
# Explicity
``` cpp
decl int a = 5;
uncl a;
// From this point and beyond the identifier a is no more declared
```
Once a undeclaration is done, any boxes that point to it are also invalid. In branching and looping structures an explicity undeclaration can be done only in local declarations.
# Implicity
All boxes that weren't moved out are undeclared when the scope is finished.