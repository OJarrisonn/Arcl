Linking [[expressions]] are used to link modules and then use stuff defined in other modules by namespace;
```
<linkexpr> :=
| link (<aliasname> <-)? <pathexpr>
```

```
link v <- ~/math/vector;

main {
	decl my_vec = new v::vector();
}
```