use trait_objects_solutions::{Book, Coffee, Priced, describe_price, total_price_dyn};

// Warm-up: describe_price (&dyn Priced)

#[test]
fn warmup_describe_book() {
    assert_eq!(describe_price(&Book { cents: 500 }), "500 cents");
}

#[test]
fn warmup_describe_free() {
    assert_eq!(describe_price(&Book { cents: 0 }), "free");
}

#[test]
fn warmup_describe_coffee() {
    assert_eq!(describe_price(&Coffee { shots: 2 }), "300 cents");
}

#[test]
fn warmup_describe_via_dyn_ref() {
    let item: &dyn Priced = &Coffee { shots: 1 };
    assert_eq!(describe_price(item), "250 cents");
}

// Main: total_price_dyn (&[Box<dyn Priced>])

#[test]
fn main_total_empty() {
    let items: Vec<Box<dyn Priced>> = Vec::new();
    assert_eq!(total_price_dyn(&items), 0);
}

#[test]
fn main_total_mixed() {
    let items: Vec<Box<dyn Priced>> = vec![
        Box::new(Book { cents: 100 }),
        Box::new(Coffee { shots: 2 }),
        Box::new(Book { cents: 50 }),
    ];
    assert_eq!(total_price_dyn(&items), 450);
}

#[test]
fn main_total_all_coffee() {
    let items: Vec<Box<dyn Priced>> =
        vec![Box::new(Coffee { shots: 1 }), Box::new(Coffee { shots: 3 })];
    assert_eq!(total_price_dyn(&items), 600);
}

#[test]
fn main_total_single() {
    let items: Vec<Box<dyn Priced>> = vec![Box::new(Book { cents: 999 })];
    assert_eq!(total_price_dyn(&items), 999);
}
