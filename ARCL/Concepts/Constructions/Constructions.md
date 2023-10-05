Constructions are the way of construct new types in ARCL
```cpp
cons Nums 1~=10; // Integer interval [1,10]

cons PersonHeight 0.~2.6; // Float interval [0.0, 2.6)

cons Country set { // Set of possible values
	Brazil,
	USA,
	Spain,
	England,
	Russia
};

cons Person struct {
	decl read var String name;
	decl read var usint age;
	decl read var PersonHeight height;
	decl read var Country country;

	decl read new = func <String name, usint age, PersonHeight height, Country country> -> Self {
		decl mov var Self this; // Can be ommited
		this.name = name;
		this.age = age;
		this.height = height;
		this.country = country;
		
		this // Can be ommited
	};
};

cons List struct :: <T> {
	decl var T[.] elements;
	decl read var uint length;
	decl read var uint max_length;

	decl read new = func <void> -> Self {
		this.elements = resr 0;
		this.length = 0;
		this.max_length = 0;
	}; 

	decl read push = method <*Self, T item> -> void {
		if this.length == this.max_length {
			this.max_length *= 2;	
			decl mov var T[.] new_elements = resr (this.max_length * |T|);
			
			for i in 0..this.length {
				new_elements[i] = this.elements[i];
			};

			uncl this.elements;
			this.elements = new_elements;
		};

		this.elements[this.length] = item;
		this.length += 1;
	};
};
```