use std::rc::Rc;

trait Monoid {
    fn zero() -> Self;
    fn append(&self, a: Self) -> Self;
}

impl Monoid for usize {
    fn zero() -> usize { 0 }
    fn append(&self, a: usize) -> usize { self + a }
}

enum FingerTree<A> {
    Empty,
    Single(A),
    Deep(Digit<A>, Rc<FingerTree<Node<A>>>, Digit<A>) // TODO: the middle subtree should be lazily evaluated
}

use FingerTree::{Empty, Single, Deep};

trait Foldable<A> {
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

enum Digit<A> {
    One(A),
    Two(A, A),
    Three(A, A, A),
    Four(A, A, A, A)
}

use Digit::{One, Two, Three, Four};

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

enum Node<A> {
    Node2(A, A),
    Node3(A, A, A)
}

use Node::{Node2, Node3};

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
