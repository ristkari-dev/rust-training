# Lesson 08 — References & borrowing

Lesson 07 taught moves: assigning or passing a `String` moves it, and
the original binding is dead. Today is the alternative. You can let a
function look at a value — or even modify it — without giving up
ownership. References make this possible, and the compiler enforces
rules to keep it safe.

## Learning goals

- Explain what `&T` and `&mut T` are and write functions that take
  them as parameters
- State the borrowing rules — at any moment for any value, either
  many shared references or one mutable reference, never both
- Choose between `T`, `&T`, `&mut T`, and `clone()` when designing a
  function signature
- Read rustc's borrow-checker errors and resolve them by reordering
  or dropping borrows
- Connect `&self` / `&mut self` from Lesson 06 to the reference
  syntax — they're just `&T` and `&mut T` where `T` is the receiver
  type

## Self-study notes

### `&T` — the shared reference

A reference is a pointer-like value that **borrows** another value
without taking ownership. The simplest kind is `&T`, the shared
(read-only) reference:

```rust
let s = String::from("hello");
let r: &String = &s;
println!("len = {}", r.len());
println!("{s}");          // s is still ours — borrowed, not moved
```

Multiple shared references to the same value can coexist. The
original owner stays valid throughout.

Most real Rust functions take `&T` (or `&str` for strings) so the
caller keeps ownership:

```rust
fn show(s: &str) {
    println!("{s}");
}

let owned = String::from("hi");
show(&owned);
show(&owned);             // call as many times as you like
println!("{owned}");      // still ours
```

### `&mut T` — the mutable reference

`&mut T` is a reference with permission to **modify** the borrowed
value:

```rust
fn shout(s: &mut String) {
    s.push('!');
}

let mut owned = String::from("hi");
shout(&mut owned);
println!("{owned}");      // "hi!"
```

To hand out a mutable reference, the caller's binding must be `mut`.
The function still doesn't own the value — it just borrows with
write access for the duration of the call.

### The borrowing rules

At any moment, for any value:

- **either** zero or more shared references (`&T`),
- **or** exactly one mutable reference (`&mut T`),
- **never both at once.**

The borrow checker enforces these rules at compile time. Code that
violates them fails to compile:

```rust
let mut s = String::from("hi");
let r1 = &s;
let r2 = &mut s;   // ERROR: can't take mut while shared borrow is live
println!("{r1}");
```

Borrows end at their **last use**, not at end-of-scope. If you
reorder so `r1` is used before `&mut s` is taken, the code compiles
— the shared borrow has ended by the time the mutable borrow begins.

### `&self` and `&mut self` revisited

The receiver kinds you learned in Lesson 06 are now demystified —
they're just `&T` and `&mut T` where `T` is the type the method
belongs to:

```rust
impl Rectangle {
    fn area(&self) -> u32 {        // borrows self shared
        self.width * self.height
    }

    fn double_width(&mut self) {   // borrows self mutable
        self.width *= 2;
    }
}
```

- `&self` is `&Rectangle` — a shared borrow
- `&mut self` is `&mut Rectangle` — a mutable borrow
- bare `self` is `Rectangle` — a move (Lesson 07)

Three modes, same machinery.

### Borrows vs moves — when to use which

| Situation | Use |
|---|---|
| Function only reads the value | `fn foo(x: &T)` |
| Function modifies the value in place | `fn foo(x: &mut T)` |
| Function consumes / transforms | `fn foo(x: T) -> T` |
| Need a copy to hand around | `clone()` then pass owned |

In real Rust, most functions take `&T` or `&mut T`. Ownership-taking
is reserved for genuinely transforming the value or storing it
somewhere new.

## Exercises

### Warm-up: `wrap_in_quotes`

Implement `wrap_in_quotes(s: &str) -> String` in
`exercises/src/lib.rs`. The function takes a shared reference and
returns a new owned `String` that wraps `s` in double quotes —
`wrap_in_quotes("hello")` returns `"\"hello\""`.

Use `String::from("\"")` to start the result, then `push_str(s)` and
`push('"')` to build it up.

### Main: `merge_into`

Implement `merge_into(target: &mut String, parts: &[&str], separator: &str)`
so it appends each part to `target` with `separator` between
consecutive parts. If `parts` is empty, leave `target` unchanged.

The signature uses one mutable reference (`target`) and two shared
references (`parts`, `separator`) at the same time. The "many shared
OR one mutable" rule applies per value — different values each have
their own borrows.

Reference solution uses the `first: bool` flag pattern:

```rust
let mut first = true;
for part in parts {
    if !first {
        target.push_str(separator);
    }
    target.push_str(part);
    first = false;
}
```

### Compile-fail

`exercises/compile_fails/08-mut-and-shared.rs` ships with code that
takes a `&mut s` while a shared `&s` is still in use. rustc's error
names both reference kinds. The fix is to reorder so the shared
borrow's last use happens before the mutable borrow begins.

### Run

```bash
make verify LESSON=08-references
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
