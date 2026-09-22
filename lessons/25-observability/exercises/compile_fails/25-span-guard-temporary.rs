// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// Entering a span with `Span::enter` hands you a GUARD that borrows the
// span, and the span stays open until the guard drops. So the span itself
// has to outlive the guard — if you create it and enter it in one
// expression, the span is a temporary that dies at the end of that
// statement, while the guard is still holding a borrow of it.
//
// This file reproduces that with plain structs (`Span::enter` here stands in
// for tracing's). rustc reports E0716: "temporary value dropped while
// borrowed".
//
// The fix: give the span a name first, so it lives as long as the guard —
//     let span = span("handle_request");
//     let guard = span.enter();

struct Span {
    name: String,
}

struct Entered<'a> {
    span: &'a Span,
}

impl Span {
    fn enter(&self) -> Entered<'_> {
        Entered { span: self }
    }
}

fn span(name: &str) -> Span {
    Span {
        name: name.to_string(),
    }
}

fn main() {
    let guard = span("handle_request").enter();
    println!("inside the span for {}", guard.span.name);
}
