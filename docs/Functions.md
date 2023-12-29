# Functions

`arcl` treats functions as regular values that support a special operation: _calling_

```c++
decl sum = func (Int a, Int b) -> Int a + b;

decl multi_print = func (String msg, Int times) {
    (0..times).foreach(func (Int _) {
        print `${&msg}`;
    })
};

sum(5, 7); // 12
multi_print("Hello World.", 3); // Hello World.Hello World.Hello World.
```

Functions may have an input, an output and must have a body. The big point here is that functions take only one value as their input, if you need more, then you use compound types, same for it1s output.

The body can be a block of code or simply a single statement. Inside the body you may use the word `retn` to end the function execution and set it's output value

## Calling

Functions call are made in many ways:

### Operators

Symbolic operators like: `+`, `-`, `<$`, etc are simply a call to a function

### Passing

Putting a function expression (something that evaluates to a function) followed by another expression will execute the function with the passed arguments

#### Curring

`arcl` support curring in a similar way that `Haskell` does. Passing a argument that does not suffice the need of the function will create a new function.

```c++
sum 8; // func (Int _1) -> Int sum (8, _1);
```

By calling sum passing only `8` as it's argument we create a function that receives an integer and completes the call.

```c++
decl complex_operation = func (Int num, Int num2, Float f, String name) -> (Int, String) ...;

complex_operation (3, 10, .6); // func (String _1) -> (Int, String) complex_operation (3, 10, .6, _1);

complex_operation (_, 4, _, "Hello"); // func (Int _1, Float _2) -> (Int, String) complex_operation (_1, 4, _2, "Hello");
```

Passing `_` will create a function which asks for this omitted parameter

Also, `arcl` support a special kind of curring where you can pass a function that returns `T` in a place where you expected a value `T` to create a function

```c++
decl adds_4 = sum (4, sum); // func (Int _1, Int _2) -> Int sum (4, sum (_1, _2));

adds_4 (1, 2); // 7
```

When dealing with binary infix operators, the underscores are quite helpful

```c++
decl double_it = (2*_); // func <T1 #number>(T1 _1) -> T1 2 * _1;

double_it 3; // 6
```

### Pipe Forwarding

The operator `|>` is used as it follows:

```c++
x |> f = f x
```

It's useful for increasing readability in long chained functional code

```c++
decl my_list = [|4, 8, 12, 4, 9, 7, 5|];

decl res = my_list // [|4, 8, 12, 4, 9, 7, 5|]
    |> List:filter (Int:is_odd) // [|9, 7, 5|]
    |> List:reverse // [|5, 7, 9|]
    |> List:map (2*_) // [|10, 14, 18|]
    |> List:sum; // 42
```

## Special Function Notations

You already know the regular function notation  but `arcl` supports some special ones:

### Pure Function

If your function output depens only on the inputs and it doesn't modify values outside the scope of the function, then it's pure (most likely). In this case, use the `pure` word instead of `func`.

```c++
decl square_it = pure <T #number> (T num) -> T num ^ 2;
```

Why use this? Remember, be meaningful. Some applications may need the warranty that the function being passed is pure. If not, no worries, pure functions can be upcasted to regular functions implictly.

### Procedure

Procedures are pretty different from pure functions. They are functions with no return value that probably will read and/or write in variables outside the function scope.

In this case, use the `proc` word instead of `func`, and similar to pure functions, procedures can be upcated to regular functions.

```c++
decl var a = 4;
decl modify_a = proc [@a](Int v) {a = v};
```

The `[]` before the input are optional and can demonstrate how the procedure is capturing the external value `a`. We'll talk about this later in "closures".

## Nullating, Failing and Crashing

Your function may not return exactly the value you meant. For instance: `List:index_of` takes an element and a list of such element and returns the index of the element. But wait, the list may not contain such element. So the result may be `null` (`?`).

```c++
decl index_of = func <T: #eq> (T elem, [|T|] list) -> ?Uzint ...;
```

Now think about a function that converts a string to a number, it may not work if the string chars are not all numeric characters. So it may fail (`!`).

```c++
decl parse_int = func (String text) -> !Int {
    decl text = List<Char> <$ text; // Converts String to List<Char>
    if !(text:all (Char:is_numeric)) 
        retn Fails::ParseFail("The string contain non numeric characters");
    
    text
        // Converts each Char to a !Int
        |> List:map (Char:as_int) 
        // It will not fail, we know because of the if above
        |> List:map (pure (!Int elem) -> Int elem:unwrap()) 
        // Uses a fold to convert the list of numbers into
        |> List:foldl (pure (Int elem, Int acc) -> Int (acc*10 + elem), 0, _)
    // No semicolon and it's the last statement, so this is the return value of the function
};

parse_int "1234"; // 1234
parse_int "12E4"; // ParseFail
```

May your failure be more severe and you can't just return a failure, you need to crash the program. That's where crashable (`~`) comes in.

```c++
decl sqrt = func <T: #(num, ord)> (T num) -> ~T {
    if num < 0 {
        crash `${num} is negative`;
    }

    num ^ (1/2)
}

sqrt 16; // 4
sqrt -1; // ~CRASH~ [sqrt.arcl:3] :: -1 is negative
```