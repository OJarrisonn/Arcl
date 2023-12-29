# Types

Types are the things that classifies values in `arcl`, moreover, they give you assurances about the operations that can be done with a given value.

## Primitive Types

We have numeric types: Int, Uint, Sint, Usint, Lint, Ulint, Zint, Uzint, Float, Double

Character types: Char, Lchar

Boolean: Bool

## Derivate Types

The derivate types are Nullable (`?T`), Failable(`!T`) and Crashble(`~T`)

## Collections

We have the `Array` type (`[T; l]`) of given type and length. The `List` type `List<T>` of variable length. The `Dict` (dictionary) type `Dict<K: #eq, V>`.

The biggest difference between `Array` and `List` if that the second one is meant for the data to be operated as a whole with the `List` functional methods

## Compound Types

The special type that can concatenate other types into a single piece.

```c++
decl (Int, Float, Bool) my_value = (42, 1.85, false);

my_value.0; // 42
my_value.2; // false
my_value.5; // ~CRASH~ [compound.arcl:5] :: 5 is not a field of (Int, Float, Bool)
```

We have some notes:

1. `()` (the unit type) is also called `Void`, and it's only value `()` is also called `null`.
2. `(T)` and `T` are the same thing
3. Any compound expression is written between `(` and `)`

### Named Compounds

You may wish not to access a field using a number, but rather a name, then just give it a name.

```c++
decl (Int age, Float height, Bool married) person = (42, 1.85, false);

person.age; // 42
person.car; // ~CRASH~ [named_compound.arcl:4] :: car is not a field of (Int age, Float height, Bool married)
```

### Objects

Objects are just named compounds with one more feature: access control. Objects fields can be private (by default), accessible to read and/or writting. Obviously, an object's internal operations can access all its fields no matter the specified access level.

```c++
decl var {read String name, write Int age, Bool married} person = ("John Doe", 42, false);

person.name; // "John Doe"
person.name = "Doe John"; // ~CRASH~ [object.arcl:4] :: name field of {read String name, write Int age, Bool married} is not writtable
person.married; // ~CRASH~ [object.arcl:5] :: married field of {read String name, write Int age, Bool married} is not readable
```

The object type is more useful when in a constructed type rather than being used annonymously.

## Alternative Types

Alternative Types are like Rust enumerations, you can specify all the values of a type and also add some adjacent data.

```c++
{| Number Int, Text String |}
```

## Referencing Types

Maybe you don't want to hold a value in your variable, but reference an **existing** value. Using `&` (a read reference) or `@` (a writting reference) you may reference existing values (no dangling pointers) and use them.

```c++
decl swap = proc <T>(@T a, @T b) {
    decl c = *a; // * means: the value being referenced by
    *a = b; // Where we can ommit the second * because it's obviously needed
    *b = c; // We are not ommiting * here, c is T not @T
}

decl var_1 = 10;
decl var_2 = 35;

swap (@var_1, @var_2); // var_1 = 35, var_2 = 10
```

There are some rules about how to use references:

1. No reading and writting references at the same time
2. You may have as many reading references as you wish
3. You may have only one writting reference
4. You can't reference variables that last shorter than yours
5. You can't write to a constant

## Numeric Intervals

The following notation creates numeric intervals (`a` and `b` are compile time known constants, and `a` < `b`)

if `a` and `b` are both integers:

- `a..b`: starts in `a` goes up to `(b-1)`
- `a..=b`: starts in `a` goes up to `b`

If one of them is a floating point number:

- `a..b` -> $(a, b)$ open interval
- `a..=b` -> $(a, b]$ left open interval
- `a=..b` -> $[a, b)$ right open interval
- `a=..=b` -> $[a, b]$ closed interval

## Type Casting

`arcl` has a strong typing but you may transform a type into another using the cast type operator `<$` which calls the casting method `cast` specified if it exists.

```c++
decl pi = Int <$ 3.14; // 3
decl chars = List<Char> <$ "Hello World"; // [|'H', 'e', 'l', 'l', 'o', ' ', 'W', 'o', 'r', 'l', 'd'|]
```

## Type Equality

Two types are equal when:

1. They were defined in the same place
2. Both has the same data and one of them is an annonymous type