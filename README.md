# finge.rs

> An implementation of finger trees in Rust.

## Example

```rust
let v = vec![1, 2, 3, 4, 5];
let t = FingerTree::from_iter(v);

println!("front: {:?}", t.front());
println!("back: {:?}", t.back());

println!("{:?}", t.push_front(0).push_back(6))

println!("{:?}", t.foldl(0, &|a, b| a + b));
```

Based on [*Finger trees: a simple general-purpose data structure, Ralf Hinze and Ross Paterson. J. Funct. Program. 16(2):197-217 (2006)*](http://staff.city.ac.uk/~ross/papers/FingerTree.pdf)
