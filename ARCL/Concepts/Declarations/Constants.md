Constants are boxes which value can't be changed after the declaration. This can be useful for regular values but it's essential for methods declaration (in most cases probably you don't want someone to change the behaviour of your functions).
```cpp
// A constant integer
decl a = 5;

// A function which can't be changed
decl f = func float x -> float x^2;

// A type declaration
decl MyInt = int;
```

In order to declare a constant, just don't declare it using ``var``.  Also, constants can't be redeclared nor assigned as ``write`` (for obvious reasons).

When talking about *pointing* boxes there are two scenarios. The *pointing* box is constant, or the box being pointed is constant.

```cpp
decl var int a = 8; //a is variable 
decl int b = 11; // b is constant

decl var &int c -> b; // c is a reading box pointing to a constant.
c -> a; // As c is variable, it can point to other information

decl *int d -> a; // d is writing box and constant, so can't point to anything else in the future
```

Keep in mind that *writing* boxes can't point to constants.