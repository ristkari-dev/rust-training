# References & borrowing

> Lesson 07 taught moves. Today: the alternative. You can let a function look at your value — or even modify it — without giving up ownership. The compiler enforces some rules to keep this safe.

---

## Recap

Lesson 07: assigning or passing a `String` **moves** it. The original binding is dead.

Today: the alternative. **Borrow** it. You hand the function a reference to the value and keep ownership yourself.

References come in two flavors.

---

## `&T` — the shared reference

```rust
let s = String::from("hello");
let r: &String = &s;

println!("len = {}", r.len());
println!("{s}");        // s is still ours
```

- `&T` is a **read-only** borrow
- Multiple shared references can coexist
- The original owner stays valid

---

## Functions taking `&T`

```rust
fn show(s: &str) {
    println!("{s}");
}

let owned = String::from("hi");
show(&owned);
show(&owned);     // ...as many times as you like
```

No `.clone()` needed. The function reads through the reference; the caller keeps ownership.

This is why most real Rust functions take `&T`.

---

## `&mut T` — the mutable reference

```rust
fn shout(s: &mut String) {
    s.push('!');
}

let mut owned = String::from("hi");
shout(&mut owned);
println!("{owned}");   // "hi!"
```

- `&mut T` is a borrow **with permission to modify**
- The caller's binding must be `mut` to hand out a mutable reference

---

## The borrowing rules

At any moment for any value:

- **either** zero or more shared references (`&T`)
- **or** exactly one mutable reference (`&mut T`)
- **never both at once**

The borrow checker enforces this at compile time. The compile-fail exercise drills the canonical violation.

```rust
let mut s = String::from("hi");
let r1 = &s;
let r2 = &mut s;   // ERROR: shared borrow still live
println!("{r1}");
```

---

## `&self` / `&mut self` revisited

The receiver kinds from Lesson 06 are just `&T` / `&mut T` where `T` is the type:

```rust
impl Rectangle {
    fn area(&self) -> u32 {        // borrows shared
        self.width * self.height
    }

    fn double_width(&mut self) {   // borrows mutable
        self.width *= 2;
    }
}
```

- `&self` — read the fields
- `&mut self` — modify the fields
- `self` — consume (move, Lesson 07)

Three modes, same machinery.

---

## Borrows vs moves — when to use which

| Situation | Use |
|---|---|
| Function only reads the value | `fn foo(x: &T)` |
| Function modifies in place | `fn foo(x: &mut T)` |
| Function consumes/transforms | `fn foo(x: T) -> T` |
| Need a copy to hand around | `clone()` then pass owned |

In real Rust, most functions take `&T` or `&mut T`. Ownership transfer is reserved for transformation or new storage.

---

## Putting it together

Today's exercises:

- **Warm-up** `wrap_in_quotes(s: &str) -> String` — borrow to read, return new owned data
- **Main** `merge_into(target: &mut String, parts: &[&str], separator: &str)` — one mut, two shared, all live together

Every reference has a *lifetime* — for today, elision handles all our cases.

**Lesson 09** is when we annotate them explicitly.

---

## Wrap — Phase 2 progress

- **`&T`** reads; **`&mut T`** modifies; both leave the original owner intact
- Many shared OR one mutable, never both
- **`&self`**/**`&mut self`** are these receiver kinds applied to methods
- Choose `&T` / `&mut T` / `T` based on what the function does
- Ownership transfer is rare in real Rust code

Next: **Lesson 09 — Lifetimes**.
