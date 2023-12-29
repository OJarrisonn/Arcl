# Constructed Types

This is the way to create types and assing methods to it. Methods are just operations executed on a value.

The construction syntax is:

```
cons <Identifier> = <type> (impl <tags>? \{ <methods> \})?
```

The word `cons` followed by a PascalCase identifier, a type definition, then optionally an implementation for some (or none)
tags and methods.

The reserved word `self` refers to the value in which the method is being called, while `Self` refers to the type being constructed.

```c++
cons Person = {
    read String name,
    Int age,
    Float height,
    write String favorite_color
} impl #eq {
    new = meth (String name, Int age, Float height) {
        self.name = name;
        self.age = age;
        self.height = height;
        self.favorite_color = "purple";
    };

    cast = polymeth (
        meth &self () -> Int self.age,
        meth &self () -> Float self.height
    );



    eq = meth &self (Self other) -> Bool self.favorite_color == other.favorite_color;
}
```

Here we create a Person object with four fields and 3 methods.

The `new` method is called whenever a new Person is created. It's special because you don't need to use the word `meth`, nor `polymeth` for it (since the value doesn't exists yet) and you don't need to specify the return type, nor value.

The `cast` is polymethod (a method with multiple definitions, like a `polyfunc`) called when the `<$` operator is used with Person.

The `eq` method is also a polymeth from the `eq` tag, but we're only providing a single definition. It's called by the `==` and `!=` operators.

```c++
decl p = Person:new("John Doe", 42, 1.85); // We access methods with `:`
decl i = Int <$ &p; // 42

if p == (Person <$ ("Doe Jonh", 24, 5.81, "purple"))
    println "Same color"; // Same color
```

Here we are casting a compound to an object, and it works since the object and the compound field matches, if they don't we'll receive a compile error. `(String, Int, Float, String) <$ p` also works.

## Methods

Let's talk more about methods semantics.

They are just functions that take a reference to a value (or the value it self), so they can be curried, used in pipe forwarding, etc.

A method can be only declared inside an `impl` block as such:

```
<identifier> = meth (&|@)?self <func_signature> <expr>
```

It means, an snake_case identifier followed by `=` and the word `meth`, then the borrowing type for the value (read, write or move) and then the regular function signature and its body. The composition `meth (&|@)?self <func_signature>` is called: method signature.

In addition, the following calls are the same:

```c++
decl x = [|4, 3, 2, 1|];

x:reverse();
List:reverse(x);

x:filter(Int:is_even);
List:filter(Int:is_even, x);
```

The methods can be called directly in the value or in the type and passing the value as the last parameter.

Maybe you're wondering: "why the last?". The answer is: curring. By default, if you don't pass a compound that is shorter than the function asks for input the remaining args are curried.

### `new`

The `new` method uses a special `meth` signature without a borrowing specifier (there is no value to be borrowed) and no return value (it's obvious what `new` must return).

## Polymorphic Methods

Methods, as functions can be polymorphic by using `polymeth` instead of `meth` in it's declaration. Then using the similar structure, but using method signatures instead of regular function signatures.

## Not Just Objects

We've shown examples of `cons` over objects because it's the most intuitive way, but in fact you can use `cons` for every single type

### Type Clones

`arcl` let you create a clone of a type

```
cons T_ = T
```

In the above context, `T_` is a clone of `T`, it has all its data structure, methods and tags. Also, there is an auto implemented cast from `T_` to `T` and vice-versa.
