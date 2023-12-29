# Generics

Generic types are basicly types as parameters for a type definition or a function call. They are often passed between `<` `>` after the type name.

Generics on types:

```c++
cons Wrapper <T> = T impl {
    new = meth (T value) {
        self = Self <$ value;
    };

    unwrap = meth self () -> T T <$ self;
}

decl w = Wrapper<Int>:new(81);
w:unwrap(); // 81
```

Generics on functions:

```c++
decl wrap = func <T> (T value) -> Wrapper<T> Wrapper<T>:new(value);

wrap(70); // T can be infered

decl parse = func <T> (String text) -> !T { ... };

parse<Int>("456"); // T can't be infered, it must be set

decl idk = func <T, U> (T value) -> U { ... };

idk<_, Int>(3.1415); // T can be infered, but U can't, use a _ for easy
```

## Not so Generic

Generics are good because the let your functions and types handle any type, but that also is a bad thing. You can't do any operation over this types because you have no warranty of what operations they support. That's when tags come in

```c++
decl sum = func <T #sum> (T a, T b) -> T a + b;
```

The above function receives two values of a type that is tagged as `#sum` and return the same type by summing them. We can do this because for a type be tagged as `#sum` it must support being summed to itself.

Addionatly, we can use tags in a method definition inside a `cons`

```c++
cons List <T> = ... impl ... {
    ...
    sum = meth <T #sum> &self () -> T {
        ...
    };
    ...
}
```

By reusing the same type `T` we make sure that this `sum` method only works if the `List` is of summable elements. Otherwise our program gonna crash.
