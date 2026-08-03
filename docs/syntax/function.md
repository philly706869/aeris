# Function

```aeris
fn foo {

}
```

```aeris
fn foo() {

}
```

```aeris
fn foo() =<> {

}
```

```aeris
fn add(a: I32, b: I32) = I32 {
  print("foo executed");
  a + b
}

fn main {
  let num = add(10, 20);
  println("{num}");
}
```
