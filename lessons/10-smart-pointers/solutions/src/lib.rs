//! Lesson 10 — reference solutions.

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

pub enum Tree {
    Leaf,
    Node(i32, Box<Tree>, Box<Tree>),
}

#[must_use]
pub fn sum(list: &List) -> i32 {
    match list {
        List::Cons(value, rest) => value + sum(rest),
        List::Nil => 0,
    }
}

#[must_use]
pub fn count_nodes(tree: &Tree) -> usize {
    match tree {
        Tree::Node(_, left, right) => 1 + count_nodes(left) + count_nodes(right),
        Tree::Leaf => 0,
    }
}
