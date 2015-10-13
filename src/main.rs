extern crate fingers;

use fingers::fingers::FingerTree; // TODO: this is dumb :-(
use std::iter::FromIterator;

fn main() {
    let n_vec = vec![0, 1, 2, 3, 4];
    let n_tree = FingerTree::<usize>::from_iter(n_vec);
    println!("{:?}", n_tree);
}
