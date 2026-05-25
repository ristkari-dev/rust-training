# Lesson 04 — Compound types

Rust has two ways to talk about a sequence of values: own them, or look
at them through a slice. Once you see this distinction, large swathes of
the standard library — including the `String`/`&str` split that famously
trips up beginners — fall into place.

## Learning goals

- Construct, destructure, and index tuples
- Read and use fixed-size arrays
- Take a slice `&[T]` as a parameter so functions work for any-length
  inputs
- Distinguish `String` (owned, heap) from `&str` (borrowed view) and
  choose the right one for a given API
- Convert between `&str` and `String`, and recognize the canonical
  signature pattern *take `&str`, return `String`*

## Self-study notes

### Tuples

A tuple groups a fixed number of values, possibly of different types:

```rust
let pair: (i32, &str) = (42, "hi");
let n = pair.0;     // access by position
let s = pair.1;
let (a, b) = pair;  // or destructure into named bindings
```

The tuple's type lists each field's type in order. Common shapes: `()`
(the empty tuple, also called the unit type), `(T,)` (a one-element
tuple — note the trailing comma), `(T, U)`, `(T, U, V)`, and so on.

### Arrays

An array is a fixed-size, same-type sequence:

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];
let first = arr[0];     // index access
let len = arr.len();    // 5
```

The length is **part of the type**: `[i32; 5]` and `[i32; 6]` are
different types. You generally don't pass arrays around by value (the
size would force every caller to use the same length); you pass slices.

### Slices

A slice `&[T]` is a **borrowed view** into a sequence: a pointer + length,
but no ownership of the data. You can create one with the range syntax:

```rust
let arr = [10, 20, 30, 40, 50];
let middle: &[i32] = &arr[1..4];   // [20, 30, 40]
```

The big payoff is in function signatures:

```rust
fn sum(numbers: &[i32]) -> i32 {
    let mut total = 0;
    for n in numbers {
        total += n;
    }
    total
}
```

`sum` works for any size of array, any sub-slice of an array, and (later)
any `Vec`. One function, many callers.

### `String` vs `&str`

`&str` is a slice — specifically, a slice of bytes guaranteed to be valid
UTF-8. String literals like `"hello"` are `&'static str` (slices into
the program binary).

`String` is the owned, heap-allocated, growable counterpart:

```rust
let mut s = String::new();
s.push_str("hello");
s.push(',');
s.push_str(" world");
// s == "hello, world"
```

`String` is what you build at runtime. `&str` is what you pass around
when you just need to look at a string without owning it.

### Conversions and signature patterns

Going between them is direct:

```rust
let owned: String = "hi".to_string();
let owned: String = String::from("hi");

let owned = String::from("hello");
let view: &str = &owned;        // &String auto-derefs to &str
```

The idiomatic signature pattern is **take `&str`, return `String`**:

```rust
fn greet(name: &str) -> String {
    let mut out = String::from("Hello, ");
    out.push_str(name);
    out.push('!');
    out
}
```

Taking `&str` lets the caller pass either a literal or a borrowed
`String`. Returning `String` makes it clear the function owns the result
and can return new data.

## Exercises

### Warm-up: `divmod`

Implement `divmod(a: u32, b: u32) -> (u32, u32)` in `exercises/src/lib.rs`
returning `(quotient, remainder)`. The function body should be a single
tail expression — no `let`, no `return`, just build and return the tuple.

### Main: `join_with_dashes`

Implement `join_with_dashes(words: &[&str]) -> String` so it joins all
the words with a `'-'` between them and returns the result as an owned
`String`. An empty input returns `""`.

You'll likely want:

- `String::new()` to start with an empty owned string
- A `let mut first = true;` flag so you only insert `'-'` between items
- A `for w in words` loop
- `out.push('-')` for the separator and `out.push_str(w)` for each word
- A tail expression that returns `out`

### Compile-fail

`exercises/compile_fails/04-string-vs-str.rs` ships in a state that
does **not** compile. The function declares it returns `String` but
binds a `&str` literal to a `String`-typed variable. Read the rustc
error, then convert the literal with `.to_string()` or `String::from(...)`.

### Run

```bash
make verify LESSON=04-compound-types
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
