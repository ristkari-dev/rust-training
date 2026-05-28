use smart_pointers_solutions::{List, Tree, count_nodes, sum};

// Warm-up: sum

#[test]
fn warmup_nil() {
    assert_eq!(sum(&List::Nil), 0);
}

#[test]
fn warmup_single() {
    assert_eq!(sum(&List::Cons(5, Box::new(List::Nil))), 5);
}

#[test]
fn warmup_three() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    assert_eq!(sum(&list), 6);
}

#[test]
fn warmup_negatives_cancel() {
    let list = List::Cons(-5, Box::new(List::Cons(5, Box::new(List::Nil))));
    assert_eq!(sum(&list), 0);
}

// Main: count_nodes

#[test]
fn main_leaf() {
    assert_eq!(count_nodes(&Tree::Leaf), 0);
}

#[test]
fn main_single_node() {
    let tree = Tree::Node(5, Box::new(Tree::Leaf), Box::new(Tree::Leaf));
    assert_eq!(count_nodes(&tree), 1);
}

#[test]
fn main_three_nodes() {
    let tree = Tree::Node(
        1,
        Box::new(Tree::Node(2, Box::new(Tree::Leaf), Box::new(Tree::Leaf))),
        Box::new(Tree::Node(3, Box::new(Tree::Leaf), Box::new(Tree::Leaf))),
    );
    assert_eq!(count_nodes(&tree), 3);
}

#[test]
fn main_lopsided() {
    let tree = Tree::Node(
        1,
        Box::new(Tree::Node(
            2,
            Box::new(Tree::Node(3, Box::new(Tree::Leaf), Box::new(Tree::Leaf))),
            Box::new(Tree::Leaf),
        )),
        Box::new(Tree::Leaf),
    );
    assert_eq!(count_nodes(&tree), 3);
}
