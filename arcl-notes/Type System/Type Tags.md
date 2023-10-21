Tags are used for describring features that a type must implement. That's very useful for generic types. When specifing the tag implementation, one must provide the feature list and provide the [[Function Expressions|associated function]](s) signature(s) using don't cares (``_``) for the parts of the signature that doesn't import.
```
cons #<tagname> impl [
	...
]
```

```
const #
```