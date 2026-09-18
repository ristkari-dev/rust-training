# Persistence

> A service that forgets everything when it restarts isn't a service. `sqlx` lets you talk to a database in plain SQL, asynchronously, with Rust types on both ends — and its transactions give you the one guarantee that makes writes safe: all of it, or none of it.

---

## What `sqlx` is

An async SQL toolkit for **Postgres**, **MySQL** and **SQLite**. Not an ORM, not a query builder — you write SQL.

This lesson uses SQLite *in memory*: nothing to install, nothing to start, and every test begins from an empty database.

---

## The pool

```rust
let pool = SqlitePool::connect("sqlite::memory:").await?;
```

A pool opens connections and hands them out as queries need them. It is cheap to `Clone` and safe to share — which is exactly why it's what you hand to axum's `.with_state(...)` (Lesson 23).

The lesson's given `connect()` builds its pool through `SqlitePoolOptions` instead, capped at one connection with a two-second timeout: SQLite takes one writer at a time, and the cap turns a statement you accidentally send to the pool during an open transaction into a quick `PoolTimedOut` instead of a hang.

---

## Migrations

```rust
sqlx::migrate!().run(&pool).await?;
```

A `migrations/` directory of numbered `.sql` files is your schema's history. `migrate!()` embeds them at *compile* time and applies the ones that haven't run yet, recording each in a `_sqlx_migrations` table.

Never edit an applied migration — add a new one. Today's second migration adds `balance INTEGER NOT NULL DEFAULT 0 CHECK (balance <= 1000)` — a rule the database enforces on every write, which is how a transfer can fail *after* it has already moved money.

---

## Queries and binding

```rust
sqlx::query("INSERT INTO accounts (name, balance) VALUES (?, ?)")
    .bind(name)
    .bind(balance)
```

Bind values; never `format!` them into the SQL string — that's how SQL injection happens.

`?` is SQLite's and MySQL's placeholder, `$1` is Postgres's.

---

## Running a query

```rust
let result = sqlx::query("INSERT ...").execute(&pool).await?;   // rows_affected(), last_insert_rowid()
let row = sqlx::query("SELECT ...").fetch_one(&pool).await?;    // also fetch_optional, fetch_all
```

Everything is `.await`ed, and everything returns `Result<_, sqlx::Error>` — propagate it with `?` (Lesson 14).

---

## Typed rows

```rust
#[derive(sqlx::FromRow)]
struct Account { id: i64, name: String, balance: i64 }

let rows = sqlx::query_as::<_, Account>("SELECT id, name, balance FROM accounts")
    .fetch_all(&pool)
    .await?;
```

`FromRow` maps columns onto fields by name; `query_scalar` pulls a single value (and needs a type annotation). A SQLite `INTEGER` is an `i64` — a narrower `i32` compiles, but fails at run time on a value that doesn't fit. (`sqlx::query!` checks SQL against a real database at compile time — it needs a live `DATABASE_URL` or a checked-in `.sqlx` cache while building, so this course uses the runtime API.)

---

## Transactions

```rust
let mut tx = pool.begin().await?;
sqlx::query("UPDATE ...").execute(&mut *tx).await?;
tx.commit().await?;                       // or tx.rollback().await?
```

Every statement goes through `&mut *tx` — one sent to `&pool` instead is **not** in the transaction.

`commit` and `rollback` both take `self`, so the compiler stops you using the transaction afterwards. Drop one without committing and it rolls back.

---

## Putting it together

Today's exercises (the migrations, `Account`, `TransferError`, `connect()` and `all_accounts()` are given):

- **Warm-up** `insert_account` — bind and execute an INSERT, return the new id
- **Main** `transfer` — move money inside a transaction, roll back an overdraft and any update the database rejects (the tests make it the credit in one case and, via a negative amount, the debit in the other)

The compile-fail runs one more statement after committing.

---

## Wrap — persistence in Rust

- a pool is the shared, cloneable handle to your database
- migrations are an ordered, append-only history run by `sqlx::migrate!()`
- `query(...).bind(...)` is the injection-safe way to send values
- `query_as` + `FromRow` turns rows into your types
- a transaction is all-or-nothing, and `commit`/`rollback` consume it

Next: **Lesson 25 — Observability** (`tracing`, structured logs, metrics) — where you'll watch these very queries go by.
