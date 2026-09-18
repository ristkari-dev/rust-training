// Compile-fail exercise: this file MUST NOT compile until you fix it.
//
// A transaction ends exactly once. That is why sqlx's `commit` and
// `rollback` take `self` BY VALUE: committing consumes the transaction,
// and the compiler makes sure you cannot use it afterwards.
//
// This file reproduces that with a plain struct (`Transaction::commit`
// here stands in for sqlx's). The code below commits, then runs one more
// statement, so rustc reports E0382: "borrow of moved value: `tx`", with
// the note "`Transaction::commit` takes ownership of the receiver
// `self`, which moves `tx`".
//
// The fix: finish the work BEFORE committing — move the second
// `tx.execute(...)` call above `tx.commit()`.

struct Transaction {
    statements: Vec<String>,
}

impl Transaction {
    fn execute(&mut self, sql: &str) {
        self.statements.push(sql.to_string());
    }

    // `commit` takes `self` BY VALUE: committing consumes the transaction.
    fn commit(self) -> usize {
        self.statements.len()
    }
}

fn main() {
    let mut tx = Transaction {
        statements: Vec::new(),
    };
    tx.execute("UPDATE accounts SET balance = balance - 10 WHERE id = 1");
    let n = tx.commit();
    tx.execute("UPDATE accounts SET balance = balance + 10 WHERE id = 2");
    println!("{n}");
}
