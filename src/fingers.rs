use std::iter::{FromIterator, Iterator};
use std::rc::Rc;

//
// --- Traits
//

pub trait Monoid {
    fn zero() -> Self;
    fn append(&self, a: Self) -> Self;
}

pub trait Foldable<A> {
    fn foldl<B, F>(&self, z: B, f: F) -> B where F: Fn(B, &A) -> B;
    fn foldr<B, F>(&self, z: B, f: F) -> B where F: Fn(&A, B) -> B;
}

//
// --- Data Types
//

// TODO: Overall think about what king of subtree sharing the current usage of Rc permits
//       Should all the values of Digit use Rc? Should the Single and Digit's in Deep use Rc?
//       example: push_front -> Deep -> Four

// TODO: the middle subtree in Deep should be lazily evaluated
#[derive(Debug, Clone)]
pub enum FingerTree<A> {
    Empty,
    Single(Node<A>),
    Deep(Digit<A>, Rc<FingerTree<A>>, Digit<A>)
}

#[derive(Debug, Clone)]
pub enum Node<A> {
    Leaf(A),
    Node2(Rc<Node<A>>, Rc<Node<A>>),
    Node3(Rc<Node<A>>, Rc<Node<A>>, Rc<Node<A>>)
}

// TODO: test wether Vec<A> is faster
#[derive(Debug, Clone)]
pub enum Digit<A> {
    One(Node<A>),
    Two(Node<A>, Node<A>),
    Three(Node<A>, Node<A>, Node<A>),
    Four(Node<A>, Node<A>, Node<A>, Node<A>)
}

use fingers::FingerTree::*;
use fingers::Node::*;
use fingers::Digit::*;

//
// --- Macros
//

#[macro_export]
macro_rules! finger_tree {
    ($( $v: expr ),*) => {{
         let tree = FingerTree::new();
         tree $( .push_back($v) )*
    }}
}

//
// --- Implementations
//

impl<A> Node<A> {
    fn front(&self) -> &A {
        match *self {
            Leaf(ref a) => a,
            Node2(ref a, _) => a.front(),
            Node3(ref a, _, _) => a.front(),
        }
    }

    fn back(&self) -> &A {
        match *self {
            Leaf(ref a) => a,
            Node2(_, ref a) => a.back(),
            Node3(_, _, ref a) => a.back(),
        }
    }
}

impl<A> Foldable<A> for Node<A> {
    fn foldl<B, F>(&self, z: B, f: F) -> B where F: Fn(B, &A) -> B {
        match *self {
            Leaf(ref a) => f(z, a),
            Node2(ref a, ref b) => b.foldl(a.foldl(z, &f), &f),
            Node3(ref a, ref b, ref c) => c.foldl(b.foldl(a.foldl(z, &f), &f), &f)
        }
    }

    fn foldr<B, F>(&self, z: B, f: F) -> B where F: Fn(&A, B) -> B {
        match *self {
            Leaf(ref a) => f(a, z),
            Node2(ref a, ref b) => a.foldr(b.foldr(z, &f), &f),
            Node3(ref a, ref b, ref c) => a.foldr(b.foldr(c.foldr(z, &f), &f), &f)
        }
    }
}

impl<A> Digit<A> {
    fn front(&self) -> &A {
        match *self {
            One(ref a) => a.front(),
            Two(ref a, _) => a.front(),
            Three(ref a, _, _) => a.front(),
            Four(ref a, _, _, _) => a.front()
        }
    }

    fn back(&self) -> &A {
        match *self {
            One(ref a) => a.back(),
            Two(_, ref a) => a.back(),
            Three(_, _, ref a) => a.back(),
            Four(_, _, _, ref a) => a.back()
        }
    }
}

impl<A> Foldable<A> for Digit<A> {
    fn foldl<B, F>(&self, z: B, f: F) -> B where F: Fn(B, &A) -> B {
        match *self {
            One(ref a) => a.foldl(z, &f),
            Two(ref a, ref b) => b.foldl(a.foldl(z, &f), &f),
            Three(ref a, ref b, ref c) => c.foldl(b.foldl(a.foldl(z, &f), &f), &f),
            Four(ref a, ref b, ref c, ref d) => d.foldl(c.foldl(b.foldl(a.foldl(z, &f), &f), &f), &f)
        }
    }

    fn foldr<B, F>(&self, z: B, f: F) -> B where F: Fn(&A, B) -> B {
        match *self {
            One(ref a) => a.foldr(z, &f),
            Two(ref a, ref b) => a.foldr(b.foldr(z, &f), &f),
            Three(ref a, ref b, ref c) => a.foldr(b.foldr(c.foldr(z, &f), &f), &f),
            Four(ref a, ref b, ref c, ref d) => a.foldr(b.foldr(c.foldr(d.foldr(z, &f), &f), &f), &f)
        }
    }
}

impl<A: Clone> FingerTree<A> {
    pub fn new() -> FingerTree<A> {
        Empty
    }

    pub fn push_front(&self, a: A) -> FingerTree<A> {
        self.push_front_aux(Leaf(a))
    }

    fn push_front_aux(&self, a: Node<A>) -> FingerTree<A> {
        match *self {
            Empty => Single(a),
            Single(ref b) => Deep(One(a), Rc::new(Empty), One(b.clone())),
            Deep(ref pr, ref m, ref sf) =>
                match *pr {
                    One(ref b) =>
                        Deep(Two(a, b.clone()), m.clone(), sf.clone()),
                    Two(ref b, ref c) =>
                        Deep(Three(a, b.clone(), c.clone()), m.clone(), sf.clone()),
                    Three(ref b, ref c, ref d) =>
                        Deep(Four(a, b.clone(), c.clone(), d.clone()), m.clone(), sf.clone()),
                    Four(ref b, ref c, ref d, ref e) =>
                        Deep(
                            Two(a, b.clone()),
                            Rc::new(m.push_front_aux(Node3(Rc::new(c.clone()), Rc::new(d.clone()), Rc::new(e.clone())))),
                            sf.clone()),
                }

        }
    }

    pub fn push_back(&self, a: A) -> FingerTree<A> {
        self.push_back_aux(Leaf(a))
    }

    fn push_back_aux(&self, a: Node<A>) -> FingerTree<A> {
        match *self {
            Empty => Single(a),
            Single(ref b) => Deep(One(b.clone()), Rc::new(Empty), One(a)),
            Deep(ref pr, ref m, ref sf) =>
                match *sf {
                    One(ref b) =>
                        Deep(pr.clone(), m.clone(), Two(b.clone(), a)),
                    Two(ref c, ref b) =>
                        Deep(pr.clone(), m.clone(), Three(c.clone(), b.clone(), a)),
                    Three(ref d, ref c, ref b) =>
                        Deep(pr.clone(), m.clone(), Four(d.clone(), c.clone(), b.clone(), a)),
                    Four(ref e, ref d, ref c, ref b) =>
                        Deep(
                            pr.clone(),
                            Rc::new(m.push_back_aux(Node3(Rc::new(e.clone()), Rc::new(d.clone()), Rc::new(c.clone())))),
                            Two(b.clone(), a))
                }

        }
    }

    pub fn front(&self) -> Option<&A> {
        match *self {
            Empty => None,
            Single(ref a) => Some(a.front()),
            Deep(ref pr, _, _) => Some(pr.front())
        }
    }

    pub fn back(&self) -> Option<&A> {
        match *self {
            Empty => None,
            Single(ref a) => Some(a.back()),
            Deep(_, _, ref sf) => Some(sf.back())
        }
    }
}

impl<A> Foldable<A> for FingerTree<A> {
    fn foldl<B, F>(&self, z: B, f: F) -> B where F: Fn(B, &A) -> B {
        match *self {
            Empty => z,
            Single(ref a) => a.foldl(z, &f),
            Deep(ref pr, ref m, ref sf) => {
                fn fold<A, B, F>(b: B, d: &Digit<A>, f: &F) -> B where F: Fn(B, &A) -> B {
                    d.foldl(b, f)
                }

                fold(m.foldl(fold(z, pr, &f), |b, a| f(b, a)), sf, &f)
            }
        }
    }

    fn foldr<B, F>(&self, z: B, f: F) -> B where F: Fn(&A, B) -> B {
        match *self {
            Empty => z,
            Single(ref a) => a.foldr(z, &f),
            Deep(ref pr, ref m, ref sf) => {
                fn fold<A, B, F>(b: B, d: &Digit<A>, f: &F) -> B where F: Fn(&A, B) -> B {
                    d.foldr(b, f)
                }

                fold(m.foldr(fold(z, sf, &f), |a, b| f(a, b)), pr, &f)
            }
        }
    }
}

impl<A: Clone> FromIterator<A> for FingerTree<A> {
    fn from_iter<T>(it: T) -> FingerTree<A> where T: IntoIterator<Item=A> {
        it.into_iter().fold(FingerTree::new(), |b, a| b.push_back(a))
    }
}
