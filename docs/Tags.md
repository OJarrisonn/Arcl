# Tags

`arcl` doesn't support inheritance, but we do support tagging. Tags are warranties that make sure that some type supports a given operation.

## Constructing Tags

To create a tag one needs to use a `cons` but the used identifier must start with `#` and be kebab-case. Then, you can list the *dependent tags*, it means, which tags the type must have in order to use this tag, and then a list of the methods to be implemented (or their implementation)

```c++
cons #eq {
    // No default implementation
    eq = meth &self (&Self other) -> Bool;

    // Default implementation provided
    neq = meth &self (&Self other) -> Bool
        !(self == other);
}

cons #ordeq #(ord, eq) {
    // Default implementation provided
    lt = meth &self (&Self other) -> Bool 
        self < other || self == other;

    gt = meth &self (&Self other) -> Bool
        self > other || self == other;
}
```

Methods with default implementations can't be overwritter, on the other hand, methods without default implementations must me implemented.

### Conflicting Names

If a type is tagged with tags which methods names are equal, just prepend the method name with the tag name

```c++
cons #tag1 {
    foo = meth &self () -> ();
}

cons #tag2 {
    foo = meth &self (Int a) -> Bool;
}

cons Bar = Int impl #(tag1, tag2) {
    #tag1:foo = meth &self () -> () {
        ...
    };

    #tag2:foo = meth &self (Int a) -> Bool {
        ...
    };
}

decl Bar b = Bar <$ 17;

#tag1:foo(b);
#tag2:foo(4, b);
```

### Freedom

A tag may wish that the type that implements the tag has some freedom when writting the method, for this, the tag definition can use underscores to let open to the developer.

Also, you may just say that you want a `polymeth` with given input or output

```c++
cons #foo {
    bar = polymeth _ -> Bool 
}
```

## Standard Library Tags

`num`: basic arithmetical operation has to be supported (addition, multiplication, ...) and it can have castings from other numerical types.

`eq`: the type can be compared on equality, it means, support `==` and `!=`

`ord`: the type can be ordered, it means, support, `<`, `>`

`ordeq`: the type is `ord` and `eq`, it means it also supports `<=` and `>=`

`char`: the type is character-like

## Compound Tags

As noticed, when something asks for tags and you need to pass more than one tag, you put them into a compound tag, actually, every tag is already a compound tag:

```
#(tag) = #tag

#(tag-1, tag-2, tag-3, ...)
```
