#[macro_use]
extern crate fingers;

use fingers::{FingerTree, Foldable};
use std::iter::FromIterator;

fn main() {
    let v = vec![0, 1, 2, 3, 4];
    let t = FingerTree::from_iter(v);
    println!("{:?}", t);

    let t2 = finger_tree![0, 1, 2, 3, 4];
    println!("{:?}", t2);

    println!("front: {:?}", t.front());
    println!("back: {:?}", t.back());

    println!("{:?}", t.push_front(0).push_back(6));

    let sum = t2.foldl(0, &|a, b| a + b);
    println!("{:?}", sum);
}
