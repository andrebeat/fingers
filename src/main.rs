#[macro_use]
extern crate fingers;

use fingers::fingers::FingerTree; // TODO: this is dumb :-(
use std::iter::FromIterator;

fn main() {
    let n_vec = vec![0, 1, 2, 3, 4];
    let n_tree = FingerTree::<usize>::from_iter(n_vec);
    println!("{:?}", n_tree);

    let n_tree2 = finger_tree![0, 1, 2, 3, 4];
    println!("{:?}", n_tree2);

    println!("front: {:?}", n_tree.front());
    println!("back: {:?}", n_tree.back());
}
