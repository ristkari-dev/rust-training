# Compound types

> Rust has two ways to talk about a sequence of values: own them, or look at them through a slice. Most of the standard library is built on this distinction.

---

## Recap

Lessons 01-03 covered single values, mutation, and control flow.

Today: group values into tuples and arrays, then meet the **slice** — the abstraction that makes those groups usable across function boundaries.

---

## Tuples

```rust
let pair: (i32, &str) = (42, "hi");

let n = pair.0;     // 42
let s = pair.1;     // "hi"

let (n, s) = pair;  // destructure
```

- Mix types in a single value
- Access by position with `.0`, `.1`, etc.
- Destructure with `let (a, b) = ...`

---

## Arrays

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first = arr[0];      // 1
let len = arr.len();     // 5
```

- Fixed size, same element type
- Length is part of the type: `[i32; 5]` and `[i32; 6]` are different
- Index with `arr[i]`

---

## Slices

`&[T]` is a **borrowed view** into a sequence:

```rust
let arr = [1, 2, 3, 4, 5];
let middle: &[i32] = &arr[1..4];   // [2, 3, 4]

fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}
```

A function taking `&[i32]` works for **any size** of array. This is huge — it decouples your function from a specific length.

---

## `&str` is a slice too

Where `&[u8]` is a view of raw bytes, `&str` is a view of bytes **guaranteed to be valid UTF-8**.

```rust
let greeting: &str = "hello";        // a slice into the binary
let slice: &str = &greeting[0..3];   // "hel"
```

String literals like `"hello"` are `&'static str` — slices pointing into the program's compiled binary itself.

---

## `String` — owned, heap-allocated

```rust
let mut s = String::new();
s.push_str("hello");
s.push(',');
s.push_str(" world");
// s == "hello, world"

let from_literal = String::from("hi");
let from_method  = "hi".to_string();
```

- Heap-allocated and growable
- Owned (we'll see what "owned" really means in Lesson 07)
- Most string data you build at runtime is a `String`

---

## `String` vs `&str` — when to use which

Rule of thumb:

- **Take `&str` as a parameter** — works for both `String` (via `&s`) and string literals
- **Return `String`** — when you build new data

Conversions:

```rust
let s: String = "hello".to_string();
let s: String = String::from("hello");

let owned: String = String::from("hello");
let view: &str = &owned;        // auto-deref from &String to &str
```

---

## Putting it together

```rust
fn join_with_dashes(words: &[&str]) -> String {
    let mut out = String::new();
    let mut first = true;
    for w in words {
        if !first {
            out.push('-');
        }
        out.push_str(w);
        first = false;
    }
    out
}
```

- `&[&str]` — slice of borrowed string slices
- `String::new()` + `push_str` + `push` to build up
- Tail returns the owned `String`

---

## Wrap

- **Tuples** mix types: `(T, U, ...)` with `.0`/`.1` access and destructuring
- **Arrays** are fixed-size and same-type: `[T; N]`
- **Slices** `&[T]` are borrowed views that decouple functions from sizes
- **`&str`** is a slice; **`String`** is owned and heap-allocated
- The canonical signature pattern: *take `&str`, return `String`*

Next: Lesson 05 — pattern matching & enums.
