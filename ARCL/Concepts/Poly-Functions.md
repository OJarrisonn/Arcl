```cpp
decl polyfunc< int -> int, float -> float > increment = polyfunc< int -> int, float -> float > {
	func<int a> -> int {
		a + 1
	};
	func<float a> -> float {
		a + 1
	};
};

decl int num = 10;
print(increment num);
```