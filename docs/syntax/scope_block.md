# Scope Block

```aeris
let a = 10;
(
  let b = 20;
  println("{a}"); // ok
)
println("{b}"); // compile error
```

```aeris
let a = (
	let b = 20;
	if b > 10 {
    break -1;
  };
	b
);
```
