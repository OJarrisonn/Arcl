Definitions are the ARCL solution for defining custom operators. In a definition you define your syntax using ``$$`` to mark a value to be captured, and anyting else to mark the operators
```
<defnexpr> := 
| defn <opexpr> 

<opexpr> := #[ (<oplit>? <optk>)+ ]#
```

# Example
```
decl get_order = (int a) -> int {
	decl var order = 0; 
	
	if (a < 0) a *= -1;
	
	while a > 9 {
		a /= 10; 
		order++; 
	}
	
	retn order;
};

defn #[ % $$ % ]# {
	(int a) ~> get_order(a);
}
```