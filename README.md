# ARCL - the Adaptive Runtime Creative Language

ARCL is meant to be a compiled programming language that supports meta-programming features. So you can define the program behavior through the program.

## Boxes

Every information is stored in a box (variables, constants, functions, data structures), and boxes can both hold a primitive value (a literal box) or point to a value stored in the memory (a pointing box).

```c++
// Creating a variable
decl int myNumber = 10; 

// Creating a constant
decl const<int> myConst = 20; 

// Creating a box which points to myNumber
decl int myNumberP -> myNumber; 

// Creating a box which points to a function without input nor output
decl func myPrint -> func { 
    io::print("Hello World"); 
};

// Creating a box that points to a function with input and output
decl func<(int, int), int> mySum -> func<(a, b), c> { 
    c = a + b;
};

// Creaing a box that points to a class definition
decl class myClass -> class {
    decl int a;
    decl bool b;

    decl pub func<(int, bool), self> new -> func<(a, b), self> {
        self.a = a;
        self.b = b;
    }
};

// Creating a box that points to a enum definition
decl enum MyEnum -> enum {
    entry1,
    entry2<int>,
    entry3<bool, char>,
    entry4<func>
}

```

### Operators
- **Copy**(`=`): copies the value from the right to the box in the left
    ```c++
    decl char myBox = 'v'; // myBox holds the char 'v'
    ```
- **Point**(`->`): the box in the left points to the value in the right
    ```c++
    decl &char mySecondBox -> &myBox; // mySecond box points to myBox
    decl &char mySecondBox -> myBox; // mySecond box points to myBox
    // Both work because the compiler can infer that you are pointing to myBox, not the value
    ```
- **Reference**(`&`): refer to the box it self, not the value that it holds
    ```c++
    decl &char myThirdBox -> mySecondBox; // Here, myThirdBox points to myBox
    decl &&char myThirdBox -> mySecondBox; // Here it points to mySecondBox
    decl &&char myThirdBox -> &mySecondBox; // Here it points to mySecondBox
    ```

### Primitive types
- **int\<len\>**: Signed integer numbers with a specified byte size
    - **int<1>** or **sint**: 1-byte long signed integer 
    - **int<4>** or **int**: 4-bytes long signed integer
    - **int<8>** or **lint**: 4-bytes long signed integer
    - **int<16>** or **xint**: 16-bytes long signed integer
- **uint\<len\>**: Unsigned integer numbers with a specified byte size
    - **uint<1>** or **usint**: 1-byte long unsigned integer
    - **uint<4>** or **uint**: 4-bytes long unsigned integer 
    - **uint<8>** or **ulint**: 4-bytes long unsigned integer 
    - **uint<16>** or **uxint**: 16-bytes long unsigned integer
- **float\<len\>**: Real numbers with specified byte size
    - **float<4>** or **float**: 4-bytes long real number
    - **float<8>** or **double**: 8-bytes long real number
    - **float<16>** or **treble**: 16-bytes long real number
- **bit\<len\>**: A bitstring of specified bit size
    - **bit<1>** or **bool**
    - ...
    - **bit<256>**
- **char\<encoding\>**: A char type of some encoding
    - **char\<ascii\>**: Ascii char
    - **char\<utf-8\>**: UTF-8 char
    - **char\<utf-16\>**: UTF-16 char
- **()**: Compount type, a data type that stores multiple data types within
    - **()** or **void**: The void type, AKA empty-type
    - **(int)** or **int**: Every simple type can be described as a simple-type
    - **(int, double)**: The dual-type, can be compoun of every type, even custom types and other compount types
    - **(char, (bool, (MyCustomType)))**: This is a dual-type of a char and a dual-type of bool and a simple-type of MyCustomType.
    - **(a, b, c)**: A triple-type
    - You can create compount types as long as you want  
- **func\<i, o\>**: The function type. ```i``` is the input type (can be compount) and ```o``` is the output type (also can be compound).
- **class\<i, g, t\>**: The class type. ```i``` is the inherited type, ```g``` are the generic types declaration and ```t``` are the tags annotations. All those can be compound.
    
