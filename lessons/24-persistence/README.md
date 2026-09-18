# Lesson 24 — Persistence

A service that forgets everything when it restarts isn't a service.
`sqlx` lets you talk to a database in plain SQL, asynchronously, with
Rust types on both ends. The production skill: a **transaction** is
all-or-nothing, and you get that guarantee by routing every statement
through the transaction handle. This lesson runs entirely against SQLite
in memory — no database to install and nothing to start. Two one-time
costs before your first `make verify`, though: the lesson raises the
pinned toolchain to 1.98 (rustup downloads it), and `sqlx` builds SQLite
from C source, which needs a C compiler.

## Learning goals

- Open a connection pool with `SqlitePool` / `SqlitePoolOptions`, and
  know that a pool is the cheap-to-clone handle you put in an axum
  `State`
- Run schema migrations from an ordered `migrations/` directory with
  `sqlx::migrate!()`, and know that migrations are append-only and
  recorded in a `_sqlx_migrations` table
- Send a query with `sqlx::query(...).bind(...)` — never
  string-formatted SQL — and run it with `.execute()` / `.fetch_one()` /
  `.fetch_all()`, propagating `sqlx::Error` with `?`
- Map result rows onto a struct with `#[derive(sqlx::FromRow)]` and
  `query_as`, and know that a SQLite `INTEGER` column is an `i64`
- Group statements into one atomic unit with `pool.begin()`, route each
  through `&mut *tx`, and `commit()` or `rollback()` — both of which
  consume the transaction

## Self-study notes

### The pool and the database

A pool opens database connections and hands them out as queries need
them:

```rust
use sqlx::sqlite::SqlitePoolOptions;
use std::time::Duration;

let pool = SqlitePoolOptions::new()
    .max_connections(1)
    .acquire_timeout(Duration::from_secs(2))
    .connect("sqlite::memory:")
    .await?;
```

`sqlite::memory:` is a database that lives in RAM and disappears when
the pool drops — perfect for tests, since each one starts empty. A pool
is cheap to `Clone` and safe to share across tasks, which is exactly why
it's what you hand to axum's `.with_state(...)` (Lesson 23). The
one-connection cap is deliberate: SQLite takes one writer at a time, and
capping the pool turns a statement that can never get a connection into
a quick error instead of a hang — you'll meet that case under
Transactions below.

### Migrations

Your schema has a history, and that history is a directory of numbered
`.sql` files:

```text
migrations/
├── 0001_create_accounts.sql    CREATE TABLE accounts (...)
└── 0002_add_balance.sql        ALTER TABLE accounts ADD COLUMN balance ...
```

The second file also caps a balance at 1000, with
`CHECK (balance <= 1000)` — a rule the database enforces on every write,
which the main exercise leans on.

`sqlx::migrate!().run(&pool).await?` embeds those files at *compile*
time and applies the ones that haven't run yet, recording each in a
`_sqlx_migrations` table so it never runs one twice. Migrations are
append-only: never edit one that has been applied — add a new file
instead; sqlx records each file's checksum and refuses to run a modified
one. (The crate's three-line `build.rs` is what makes cargo notice a
newly added `.sql` file.)

### Queries, binding, and `sqlx::Error`

You write the SQL; values go in through `bind`:

```rust
let result = sqlx::query("INSERT INTO accounts (name, balance) VALUES (?, ?)")
    .bind(name)
    .bind(balance)
    .execute(&pool)
    .await?;

result.rows_affected();      // how many rows changed
result.last_insert_rowid();  // the new row's id (SQLite-specific)
```

One `.bind(...)` per `?`, in order. (`&pool` here because this snippet
owns the pool; in the exercises the parameter is already a
`&SqlitePool`, so you pass `pool` — a `&&SqlitePool` is not an
executor.) Never `format!` a value into the SQL string — that's how SQL
injection happens, and binding is both safer and faster. `.execute()`
runs a statement; `.fetch_one()`, `.fetch_optional()` and `.fetch_all()`
return rows. Everything is
`.await`ed, and every query returns `Result<_, sqlx::Error>`, which you
propagate with `?` (Lesson 14). (`migrate!().run()` returns a
`MigrateError`, which `?` converts into `sqlx::Error`.) `?` is SQLite's
and MySQL's placeholder; Postgres writes `$1`, and
`last_insert_rowid()` becomes `INSERT ... RETURNING id` there.

### Typed rows — `FromRow` and `query_as`

`query_as` maps columns onto a struct by field name:

```rust
#[derive(Debug, sqlx::FromRow)]
struct Account {
    id: i64,
    name: String,
    balance: i64,
}

let rows = sqlx::query_as::<_, Account>("SELECT id, name, balance FROM accounts")
    .fetch_all(&pool)
    .await?;
```

A SQLite `INTEGER` is an `i64` in Rust — that's what these columns decode
to and what `last_insert_rowid()` returns. (A narrower `i32` compiles
too, but fails at *run* time on a value that doesn't fit.) Nullable
columns map to `Option<T>`. When you want one value rather than a row,
`query_scalar` gives it to you directly:
`let n: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM accounts").fetch_one(&pool).await?;`
— that annotation is required, since `query_scalar` decodes into whatever
type you ask for. Without it the compiler reports a pile of errors
mentioning `!`, the never type, rather than the missing annotation.

### Transactions

Two updates that must both happen, or neither, belong in a transaction:

```rust
let mut tx = pool.begin().await?;

sqlx::query("UPDATE accounts SET balance = balance - ? WHERE id = ?")
    .bind(amount)
    .bind(from)
    .execute(&mut *tx)          // through the transaction, not the pool
    .await?;

tx.commit().await?;             // or tx.rollback().await?
```

`&mut *tx` is the whole trick: a `Transaction` derefs to a connection,
and that connection is what `.execute()` wants. Write `&mut tx` without
the `*` and the compiler says "the trait bound `&mut Transaction<'_,
Sqlite>: Executor<'_>` is not satisfied" and lists
`&mut SqliteConnection` among the types that do work — that's the hint.
Both `commit` and `rollback` take `self`, so a committed transaction is
gone and the compiler rejects any later use of it — and if you drop one
without committing, it rolls back. That last guarantee is what makes `?`
safe inside a transaction: an early return rolls it back for you.

`pool.begin()` is also what holds one connection for the whole
transaction — sending `BEGIN` as a query string does not, so on a bigger
pool your later statements would land on other connections.

Calling `rollback()` where you decide to abort is worth doing even
though a drop would roll back too: it says so out loud, and it hands you
a `Result` you can propagate.

A statement you send to `&pool` isn't in the transaction at all. On a
bigger pool it would take its own connection and then block on SQLite's
write lock, which the open transaction holds — the hang the cap exists
to prevent. Here it simply can't get a connection, so **if a test stalls
for two seconds and then fails with `Database(PoolTimedOut)`, you sent a
statement to `&pool` while your transaction was open.**

## Exercises

### Warm-up: `insert_account`

Implement `insert_account` so it inserts one row and returns its new id:

```rust
pub async fn insert_account(
    _pool: &SqlitePool,
    _name: &str,
    _balance: i64,
) -> Result<i64, sqlx::Error> {
    // sqlx::query("INSERT INTO accounts (name, balance) VALUES (?, ?)")
    //     .bind(name).bind(balance).execute(pool).await?
    todo!("INSERT the account, then return its new row id")
}
```

The stub's parameters start with `_` because this course's lints make an
unused variable a compile error — rename them in the same edit where you
replace `todo!()`. `execute` returns a result; `last_insert_rowid()` on
it is the new id.

### Main: `transfer`

Implement `transfer` so it moves `amount` from one account to another,
atomically:

```rust
pub async fn transfer(
    _pool: &SqlitePool,
    _from: i64,
    _to: i64,
    _amount: i64,
) -> Result<(), TransferError> {
    todo!("move the money inside one transaction; roll back an overdraft")
}
```

Open the transaction with `pool.begin()`, then run all three statements
through `&mut *tx`:

1. `UPDATE accounts SET balance = balance - ? WHERE id = ?` — bind
   `amount`, then `from`
2. the same with `+` and `to` — the receiver's side
3. `SELECT balance FROM accounts WHERE id = ?` — bind `from`, and read
   the single value back with
   `let balance: i64 = sqlx::query_scalar(...).fetch_one(&mut *tx).await?;`
   The `: i64` is not optional — without it you get the never-type error
   pile described above.

If that balance is negative, `tx.rollback().await?` and return
`TransferError::InsufficientFunds { id: from, balance: balance + amount, amount }`
— the `+ amount` puts back what you just debited, so the error reports
what the sender had *before* the transfer. Otherwise `tx.commit().await?`.

`transfer` does not validate `amount` — the database's `CHECK` is the
only guard, and a negative amount is a legal call. Two tests make the
database reject one of the updates: one transfers into an account
already at the 1000 cap, and one passes a negative amount, which makes
the sender's own update break the cap. You don't need to handle either
case — `?` returns early, the transaction drops, and the drop rolls back
whatever already ran.

You could instead read both balances first and re-check the database's
rules in Rust before writing anything. The tests won't catch that, but
it's the wrong habit: your copy of the rules goes stale the moment the
schema changes, and in a real deployment — not this private, one-writer
in-memory database — another writer can change the row between your
check and your write. The transaction gets it right without you knowing
the rules at all.

Finish the warm-up first: the transfer tests seed their accounts with
`insert_account`, so all eight fail until it works. `Account`,
`TransferError`, `connect` and `all_accounts` are given — read them.

### Compile-fail

`exercises/compile_fails/24-tx-consumed.rs` runs one more statement
after committing. Because `commit` takes `self`, the compiler rejects it
(E0382 — borrow of moved value). Fix it by finishing the work before the
commit. You can check this one without waiting for the tests:
`cargo run --package compile-fails -- --expect compiles lessons/24-persistence`.

### Run

```bash
make verify LESSON=24-persistence
```

This runs your exercise tests and asserts the compile-fail file now
compiles. The first run pays the two costs from the top of this page —
the 1.98 toolchain download and building SQLite from C source (Xcode
command line tools on macOS, `build-essential` on Debian/Ubuntu). A few
minutes once; every later run is fast.

## Solutions

See `solutions/src/lib.rs` for the reference implementations. Try the
exercises before peeking.
