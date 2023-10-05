Eventhough one can give an operator any desired behavior, they do have a meaning assingned to it.
# Assignments
There are two ways of assign a value to a box in ARCL
## Copy Operator (=)
Most languages call this an "assignment", but we rather call it "copying" because this operators copies the right value into the box to the right.
```cpp
decl int b = 10;
decl a = b; // a now has a deep-copy of the value stored in b
```

## Pointing Operator (->)
While ```=``` copies values, ```->``` will just point to it
```cpp
decl int a = 8;
decl &int b -> a;
decl :int c -> a;
b = c - 4; // This copies the value of a - 4 to the box that b is pointing to
```
# Pointing Box Operator (&)
This operator is used to turn a box type into a pointing box type

# Seeking Box Operator (:)
This operator is used to turn a box type into a seeking box type