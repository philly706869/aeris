# Lambda

Undecided

## Concept 1

```aeris
{
  println("lambda");
}
```

```aeris
|number: U32| {
  println("number is {number}");
}
```

```aeris
fn receive_lambda(lambda: Fn<<>, U32>) {
  let num = labmda();
  println("{num}");
}

fn main {
  receive_lambda {
    println("lambda executed");
    10
  }
}
```

```aeris
fn receive_lambda(lambda: Fn<<U32, U32>, U32>) {
  let num = labmda(10, 20);
  println("{num}");
}

fn main {
  receive_lambda |a, b| {
    a + b
  }
}
```
