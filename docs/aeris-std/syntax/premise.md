# Premise

```aeris
fn print_first(arr: Vec<U32>) {
  premise arr.len() > 0;
  let first = arr[0];
  println("first item is {first}");
}

fn main {
  print_first([1, 2, 3]); // ok
  print_first([]); // compile error
}
```
