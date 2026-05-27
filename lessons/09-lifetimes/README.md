# Lesson 09 — Lifetimes

Every reference has a lifetime — a scope during which the reference
is valid. Most of the time, Rust figures lifetimes out for you. This
lesson is about the syntax for the cases where it can't, and why.

## Learning goals

- Explain that every reference has a lifetime — a scope during which
  it's valid — and that the compiler tracks it
- Identify when lifetime elision applies (single input reference,
  methods with `&self`) and when it doesn't
- Write a function signature with explicit `<'a>` annotation when
  elision fails
- Define a struct that holds a reference, declaring and using the
  lifetime parameter consistently across struct, `impl`, and methods
- Recognize `&'static str` as a reference that lives for the entire
  program

## Self-study notes

### Every reference has a lifetime

A **lifetime** is the scope during which a reference is valid. The
compiler tracks it for every reference. Most of the time you never
see it — Rust infers it through *lifetime elision*. When you do see
it, like `&'static str` for string literals, the syntax is just
surfacing what was already there.

### Elision handles the common cases

For most function signatures, Rust deduces the lifetime relationships
automatically:

```rust
fn first_char(s: &str) -> &str { /* ... */ }
fn area(rect: &Rectangle) -> u32 { /* ... */ }
```

You've been writing code like this since Lesson 04 without thinking
about lifetimes. The rule, informally: when there's exactly one
input reference, the output reference (if any) borrows from it.

### When elision fails, and the `<'a>` syntax

Two reference parameters, one returned reference — the compiler
can't guess which input the output borrows from:

```rust
fn longest(a: &str, b: &str) -> &str {     // won't compile
    if a.len() >= b.len() { a } else { b }
}
```

rustc says: *"this function's return type contains a borrowed value,
but the signature does not say whether it is borrowed from `a` or
`b`."*

You spell out the relationship with `<'a>`:

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

Read this as: "for some lifetime `'a`, both inputs and the output
all share that lifetime." The compiler picks `'a` at each call site
to be the shorter of the two input lifetimes — that's the lifetime
of the returned reference.

### Lifetimes in struct fields

Any struct that holds a reference must declare a lifetime parameter:

```rust
struct Excerpt<'a> {
    text: &'a str,
}
```

This says: "the struct cannot outlive the `&str` it holds." If the
original string is dropped, any `Excerpt` referencing it is
invalidated by the compiler.

The `impl` block also carries `<'a>`:

```rust
impl<'a> Excerpt<'a> {
    pub fn new(text: &'a str) -> Self {
        Excerpt { text }
    }

    pub fn length(&self) -> usize {
        self.text.len()
    }
}
```

Within methods that take `&self`, you don't need to annotate further
— the receiver's lifetime is implicit and elision handles it.

### `'static` — the special lifetime

`&'static str` is a reference that lives for the entire program.
String literals are `&'static str` (slices into the compiled binary):

```rust
let greeting: &'static str = "hello";
```

Most other references have shorter, scope-limited lifetimes. You'll
occasionally need `'static` in trait bounds and APIs (it'll come up
later in the course), but it's not common in everyday function
signatures.

## Exercises

### Warm-up: `longest`

Implement `longest<'a>(a: &'a str, b: &'a str) -> &'a str` in
`exercises/src/lib.rs`. Return whichever of `a` and `b` has the
greater length (or `a` on a tie).

The signature is already annotated for you — your job is to fill in
the body. The `<'a>` says "the returned reference borrows from one
of `a` or `b`, both of which share lifetime `'a`."

### Main: `Excerpt<'a>`

The exercises crate ships an `Excerpt<'a>` struct holding a `&'a
str` field, plus an `impl` block with three stub methods:

- `Excerpt::new(text: &'a str) -> Self` — constructor
- `excerpt.length()` — returns the length of the held text
- `excerpt.is_empty()` — returns whether the held text is empty

Fill in the three `todo!()` bodies. The `<'a>` plumbing is already
done — the lesson is to see how it threads through the struct, the
impl block, and the constructor.

### Compile-fail

`exercises/compile_fails/09-missing-lifetime.rs` ships with a struct
holding a `&str` field but no lifetime parameter. rustc's error
message *includes the fix* — it'll suggest adding `<'a>` to the
struct and `&'a str` to the field. Read the error carefully; the
compiler is your friend here.

### Run

```bash
make verify LESSON=09-lifetimes
```

This runs your exercise tests and asserts the compile-fail file now
compiles.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try
the exercises before peeking.
