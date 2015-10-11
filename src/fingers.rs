use std::iter::FromIterator;
use std::rc::Rc;

pub trait Monoid {
    fn zero() -> Self;
    fn append(&self, a: Self) -> Self;
}

impl Monoid for usize {
    fn zero() -> usize { 0 }
    fn append(&self, a: usize) -> usize { self + a }
}

pub enum FingerTree<A> {
    Empty,
    Single(A),
    Deep(Rc<Digit<A>>, Rc<FingerTree<Node<A>>>, Rc<Digit<A>>) // TODO: the middle subtree should be lazily evaluated
}

use fingers::FingerTree::{Empty, Single, Deep};

pub trait Foldable<A> {
    fn foldl<B, F>(&self, z: B, f: F) -> B where F: Fn(B, &A) -> B;
    fn foldr<B, F>(&self, z: B, f: F) -> B where F: Fn(&A, B) -> B;
}

impl<A> Foldable<A> for FingerTree<A> {
    fn foldl<B, F>(&self, z: B, f: F) -> B where F: Fn(B, &A) -> B {
        match *self {
            Empty => z,
            Single(ref a) => f(z, a),
            Deep(ref pr, ref m, ref sf) => {
                fn f1<A, B, F>(b: B, d: &Digit<A>, f: &F) -> B where F: Fn(B, &A) -> B {
                    d.foldl(b, f)
                }

                f1(m.foldl(f1(z, pr, &f), |b, a| a.foldl(b, &f)), sf, &f)
            }
        }
    }

    fn foldr<B, F>(&self, z: B, f: F) -> B where F: Fn(&A, B) -> B {
        match *self {
            Empty => z,
            Single(ref a) => f(a, z),
            Deep(ref pr, ref m, ref sf) => {
                fn f1<A, B, F>(b: B, d: &Digit<A>, f: &F) -> B where F: Fn(&A, B) -> B {
                    d.foldr(b, f)
                }

                f1(m.foldr(f1(z, pr, &f), |a, b| a.foldr(b, &f)), sf, &f)
            }
        }
    }
}


impl<A: Clone> FromIterator<A> for FingerTree<A> {
    fn from_iter<T>(it: T) -> FingerTree<A> where T: IntoIterator<Item=A> {
        it.into_iter().fold(Empty, |b, a| b.push_back(a))
    }
}

impl<A: Clone> FingerTree<A> {
    pub fn push_front(&self, a: A) -> FingerTree<A> {
        match *self {
            Empty => Single(a),
            Single(ref b) => Deep(Rc::new(One(a)), Rc::new(Empty), Rc::new(One(b.clone()))),
            Deep(ref pr, ref m, ref sf) =>
                match **pr {
                    One(ref b) =>
                        Deep(Rc::new(Two(a, b.clone())), m.clone(), sf.clone()),
                    Two(ref b, ref c) =>
                        Deep(Rc::new(Three(a, b.clone(), c.clone())), m.clone(), sf.clone()),
                    Three(ref b, ref c, ref d) =>
                        Deep(Rc::new(Four(a, b.clone(), c.clone(), d.clone())), m.clone(), sf.clone()),
                    Four(ref b, ref c, ref d, ref e) =>
                        Deep(Rc::new(Two(a, b.clone())), Rc::new(m.push_front(Node3(c.clone(), d.clone(), e.clone()))), sf.clone()),
                }
        }
    }

    pub fn push_back(&self, a: A) -> FingerTree<A> {
        match *self {
            Empty => Single(a),
            Single(ref b) => Deep(Rc::new(One(b.clone())), Rc::new(Empty), Rc::new(One(a))),
            Deep(ref pr, ref m, ref sf) =>
                match **sf {
                    One(ref b) =>
                        Deep(pr.clone(), m.clone(), Rc::new(Two(b.clone(), a))),
                    Two(ref c, ref b) =>
                        Deep(pr.clone(), m.clone(), Rc::new(Three(c.clone(), b.clone(), a))),
                    Three(ref d, ref c, ref b) =>
                        Deep(pr.clone(), m.clone(), Rc::new(Four(d.clone(), c.clone(), b.clone(), a))),
                    Four(ref e, ref d, ref c, ref b) =>
                        Deep(pr.clone(), Rc::new(m.push_back(Node3(e.clone(), d.clone(), c.clone()))), Rc::new(Two(b.clone(), a))),
                }
        }
    }
}

pub enum Digit<A> {
    One(A),
    Two(A, A),
    Three(A, A, A),
    Four(A, A, A, A)
}

use fingers::Digit::{One, Two, Three, Four};

impl<A> Foldable<A> for Digit<A> {
    fn foldl<B, F>(&self, z: B, f: F) -> B where F: Fn(B, &A) -> B {
        match *self {
            One(ref a) => f(z, a),
            Two(ref a, ref b) => f(f(z, a), b),
            Three(ref a, ref b, ref c) => f(f(f(z, a), b), c),
            Four(ref a, ref b, ref c, ref d) => f(f(f(f(z, a), b), c), d)
        }
    }

    fn foldr<B, F>(&self, z: B, f: F) -> B where F: Fn(&A, B) -> B {
        match *self {
            One(ref a) => f(a, z),
            Two(ref a, ref b) => f(a, f(b, z)),
            Three(ref a, ref b, ref c) => f(a, f(b, f(c, z))),
            Four(ref a, ref b, ref c, ref d) => f(a, f(b, f(c, f(d, z))))
        }
    }
}

#[derive(Clone)]
pub enum Node<A> {
    Node2(A, A),
    Node3(A, A, A)
}

use fingers::Node::{Node2, Node3};

impl<A> Foldable<A> for Node<A> {
    fn foldl<B, F>(&self, z: B, f: F) -> B where F: Fn(B, &A) -> B {
        match *self {
            Node2(ref a, ref b) => f(f(z, a), b),
            Node3(ref a, ref b, ref c) => f(f(f(z, a), b), c)
        }
    }

    fn foldr<B, F>(&self, z: B, f: F) -> B where F: Fn(&A, B) -> B {
        match *self {
            Node2(ref a, ref b) => f(a, f(b, z)),
            Node3(ref a, ref b, ref c) => f(a, f(b, f(c, z)))
        }
    }
}

fn main() {
    let n_vec = vec![0, 1, 2, 3, 4];
    // let n_tree = FingerTree::<usize>::from_iter(n_vec);
    // let n_tree: FingerTree:<usize> = Empty;
}
