# Lesson 24 — Persistence — design

The third lesson of Phase 6 (Production services). `sqlx` is the async
SQL toolkit: you write plain SQL, bind values to it, and get your own
Rust types back. The production skill at the center: a **transaction**
is all-or-nothing, and you get that guarantee by routing every statement
through the transaction handle. Builds on error handling (L14), async
(L18), testing-without-the-real-thing (L21), and the pool that a handler
reads through `State` (L23).

A deliberate constraint: the exercises run against **SQLite in memory**
(`sqlite::memory:`), so there is nothing to install, nothing to start,
and every test begins from an empty database. The compile-time-checked
`sqlx::query!` macros are out: they need a live database at build time,
which no student of this course should have to arrange.

## Audience and prerequisites

- Has completed Lessons 01-23
- Comfortable with `Result`/`?` and `thiserror` (L14), `async`/`.await`
  and `#[tokio::test]` (L18), and adding a dependency (L14/L18/L22/L23)
- Has the toolchain installed and `make verify` working

## Learning goals

By the end of this lesson, the student can:

1. Open a connection pool with `SqlitePool` / `SqlitePoolOptions`, and
   explain that a pool is the cheap-to-clone shared handle that goes in
   an axum `State`
2. Run schema migrations from an ordered `migrations/` directory with
   `sqlx::migrate!()`, and explain that migrations are append-only and
   recorded in a `_sqlx_migrations` table
3. Send a query with `sqlx::query(...).bind(...)` — never
   string-formatted SQL — and run it with `.execute()` / `.fetch_one()`
   / `.fetch_all()`, propagating `sqlx::Error` with `?`
4. Map result rows onto a Rust struct with `#[derive(sqlx::FromRow)]`
   and `query_as`, and know that a SQLite `INTEGER` column is an `i64`
5. Group several statements into one atomic unit with `pool.begin()`,
   route each statement through `&mut *tx`, and finish with `commit()`
   or `rollback()` — both of which consume the transaction, which is why
   the compiler rejects using it afterwards

## Scope

In scope: `SqlitePool` and `SqlitePoolOptions` (`max_connections`,
`acquire_timeout`); `sqlite::memory:` as the test database; migrations
as numbered `.sql` files applied by `sqlx::migrate!()`; the runtime
query API (`sqlx::query`, `query_as`, `query_scalar`) with `.bind(...)`
placeholders; `#[derive(sqlx::FromRow)]`; `sqlx::Error` and a domain
error type built with `thiserror` (L14); transactions — `pool.begin()`,
`&mut *tx`, `commit()`, `rollback()`, and the drop-rolls-back
guarantee. New infrastructure: the workspace toolchain moves from 1.85
to 1.98, and `sqlx` joins `[workspace.dependencies]`.

Out of scope (deferred or skipped): the compile-time-checked
`sqlx::query!` / `query_as!` macros (they need `DATABASE_URL` at build
time or a checked-in `.sqlx` cache — named on a slide and in the README,
and forbidden in the lesson's code); Postgres and MySQL beyond a
mention of placeholder syntax; connection-pool tuning; isolation levels
and savepoints; `SELECT ... FOR UPDATE SKIP LOCKED` (the capstone,
L31); indexes and query plans; N+1 and query performance; ORMs
(SeaORM, Diesel); JSON columns and serde (L29); `sqlx-cli`; a real
file-backed or networked database. Persistence is introduced as *a
pool, migrations, bound queries, typed rows, and transactions, tested
against an in-memory database*; operating a real database is out of
band.

## New dependency infrastructure

### Toolchain bump: 1.85 → 1.98

`sqlx` cannot build on the repo's pinned Rust 1.85. `sqlx-core` depends
on `url` → `idna` → `icu_*` → `yoke 0.8.3`, whose proc-macro crate
`yoke-derive 0.8.3` calls `str::from_utf8` (an associated function
stabilized after 1.85) and publishes **no `rust-version` field**, so
cargo's MSRV-aware resolver cannot filter it out. A fresh clone fails
with:

```
error[E0599]: no function or associated item named `from_utf8` found for type `str`
   --> yoke-derive-0.8.3/src/lib.rs:202:32
error: could not compile `yoke-derive` (lib) due to 1 previous error
```

Pinning `yoke-derive` to 0.8.2 fixes it, but `Cargo.lock` is gitignored
in this repo, so the pin would never reach a student. The lesson
therefore raises the floor instead:

- `rust-toolchain.toml`: `channel = "1.85"` → `channel = "1.98"`
- root `Cargo.toml`: `rust-version = "1.85"` → `rust-version = "1.98"`

Verified on 2026-09-18 against the repo at `093e3bf`, toolchain
`rustc 1.98.1 (48a229cea 2026-09-01)`: `cargo clippy --workspace
--all-targets -- -D warnings` exits 0 with no output, `cargo fmt --all
--check` exits 0, and `cargo test` reports no failing test result — so
lessons 01-23 need no changes. This is its own commit, landed before the
lesson content.

### `sqlx` as a workspace dependency

`sqlx` is **not** yet a workspace dependency. The plan adds, to the root
`[workspace.dependencies]`:

```toml
sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio", "sqlite", "migrate", "macros"] }
```

`default-features = false` is load-bearing: sqlx's defaults
(`any`, `macros`, `migrate`, `json`) drag in the `any` driver and
`serde_json` for no teaching benefit. The four features kept are the
minimum: `runtime-tokio` (the async runtime), `sqlite` (the driver —
it statically compiles SQLite through `libsqlite3-sys`'s `bundled`
feature, so no system library and no server are needed), `migrate` and
`macros` (together, what `sqlx::migrate!()` needs).

`thiserror` and `tokio` are already workspace dependencies. Resolved
during design: sqlx 0.8.6, libsqlite3-sys 0.30.1, ~180 packages, ~115
crates compiled, about 15-20 seconds cold on a developer machine.

**This is the first lesson whose dependency compiles C.** The `bundled`
SQLite build needs a working `cc` on the machine. GitHub's
`ubuntu-latest` runner has one, and `deploy/Dockerfile` never builds the
lesson crates (only `--package build-index`), so neither CI nor Deploy
is at risk. Students on macOS/Linux with the usual developer tooling
already have one; this is worth one sentence in the README.

Both lesson crates add, after `[lints]`:

```toml
[dependencies]
sqlx = { workspace = true }
thiserror = { workspace = true }
tokio = { workspace = true }
```

`Cargo.lock` stays gitignored (CI regenerates it); the CI cache key
hashes `**/Cargo.toml`, so the first run after this change refetches
cleanly — a one-time cold cache on both the stable and beta legs, as
lessons 18 and 23 also paid.

## Slide arc (10 slides)

1. **Title — Persistence.** Hook: *"A service that forgets everything
   when it restarts isn't a service. `sqlx` lets you talk to a database
   in plain SQL, asynchronously, with Rust types on both ends — and its
   transactions give you the one guarantee that makes writes safe: all
   of it, or none of it."*
2. **What `sqlx` is.** An async SQL toolkit for Postgres, MySQL and
   SQLite. Not an ORM, not a query builder — you write SQL. This lesson
   uses SQLite in memory, so there is nothing to install and every test
   starts from an empty database.
3. **The pool.**
   ```rust
   let pool = SqlitePool::connect("sqlite::memory:").await?;
   ```
   A pool opens connections and hands them out as queries need them.
   It is cheap to `Clone` and safe to share — which is exactly why it's
   what you hand to axum's `.with_state(...)` (Lesson 23). The lesson's
   given `connect()` builds the same pool through `SqlitePoolOptions`,
   capped at one connection: SQLite takes one writer at a time, and the
   cap turns a stuck read into a quick error instead of a hang.
4. **Migrations.**
   ```rust
   sqlx::migrate!().run(&pool).await?;
   ```
   A `migrations/` directory of numbered `.sql` files is your schema's
   history. `migrate!()` embeds them at *compile* time and applies the
   ones that haven't run yet, recording each in a `_sqlx_migrations`
   table. Never edit an applied migration — add a new one.
5. **Queries and binding.**
   ```rust
   sqlx::query("INSERT INTO accounts (name, balance) VALUES (?, ?)")
       .bind(name)
       .bind(balance)
   ```
   Bind values; never `format!` them into SQL. `?` is SQLite's and
   MySQL's placeholder, `$1` is Postgres's.
6. **Running a query.** `.execute(&pool)` returns a result carrying
   `rows_affected()` and `last_insert_rowid()`; `.fetch_one()`,
   `.fetch_optional()` and `.fetch_all()` return rows. Everything is
   `.await`ed and everything returns `Result<_, sqlx::Error>`, which you
   propagate with `?` (Lesson 14).
7. **Typed rows.**
   ```rust
   #[derive(sqlx::FromRow)]
   struct Account { id: i64, name: String, balance: i64 }

   let rows = sqlx::query_as::<_, Account>("SELECT id, name, balance FROM accounts")
       .fetch_all(&pool)
       .await?;
   ```
   `FromRow` maps columns onto fields by name; `query_scalar` pulls a
   single value. A SQLite `INTEGER` is an `i64` in Rust. (There is also
   `sqlx::query!`, checked against a real database at compile time — it
   needs a `DATABASE_URL` while building, so this course uses the
   runtime API.)
8. **Transactions.**
   ```rust
   let mut tx = pool.begin().await?;
   sqlx::query("UPDATE ...").execute(&mut *tx).await?;
   tx.commit().await?;          // or tx.rollback().await?
   ```
   Every statement goes through `&mut *tx` — one that goes to `&pool`
   instead is *not* in the transaction. Both `commit` and `rollback`
   take `self`, so the compiler stops you using the transaction
   afterwards, and dropping one without committing rolls it back.
9. **Putting it together.** Walk through the exercises: `insert_account`
   binds and executes an INSERT and returns the new id (warm-up);
   `transfer` moves money inside a transaction and rolls back if the
   sender would go negative (main). The migrations, `Account`,
   `TransferError`, `connect()` and `all_accounts()` are given — read
   them. The compile-fail uses a transaction after committing it.
10. **Wrap — persistence in Rust.** Five takeaways: a pool is the
    shared, cloneable handle to your database; migrations are an
    ordered, append-only schema history run by `sqlx::migrate!()`;
    `query(...).bind(...)` is the injection-safe way to send values;
    `query_as` + `FromRow` turns rows into your types; a transaction is
    all-or-nothing, and `commit`/`rollback` consume it. Next:
    **Lesson 25 — Observability** (`tracing`, structured logs, metrics),
    where you'll watch these very queries go by.

## Exercise spec

`lessons/24-persistence/` follows the standard four-part lesson shape,
plus a `migrations/` directory and a `build.rs` in each crate:

```
24-persistence/
├── README.md
├── slides/
│   ├── index.html
│   └── slides.md
├── exercises/
│   ├── Cargo.toml          # adds sqlx + thiserror + tokio (workspace deps)
│   ├── build.rs
│   ├── migrations/
│   │   ├── 0001_create_accounts.sql
│   │   └── 0002_add_balance.sql
│   ├── src/lib.rs
│   ├── tests/exercise.rs
│   └── compile_fails/24-tx-consumed.rs
└── solutions/
    ├── Cargo.toml
    ├── build.rs
    ├── migrations/
    │   ├── 0001_create_accounts.sql
    │   └── 0002_add_balance.sql
    ├── src/lib.rs
    └── tests/exercise.rs
```

Cargo package names: `persistence-exercises` and `persistence-solutions`
(import idents `persistence_exercises` / `persistence_solutions`). This
matches the build-index registry slug `persistence`, so the landing page
links lesson 24 without any tool change.

The `migrations/` directory is duplicated in both crates because
`sqlx::migrate!()` resolves relative to `$CARGO_MANIFEST_DIR` — the same
way `tests/exercise.rs` is already duplicated across the two crates.

### `build.rs` (both crates)

```rust
fn main() {
    println!("cargo:rerun-if-changed=migrations");
}
```

`migrate!()` embeds the `.sql` files at compile time, but a proc macro
cannot register a rebuild trigger. Without this file, editing or adding
a migration leaves the compiled-in copy stale and the change silently
does nothing.

### Migrations (both crates)

`migrations/0001_create_accounts.sql`:

```sql
CREATE TABLE accounts (
    id   INTEGER PRIMARY KEY,
    name TEXT    NOT NULL
);
```

`migrations/0002_add_balance.sql`:

```sql
ALTER TABLE accounts ADD COLUMN balance INTEGER NOT NULL DEFAULT 0;
```

Two files, not one, so the ordered-history idea is visible: the schema
*grew*, and the second file is how you add a column without touching
the first.

### Exercise stub (`exercises/src/lib.rs`)

`Account`, `TransferError`, `connect()` and `all_accounts()` ship as
given; the two exercises ship with `todo!()` bodies. The crate and tests
compile; the tests fail at runtime with the `todo!()` panic.

```rust
//! Lesson 24 — exercises.
//!
//! Implement `insert_account` (warm-up) and `transfer` (main) so that
//! `cargo test --manifest-path lessons/24-persistence/exercises/Cargo.toml`
//! passes. `Account`, `TransferError`, `connect` and `all_accounts` are
//! given. The tests live in `tests/exercise.rs`.

use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use std::time::Duration;

/// One row of the `accounts` table.
#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)]
pub struct Account {
    pub id: i64,
    pub name: String,
    pub balance: i64,
}

/// Why a transfer failed.
#[derive(Debug, thiserror::Error)]
pub enum TransferError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("account {id} has {balance}, cannot send {amount}")]
    InsufficientFunds { id: i64, balance: i64, amount: i64 },
}

/// Open a fresh in-memory database and run the migrations.
///
/// One connection means one database — and it turns "I read through the
/// pool while my transaction was open" from a hang into a quick error.
pub async fn connect() -> Result<SqlitePool, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .acquire_timeout(Duration::from_secs(2))
        .connect("sqlite::memory:")
        .await?;
    sqlx::migrate!().run(&pool).await?;
    Ok(pool)
}

/// Read every account back, ordered by id.
pub async fn all_accounts(pool: &SqlitePool) -> Result<Vec<Account>, sqlx::Error> {
    sqlx::query_as::<_, Account>("SELECT id, name, balance FROM accounts ORDER BY id")
        .fetch_all(pool)
        .await
}

pub async fn insert_account(
    _pool: &SqlitePool,
    _name: &str,
    _balance: i64,
) -> Result<i64, sqlx::Error> {
    todo!("INSERT the account, then return its new row id")
}

pub async fn transfer(
    _pool: &SqlitePool,
    _from: i64,
    _to: i64,
    _amount: i64,
) -> Result<(), TransferError> {
    todo!("move the money inside one transaction; roll back an overdraft")
}
```

### Warm-up: `insert_account`

Reference solution:

```rust
pub async fn insert_account(
    pool: &SqlitePool,
    name: &str,
    balance: i64,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO accounts (name, balance) VALUES (?, ?)")
        .bind(name)
        .bind(balance)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid())
}
```

Pedagogical packing: the whole write path in six lines — SQL with `?`
placeholders, one `.bind(...)` per placeholder in order, `.execute(pool)`
to run it, `?` to propagate `sqlx::Error` (L14), and `last_insert_rowid()`
off the result to learn the new row's id. It is also where the
injection-safety habit is set: the values never touch the SQL string.
(`last_insert_rowid()` is SQLite-specific — Postgres uses
`INSERT ... RETURNING id`. One sentence in the README says so.)

Four tests (`#[tokio::test]`, each on its own fresh database):

```rust
#[tokio::test]
async fn warmup_first_row_gets_id_one() {
    let pool = connect().await.unwrap();
    assert_eq!(insert_account(&pool, "alice", 100).await.unwrap(), 1);
}

#[tokio::test]
async fn warmup_ids_increment() {
    let pool = connect().await.unwrap();
    assert_eq!(insert_account(&pool, "alice", 100).await.unwrap(), 1);
    assert_eq!(insert_account(&pool, "bob", 50).await.unwrap(), 2);
}

#[tokio::test]
async fn warmup_row_reads_back() {
    let pool = connect().await.unwrap();
    let id = insert_account(&pool, "alice", 100).await.unwrap();
    assert_eq!(
        all_accounts(&pool).await.unwrap(),
        vec![Account {
            id,
            name: "alice".to_string(),
            balance: 100
        }]
    );
}

#[tokio::test]
async fn warmup_stores_a_zero_balance() {
    let pool = connect().await.unwrap();
    let id = insert_account(&pool, "carol", 0).await.unwrap();
    let rows = all_accounts(&pool).await.unwrap();
    assert_eq!(
        rows[0],
        Account {
            id,
            name: "carol".to_string(),
            balance: 0
        }
    );
}
```

### Main: `transfer`

Reference solution:

```rust
pub async fn transfer(
    pool: &SqlitePool,
    from: i64,
    to: i64,
    amount: i64,
) -> Result<(), TransferError> {
    let mut tx = pool.begin().await?;

    sqlx::query("UPDATE accounts SET balance = balance - ? WHERE id = ?")
        .bind(amount)
        .bind(from)
        .execute(&mut *tx)
        .await?;
    sqlx::query("UPDATE accounts SET balance = balance + ? WHERE id = ?")
        .bind(amount)
        .bind(to)
        .execute(&mut *tx)
        .await?;

    let balance: i64 = sqlx::query_scalar("SELECT balance FROM accounts WHERE id = ?")
        .bind(from)
        .fetch_one(&mut *tx)
        .await?;

    if balance < 0 {
        tx.rollback().await?;
        return Err(TransferError::InsufficientFunds {
            id: from,
            balance: balance + amount,
            amount,
        });
    }
    tx.commit().await?;
    Ok(())
}
```

Pedagogical packing: the transaction lifecycle end to end — `begin()`,
three statements routed through `&mut *tx`, a business rule checked
*inside* the transaction against uncommitted state, then `rollback()`
with a typed domain error or `commit()`. Two things a student can only
learn by doing it: `&mut *tx` (a `Transaction` derefs to a connection,
and that is what `.execute` wants), and reading the balance *after* the
UPDATE, which is why the reported balance adds the amount back. It also
reuses `thiserror`'s `#[from]` (L14): `?` on a `sqlx::Error` becomes
`TransferError::Database` for free, while the overdraft is a distinct,
matchable variant.

Four tests:

```rust
async fn seeded() -> sqlx::SqlitePool {
    let pool = connect().await.unwrap();
    insert_account(&pool, "alice", 100).await.unwrap();
    insert_account(&pool, "bob", 50).await.unwrap();
    pool
}

#[tokio::test]
async fn main_transfer_commits() {
    let pool = seeded().await;
    transfer(&pool, 1, 2, 30).await.unwrap();
    let rows = all_accounts(&pool).await.unwrap();
    assert_eq!(rows[0].balance, 70);
    assert_eq!(rows[1].balance, 80);
}

#[tokio::test]
async fn main_transfer_whole_balance_commits() {
    let pool = seeded().await;
    transfer(&pool, 1, 2, 100).await.unwrap();
    let rows = all_accounts(&pool).await.unwrap();
    assert_eq!(rows[0].balance, 0);
    assert_eq!(rows[1].balance, 150);
}

#[tokio::test]
async fn main_overdraft_is_rejected() {
    let pool = seeded().await;
    let err = transfer(&pool, 1, 2, 500).await.unwrap_err();
    assert!(matches!(
        err,
        TransferError::InsufficientFunds {
            id: 1,
            balance: 100,
            amount: 500
        }
    ));
}

#[tokio::test]
async fn main_overdraft_rolls_everything_back() {
    let pool = seeded().await;
    assert!(transfer(&pool, 1, 2, 500).await.is_err());
    let rows = all_accounts(&pool).await.unwrap();
    assert_eq!(rows[0].balance, 100, "the sender must be untouched");
    assert_eq!(rows[1].balance, 50, "the receiver must be untouched too");
}
```

**Eight tests total** (four warm-up + four main), all `#[tokio::test]`,
each opening its own in-memory database through `connect()` — no shared
state, no external service, deterministic. `main_overdraft_rolls_everything_back`
is the load-bearing one: it is the only test that fails if a student
writes `.execute(pool)` instead of `.execute(&mut *tx)`, and it fails
loudly, with `left: -400, right: 100`.

The four main tests seed through `insert_account`, so a student who has
not finished the warm-up sees all eight fail rather than four. Lesson 22
has the same shape (`main_run_via_parse` calls `parse`); the README's
Main subsection says to finish the warm-up first.

### Compile-fail: `24-tx-consumed.rs`

Path: `exercises/compile_fails/24-tx-consumed.rs`. Self-contained and
std-only — the `compile-fails` tool type-checks with bare `rustc` and no
`--extern`, so it cannot use `sqlx`. It mirrors sqlx's real signature
(`pub async fn commit(mut self)`, sqlx-core 0.8.6 `src/transaction.rs`)
with a plain struct whose `commit` takes `self`.

```rust
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
```

Pass condition: the student moves the second `execute` call above
`commit`. rustc reports E0382 "borrow of moved value: `tx`" plus the
`takes ownership of the receiver` note — verified during design against
rustc 1.98.1 under the tool's exact invocation. After the reorder the
file type-checks.

This is the lesson's ownership payback (L07): a committed transaction is
*gone*, and the type system is what tells you.

## README structure

`lessons/24-persistence/README.md` follows the established shape:

- **Title + one-paragraph hook**
- **Learning goals** — the five bullets above
- **Self-study notes** with five subsections:
  - The pool and the database
  - Migrations
  - Queries, binding, and `sqlx::Error`
  - Typed rows — `FromRow` and `query_as`
  - Transactions
- **Exercises** — four subsections: Warm-up (`insert_account`), Main
  (`transfer`), Compile-fail, Run
- **Solutions** — pointer to `solutions/src/lib.rs`

Each `###` subsection runs ~4-6 sentences plus a small code block. The
"Transactions" and "Queries, binding, and `sqlx::Error`" sections are
the heaviest — they carry the atomicity guarantee and the
injection-safety habit. The Run subsection notes that the first build
compiles SQLite from source and needs a C compiler.

## Lint expectations

Lesson 24's reference solution is clippy-clean (`clippy::all` +
`clippy::pedantic` denied) with **no `#[allow]` attributes** — verified
during design by building the complete lesson under the repo's exact
lint table:

- No `#[must_use]` anywhere: every function returns a `Result` or a
  `Future`, both already `#[must_use]`, so adding it would trip
  `clippy::double_must_use`.
- `Account` derives `Debug, Clone, PartialEq, Eq, sqlx::FromRow`.
  Deriving `Eq` beside `PartialEq` satisfies
  `clippy::derive_partial_eq_without_eq`; the tests compare whole
  `Account` values, which is what makes `PartialEq` earn its place.
- `TransferError` derives `Debug, thiserror::Error`. `sqlx::Error` is
  large, but `clippy::result_large_err` does not fire at this size —
  if it ever does, box the variant rather than allowing the lint.
- The stub's parameters are `_`-prefixed (`_pool`, `_name`, …) because
  the workspace denies `unused`; every import is used by a signature or
  by the given code, so there are no unused-import warnings, and the
  `todo!()` bodies lint clean.
- The lesson's code uses the runtime query API only. `sqlx::query!` and
  `query_as!` must not appear in either crate: they would require
  `DATABASE_URL` at build time and break `make ci` on a fresh clone.

If clippy fires on anything unexpected, fix the code rather than adding
an allow, and report it.

## Done criteria

- `rust-toolchain.toml` and root `Cargo.toml` pin 1.98; `make ci` is
  green on lessons 01-23 with no other change
- `lessons/24-persistence/` exists with the four-part structure, plus
  `build.rs` and `migrations/` in both crates
- Root `Cargo.toml` `[workspace.dependencies]` includes the `sqlx` entry
  with `default-features = false` and exactly the four features; both
  lesson `Cargo.toml`s declare `sqlx`, `thiserror` and `tokio` as
  workspace dependencies
- Cargo manifests use the package names `persistence-exercises` and
  `persistence-solutions`
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same
  `Account`, `TransferError`, `connect`, `all_accounts`,
  `insert_account` and `transfer`; the exercise ships `todo!()` bodies
  for the last two, the solution ships real ones
- Both crates' `migrations/` hold the same two `.sql` files
- `cargo test --package persistence-solutions` → 8 tests pass
- `cargo test --manifest-path lessons/24-persistence/exercises/Cargo.toml`
  → compiles, all 8 tests panic with `not yet implemented`
- `cargo run --package compile-fails -- --expect broken lessons/24-persistence`
  → ok
- `cargo run --package compile-fails -- --expect compiles lessons/24-persistence`
  → fails (the file ships broken, that's the point)
- `make ci` is green
- `make slides-build` produces `dist/lessons/24-persistence/slides/index.html`
- `dist/index.html` lists lesson 24 as a clickable link (registry slug
  `persistence` already matches this directory name)
- One push to `origin/main` triggers a green CI run and a green Deploy
  run; `https://rust.ristkari.dev/lessons/24-persistence/slides/`
  returns 200

## Open questions

None.
