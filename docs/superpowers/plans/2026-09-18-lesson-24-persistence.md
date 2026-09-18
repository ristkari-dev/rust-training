# Lesson 24 — Persistence — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Author the third lesson of Phase 6 of the Rust training course: persistence with `sqlx`. The production skill: a transaction is all-or-nothing, and you get that by routing every statement through the transaction handle. Warm-up: `insert_account` (bind + execute + `last_insert_rowid`). Main: `transfer` (begin, `&mut *tx`, rollback an overdraft, commit). Compile-fail: using a transaction after `commit(self)` consumed it (E0382).

**Architecture:** Raise the workspace toolchain 1.85 → 1.98 first, in its own commit (sqlx cannot build on 1.85 — see Task 1). Then scaffold the lesson, add `sqlx` to `[workspace.dependencies]`, and wire both lesson crates with `sqlx` + `thiserror` + `tokio`. Each lesson crate carries its own `migrations/` directory and a three-line `build.rs`, because `sqlx::migrate!()` resolves relative to `$CARGO_MANIFEST_DIR`, and because the macro's `include_str!`s track existing files but not the directory — so a newly *added* migration needs `cargo:rerun-if-changed=migrations` to be noticed. `Account`, `TransferError`, `connect()` and `all_accounts()` ship as given; students implement two functions. Tests are `#[tokio::test]`s, each opening its own `sqlite::memory:` database — no external service.

**Tech Stack:** Rust 2024 edition (toolchain moves to 1.98), `sqlx` 0.8.6 with `runtime-tokio` + `sqlite` + `migrate` + `macros` (new workspace dependency), `thiserror` 2 and `tokio` 1 (existing workspace dependencies), existing tools (`new-lesson`, `compile-fails`, `slides-dev`, `build-index`), reveal.js (vendored), GNU Make.

**Spec:** [`docs/superpowers/specs/2026-09-18-lesson-24-persistence-design.md`](../specs/2026-09-18-lesson-24-persistence-design.md).

**Working directory:** `/Users/ristkari/code/private/rust-training`.

**Commit convention:** Plain commit messages only — no `Co-Authored-By` trailer or any AI attribution. If a commit fails with a GPG/pinentry error, simply retry the same `git commit` command once or twice.

## Global Constraints

- Lesson directory: `lessons/24-persistence/`; Cargo package names `persistence-exercises` and `persistence-solutions` (import idents `persistence_exercises` / `persistence_solutions`).
- Root `Cargo.toml` `[workspace.dependencies]` gains exactly: `sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio", "sqlite", "migrate", "macros"] }`. `default-features = false` is load-bearing — the defaults drag in the `any` driver and `serde_json`.
- Both lesson crates declare `sqlx`, `thiserror` and `tokio` as `{ workspace = true }` in a `[dependencies]` section after `[lints]`, and each carries `build.rs` plus its own copy of `migrations/`.
- Workspace lints deny `clippy::all` + `clippy::pedantic`, and `unused` is denied (an unused variable is a compile error, which is why the stub's parameters are `_`-prefixed). No `#[allow]` attributes anywhere. No `#[must_use]` anywhere (every function returns a `Result` or a `Future`, both already `#[must_use]`).
- The lesson uses the runtime query API only. `sqlx::query!` / `query_as!` must not appear in either crate: they need `DATABASE_URL` at build time and would break a fresh clone.
- Compile-fail files are std-only (the tool runs bare `rustc` with no `--extern`, so it cannot resolve `sqlx`).
- Out of scope — do not add: Postgres/MySQL beyond a placeholder-syntax mention, pool tuning, isolation levels, savepoints, `FOR UPDATE SKIP LOCKED`, indexes, query plans, ORMs, JSON columns/serde, `sqlx-cli`, a file-backed database.
- All code in this plan was verified on rustc 1.98.1 against sqlx 0.8.6: solutions 8/8 tests pass, the stub compiles with all 8 tests panicking, and both are clippy- and rustfmt-clean. If clippy or rustfmt fires, do NOT add an `#[allow]` and do NOT change the code; STOP and report the exact output.

## Deviations from the spec

Each was verified empirically (sqlx 0.8.6, rustc 1.98.1) while writing this plan:

- **On a firing lint, STOP instead of fixing.** The spec says to fix the code and report (and to box `TransferError` if `clippy::result_large_err` ever fires). This plan's code was verified clippy- and rustfmt-clean, so a firing lint means toolchain or dependency drift worth reporting, not patching.
- **Slide 6 gains a code block** the spec's prose-only slide did not have. It shows two separate `sqlx::query(...)` bindings rather than one reused binding, because `Query::execute` and `Query::fetch_one` both take `self`.
- **The README's Compile-fail subsection is prose only** (as in Lessons 22 and 23), so the README carries 8 code blocks, not one per `###`.
- **Slides 3, 4 and 9 carry more detail than the spec's outline**: slide 3 names the two-second timeout and `PoolTimedOut`, slide 4 names the `CHECK (balance <= 1000)` the exercise leans on, and slide 9 names both rollback cases. The spec's arc is an outline; these fill it in without adding scope.

---

## Task 1: Raise the workspace toolchain to 1.98

**Files:**
- Modify: `rust-toolchain.toml`
- Modify: `Cargo.toml` (root — `[workspace.package] rust-version`)
- Modify: `clippy.toml` (`msrv`)
- Modify: `deploy/Dockerfile` (builder image tag)
- Modify: `lessons/01-hello-rust/README.md` (the version students check)

**Interfaces:**
- Consumes: nothing.
- Produces: a toolchain that can build `sqlx` (Task 3 onward).

Why: `sqlx-core` depends on `url` → `idna` → `icu_*` → `yoke 0.8.3`, whose proc-macro crate `yoke-derive 0.8.3` calls `str::from_utf8` (stabilized after 1.85) and publishes **no `rust-version`**, so cargo's MSRV-aware resolver cannot filter it out. On 1.85 a fresh clone fails with `error[E0599]: no function or associated item named 'from_utf8' found for type 'str'`. Pinning would not help: `Cargo.lock` is gitignored here. Verified during planning: on 1.98 a fresh resolution takes `yoke-derive 0.8.3` and builds in ~19s, and the existing workspace is clippy-, fmt- and test-clean on 1.98.1 with no source changes.

- [ ] **Step 1: Edit `rust-toolchain.toml`**

Change the channel line only, so the file reads:

```toml
[toolchain]
channel = "1.98"
components = ["rustfmt", "clippy", "rust-src"]
profile = "default"
```

- [ ] **Step 2: Edit the root `Cargo.toml`**

In `[workspace.package]`, change `rust-version = "1.85"` to `rust-version = "1.98"`. Change nothing else.

- [ ] **Step 3: Edit `clippy.toml`**

Change `msrv = "1.85"` to `msrv = "1.98"`. Without this, clippy warns `the MSRV in \`clippy.toml\` and \`Cargo.toml\` differ; using \`1.85.0\` from \`clippy.toml\`` on every crate in the workspace, and keeps gating its MSRV-sensitive lints on 1.85. The warning does not fail `-D warnings` (clippy raises it outside the lint machinery), so it would rot silently.

- [ ] **Step 4: Edit `deploy/Dockerfile`**

Change line 4 from `FROM rust:1.85-slim-bookworm AS builder` to `FROM rust:1.98-slim-bookworm AS builder`. `tools/build-index` inherits the workspace `rust-version`, so the old builder image would refuse to compile it (`error: rustc 1.85.1 is not supported by the following packages: build-index@0.1.0 requires rustc 1.98`). Change nothing else in the file.

If `docker` is available, confirm the new base image exists and still builds the tool — this is the Deploy path, and otherwise nothing exercises it until the push:

```bash
docker build -f deploy/Dockerfile -t rust-training-slides:l24check .
```

Expected: the `rust:1.98-slim-bookworm` pull succeeds and `cargo build --release --package build-index` completes. If docker is not installed, skip it and say so in the report.

- [ ] **Step 5: Edit `lessons/01-hello-rust/README.md`**

Change ``You should see version `1.85` or newer.`` to ``You should see version `1.98` or newer.`` — it is the only lesson text naming a toolchain version, and after this task it would otherwise contradict the pin.

- [ ] **Step 6: Confirm the toolchain is active**

```bash
rustc --version
```

Expected: `rustc 1.98.x`. (rustup installs it on first use; this may take a minute.)

- [ ] **Step 7: Run the full CI sequence on the existing lessons**

```bash
make ci
```

Expected: exit 0 — clippy clean, fmt clean, all existing tests pass, compile-fail checks pass. Verified during planning on 1.98.1: lessons 01-23 need no source changes. Two specific things to check in the output: no line containing `the MSRV in` (that would mean Step 3 was missed), and no new clippy lint. If clippy reports anything new under the newer lint set, STOP and report the exact output rather than editing lessons.

- [ ] **Step 8: Commit**

```bash
git add rust-toolchain.toml Cargo.toml clippy.toml deploy/Dockerfile lessons/01-hello-rust/README.md
git commit -m "build: raise the workspace toolchain to 1.98"
```

---

## Task 2: Scaffold lessons/24-persistence

**Files (all created by the scaffolder):**
- `lessons/24-persistence/README.md` (placeholder, replaced in Task 6)
- `lessons/24-persistence/slides/index.html` (final — no edit needed)
- `lessons/24-persistence/slides/slides.md` (placeholder, replaced in Task 7)
- `lessons/24-persistence/exercises/Cargo.toml` (dependencies added in Task 3)
- `lessons/24-persistence/exercises/src/lib.rs` (placeholder, replaced in Task 4)
- `lessons/24-persistence/exercises/tests/exercise.rs` (placeholder, replaced in Task 4)
- `lessons/24-persistence/solutions/Cargo.toml` (dependencies added in Task 3)
- `lessons/24-persistence/solutions/src/lib.rs` (placeholder, replaced in Task 5)
- `lessons/24-persistence/solutions/tests/exercise.rs` (placeholder, replaced in Task 5)

**Interfaces:**
- Consumes: the 1.98 toolchain from Task 1.
- Produces: workspace members `persistence-exercises` and `persistence-solutions`.

- [ ] **Step 1: Run the scaffolder**

```bash
make new-lesson NAME=24-persistence
```

Expected: `scaffolded ./lessons/24-persistence`.

- [ ] **Step 2: Verify directory structure**

```bash
ls lessons/24-persistence/
ls lessons/24-persistence/slides/ lessons/24-persistence/exercises/ lessons/24-persistence/solutions/
```

Expected: top-level `README.md`, `slides/`, `exercises/`, `solutions/` populated from templates.

- [ ] **Step 3: Verify Cargo package names**

```bash
grep '^name' lessons/24-persistence/exercises/Cargo.toml lessons/24-persistence/solutions/Cargo.toml
```

Expected:
```
lessons/24-persistence/exercises/Cargo.toml:name = "persistence-exercises"
lessons/24-persistence/solutions/Cargo.toml:name = "persistence-solutions"
```

- [ ] **Step 4: Verify the workspace picks up the new crates**

```bash
cargo metadata --no-deps --format-version 1 | grep -o '"name":"persistence-[^"]*"' | sort -u
```

Expected output:
```
"name":"persistence-exercises"
"name":"persistence-solutions"
```

- [ ] **Step 5: Verify the scaffolded workspace builds clean**

```bash
cargo build --workspace
```

Expected: warning-free build.

- [ ] **Step 6: Commit**

```bash
git add lessons/24-persistence
git commit -m "chore: scaffold lessons/24-persistence"
```

---

## Task 3: Add the `sqlx` workspace dependency and wire the lesson crates

**Files:**
- Modify: `Cargo.toml` (root — add `sqlx` to `[workspace.dependencies]`)
- Modify: `lessons/24-persistence/exercises/Cargo.toml`
- Modify: `lessons/24-persistence/solutions/Cargo.toml`

**Interfaces:**
- Consumes: the two scaffolded manifests from Task 2.
- Produces: `sqlx` 0.8.x, `thiserror` and `tokio` available to both lesson crates (Tasks 4 and 5).

Note: `thiserror = "2"` and `tokio = { version = "1", features = ["rt", "macros"] }` are ALREADY workspace dependencies. Only `sqlx` is new at the root.

- [ ] **Step 1: Add `sqlx` to the root `[workspace.dependencies]`**

Insert the `sqlx` line after `clap`, so the table reads:

```toml
[workspace.dependencies]
anyhow = "1"
axum = "0.8"
clap = { version = "4", features = ["derive"] }
sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio", "sqlite", "migrate", "macros"] }
thiserror = "2"
tokio = { version = "1", features = ["rt", "macros"] }
toml_edit = "0.22"
walkdir = "2"
tempfile = "3"
tiny_http = "0.12"
```

Change nothing else in the root `Cargo.toml`.

- [ ] **Step 2: Add the dependencies to `lessons/24-persistence/exercises/Cargo.toml`**

Append a `[dependencies]` section after `[lints]`, so the whole file reads:

```toml
[package]
name = "persistence-exercises"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true

[lints]
workspace = true

[dependencies]
sqlx = { workspace = true }
thiserror = { workspace = true }
tokio = { workspace = true }
```

- [ ] **Step 3: Add the dependencies to `lessons/24-persistence/solutions/Cargo.toml`**

Same change for the solutions crate:

```toml
[package]
name = "persistence-solutions"
version = "0.1.0"
edition.workspace = true
rust-version.workspace = true
license.workspace = true
publish.workspace = true

[lints]
workspace = true

[dependencies]
sqlx = { workspace = true }
thiserror = { workspace = true }
tokio = { workspace = true }
```

- [ ] **Step 4: Build to confirm `sqlx` resolves and compiles**

```bash
cargo build --workspace
```

Expected: cargo locks ~130 new packages (129 when measured) and compiles 115 crates — including `libsqlite3-sys`, which builds SQLite from C source — then a warning-free build. Measured during planning: 16-20s on a developer machine with a warm crates.io cache; budget a minute or two on a cold cache or a slower runner. A C compiler must be available — if the build fails with `failed to run custom build command for 'libsqlite3-sys'`, report that rather than working around it.

- [ ] **Step 5: Verify the resolved versions**

```bash
cargo tree --package persistence-solutions --depth 1
```

Expected: lists `sqlx v0.8.x`, `thiserror v2.x` and `tokio v1.x`.

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml lessons/24-persistence/exercises/Cargo.toml lessons/24-persistence/solutions/Cargo.toml
git commit -m "build(lesson-24): add sqlx workspace dependency and wire the lesson crates"
```

---

## Task 4: Exercise content (migrations, build.rs, stub, tests, compile-fail)

**Files:**
- Create: `lessons/24-persistence/exercises/build.rs`
- Create: `lessons/24-persistence/exercises/migrations/0001_create_accounts.sql`
- Create: `lessons/24-persistence/exercises/migrations/0002_add_balance.sql`
- Overwrite: `lessons/24-persistence/exercises/src/lib.rs`
- Overwrite: `lessons/24-persistence/exercises/tests/exercise.rs`
- Create: `lessons/24-persistence/exercises/compile_fails/24-tx-consumed.rs`

**Interfaces:**
- Consumes: `sqlx` + `thiserror` + `tokio` from Task 3.
- Produces (the public API of `persistence_exercises`, mirrored exactly by `persistence_solutions` in Task 5):
  - `#[derive(Debug, Clone, PartialEq, Eq, sqlx::FromRow)] pub struct Account { pub id: i64, pub name: String, pub balance: i64 }`
  - `pub enum TransferError { Database(#[from] sqlx::Error), InsufficientFunds { id: i64, balance: i64, amount: i64 } }`
  - `pub async fn connect() -> Result<SqlitePool, sqlx::Error>`
  - `pub async fn all_accounts(pool: &SqlitePool) -> Result<Vec<Account>, sqlx::Error>`
  - `pub async fn insert_account(pool: &SqlitePool, name: &str, balance: i64) -> Result<i64, sqlx::Error>`
  - `pub async fn transfer(pool: &SqlitePool, from: i64, to: i64, amount: i64) -> Result<(), TransferError>`

- [ ] **Step 1: Create `lessons/24-persistence/exercises/build.rs`**

```rust
fn main() {
    println!("cargo:rerun-if-changed=migrations");
}
```

`migrate!()` emits an `include_str!` per migration, so *edits* to an existing `.sql` file already trigger a rebuild without this file — verified during planning. What `build.rs` adds is the directory: without it a newly *added* migration is not noticed and the embedded set goes stale.

- [ ] **Step 2: Create the two migration files**

`lessons/24-persistence/exercises/migrations/0001_create_accounts.sql`:

```sql
CREATE TABLE accounts (
    id   INTEGER PRIMARY KEY,
    name TEXT    NOT NULL
);
```

`lessons/24-persistence/exercises/migrations/0002_add_balance.sql`:

```sql
-- The 1000 cap is a teaching device: a rule the database enforces on
-- every write, so a transfer can fail on its SECOND update, after the
-- first has already moved money. That is why `transfer` needs a
-- transaction. Migrations are append-only: to change this, add 0003.
ALTER TABLE accounts ADD COLUMN balance INTEGER NOT NULL DEFAULT 0 CHECK (balance <= 1000);
```

The `CHECK` is load-bearing pedagogy, not decoration: it lets one of the two UPDATEs fail after the other has already moved money, which is what the two rollback tests exploit. `main_failed_credit_rolls_back_the_debit` catches a debit-first implementation with no transaction (`left: 90, right: 100`), and `main_rejected_write_leaves_both_accounts_untouched` catches a credit-first one via a negative amount (`left: -1950, right: 50`). Both messages quote the database's own `CHECK constraint failed: balance <= 1000`. What the tests CANNOT do is prove a transaction was used: an implementation that reads both balances first and re-checks the database's rules in Rust passes all 8 — verified during planning. The README says so plainly and argues the case on staleness and races rather than pretending the tests force it.

- [ ] **Step 3: Overwrite `lessons/24-persistence/exercises/src/lib.rs`**

Write EXACTLY as shown:

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
/// The pool is capped at one connection: SQLite takes one writer at a
/// time, and the cap turns a statement sent to the pool during an open
/// transaction into a quick `PoolTimedOut` instead of a hang.
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

// The `_` prefixes keep the unfinished stubs compiling (unused variables
// are errors in this course). Rename them when you write the body.
pub async fn insert_account(
    _pool: &SqlitePool,
    _name: &str,
    _balance: i64,
) -> Result<i64, sqlx::Error> {
    todo!("INSERT the account, then return its new row id")
}

// `transfer` doesn't reject odd values of `amount` up front: a negative
// amount is a legal call. It checks the sender's balance itself, and
// leaves the rest to the database's CHECK — either way, a transfer the
// database rejects must leave both balances unchanged.
pub async fn transfer(
    _pool: &SqlitePool,
    _from: i64,
    _to: i64,
    _amount: i64,
) -> Result<(), TransferError> {
    todo!("move the money inside one transaction; roll back an overdraft")
}
```

- [ ] **Step 4: Overwrite `lessons/24-persistence/exercises/tests/exercise.rs`**

```rust
use persistence_exercises::{
    Account, TransferError, all_accounts, connect, insert_account, transfer,
};

// Warm-up: insert_account

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

// Main: transfer

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
    assert_eq!(
        rows[0].balance, 70,
        "the sender should have been debited - if nothing moved, did you tx.commit()?"
    );
    assert_eq!(
        rows[1].balance, 80,
        "the receiver should have been credited"
    );
}

#[tokio::test]
async fn main_overdraft_is_rejected_and_writes_nothing() {
    let pool = seeded().await;
    let err = transfer(&pool, 1, 2, 500).await.unwrap_err();
    match err {
        TransferError::InsufficientFunds {
            id,
            balance,
            amount,
        } => assert_eq!(
            (id, balance, amount),
            (1, 100, 500),
            "report the sender's balance BEFORE the transfer"
        ),
        other @ TransferError::Database(_) => panic!("expected InsufficientFunds, got {other:?}"),
    }
    let rows = all_accounts(&pool).await.unwrap();
    assert_eq!(rows[0].balance, 100, "the sender must be untouched");
    assert_eq!(rows[1].balance, 50, "the receiver must be untouched too");
}

#[tokio::test]
async fn main_failed_credit_rolls_back_the_debit() {
    let pool = connect().await.unwrap();
    insert_account(&pool, "alice", 100).await.unwrap();
    // "vault" sits at the 1000 cap from migration 0002, so crediting it
    // anything fails - after the sender has already been debited.
    insert_account(&pool, "vault", 1000).await.unwrap();
    let err = transfer(&pool, 1, 2, 10).await.unwrap_err();
    let rows = all_accounts(&pool).await.unwrap();
    assert_eq!(
        rows[0].balance, 100,
        "the credit failed with `{err}` - the debit must be rolled back too"
    );
    assert_eq!(rows[1].balance, 1000);
}

#[tokio::test]
async fn main_rejected_write_leaves_both_accounts_untouched() {
    let pool = seeded().await;
    // A negative amount makes the SENDER's update the one the cap rejects
    // (100 - -2000 = 2100). Debit first and it fails before anything moved;
    // credit first and the receiver is already at -1950 when it fails.
    let err = transfer(&pool, 1, 2, -2000).await.unwrap_err();
    let rows = all_accounts(&pool).await.unwrap();
    assert_eq!(rows[0].balance, 100, "the sender must be untouched");
    assert_eq!(
        rows[1].balance, 50,
        "the transfer failed with `{err}` - the receiver must be untouched too"
    );
}
```

- [ ] **Step 5: Create `lessons/24-persistence/exercises/compile_fails/24-tx-consumed.rs`**

The `compile_fails/` directory does not exist yet — create it. This file is self-contained and std-only (it must NOT use `sqlx`).

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

- [ ] **Step 6: Verify exercise tests compile and fail with `todo!()` panics (intentional)**

```bash
cargo test --manifest-path lessons/24-persistence/exercises/Cargo.toml
```

Expected: the crate COMPILES, then ALL 8 tests FAIL with a `not yet implemented` panic. Result line: `test result: FAILED. 0 passed; 8 failed`. This is the correct undone state.

- [ ] **Step 7: Verify the exercises crate builds cleanly**

```bash
cargo build --package persistence-exercises
```

Expected: warning-free build.

- [ ] **Step 8: Verify compile-fail ships broken**

```bash
cargo run --package compile-fails -- --expect broken lessons/24-persistence
```

Expected: prints `ok:   lessons/24-persistence/exercises/compile_fails/24-tx-consumed.rs` and exits 0. (The tool printing the rustc E0382 error text is expected — what matters is the final `ok:` line and exit 0.)

- [ ] **Step 9: Verify compile-fail's student-mode check fires**

```bash
cargo run --package compile-fails -- --expect compiles lessons/24-persistence
```

Expected: non-zero exit with a `FAIL: ...` message naming the file. (Correct — it ships broken on purpose.)

- [ ] **Step 10: Verify lint passes on the exercises crate**

```bash
cargo clippy --package persistence-exercises --all-targets -- -D warnings
cargo fmt --check --package persistence-exercises
```

Expected: both exit 0.

- [ ] **Step 11: Commit**

```bash
git add lessons/24-persistence/exercises
git commit -m "feat(lesson-24): add migrations, exercise stubs, tests, and compile-fail"
```

---

## Task 5: Reference solutions

**Files:**
- Create: `lessons/24-persistence/solutions/build.rs`
- Create: `lessons/24-persistence/solutions/migrations/0001_create_accounts.sql`
- Create: `lessons/24-persistence/solutions/migrations/0002_add_balance.sql`
- Overwrite: `lessons/24-persistence/solutions/src/lib.rs`
- Overwrite: `lessons/24-persistence/solutions/tests/exercise.rs`

**Interfaces:**
- Consumes: `sqlx` + `thiserror` + `tokio` from Task 3.
- Produces: `persistence_solutions` with the identical public API listed in Task 4, with real bodies for `insert_account` and `transfer`.

`sqlx::migrate!()` resolves relative to `$CARGO_MANIFEST_DIR`, so the solutions crate needs its own copy of `build.rs` and `migrations/` — byte-identical to the exercises copies.

- [ ] **Step 1: Create `lessons/24-persistence/solutions/build.rs`**

```rust
fn main() {
    println!("cargo:rerun-if-changed=migrations");
}
```

- [ ] **Step 2: Create the two migration files**

`lessons/24-persistence/solutions/migrations/0001_create_accounts.sql`:

```sql
CREATE TABLE accounts (
    id   INTEGER PRIMARY KEY,
    name TEXT    NOT NULL
);
```

`lessons/24-persistence/solutions/migrations/0002_add_balance.sql`:

```sql
-- The 1000 cap is a teaching device: a rule the database enforces on
-- every write, so a transfer can fail on its SECOND update, after the
-- first has already moved money. That is why `transfer` needs a
-- transaction. Migrations are append-only: to change this, add 0003.
ALTER TABLE accounts ADD COLUMN balance INTEGER NOT NULL DEFAULT 0 CHECK (balance <= 1000);
```

The `CHECK` is load-bearing pedagogy, not decoration: it lets one of the two UPDATEs fail after the other has already moved money, which is what the two rollback tests exploit. `main_failed_credit_rolls_back_the_debit` catches a debit-first implementation with no transaction (`left: 90, right: 100`), and `main_rejected_write_leaves_both_accounts_untouched` catches a credit-first one via a negative amount (`left: -1950, right: 50`). Both messages quote the database's own `CHECK constraint failed: balance <= 1000`. What the tests CANNOT do is prove a transaction was used: an implementation that reads both balances first and re-checks the database's rules in Rust passes all 8 — verified during planning. The README says so plainly and argues the case on staleness and races rather than pretending the tests force it.

- [ ] **Step 3: Overwrite `lessons/24-persistence/solutions/src/lib.rs`**

Write EXACTLY as shown:

```rust
//! Lesson 24 — reference solutions.

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
/// The pool is capped at one connection: SQLite takes one writer at a
/// time, and the cap turns a statement sent to the pool during an open
/// transaction into a quick `PoolTimedOut` instead of a hang.
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

/// Insert one account, return its new id.
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

// `transfer` doesn't reject odd values of `amount` up front: a negative
// amount is a legal call. It checks the sender's balance itself, and
// leaves the rest to the database's CHECK — either way, a transfer the
// database rejects must leave both balances unchanged.
/// Move `amount` from one account to another, atomically.
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

> Pedagogical notes:
> - `insert_account` is the whole write path in six lines: SQL with `?` placeholders, one `.bind(...)` per placeholder in order, `.execute(pool)`, `?` to propagate `sqlx::Error` (Lesson 14), and `last_insert_rowid()` for the new id. The values never touch the SQL string — that is the injection-safety habit.
> - `transfer` is the transaction lifecycle end to end. `&mut *tx` is the load-bearing detail: a `Transaction` derefs to a connection, which is what `.execute()` and `.fetch_one()` want. A statement sent to `&pool` is not in the transaction at all — on a bigger pool it would take its own connection and then block on SQLite's write lock (the hang the cap exists to prevent); here it cannot get a connection at all, so it waits out `acquire_timeout` and fails with `Database(PoolTimedOut)`.
> - The two rollback tests carry the lesson. `main_failed_credit_rolls_back_the_debit` fails the *credit* (the receiver sits at the 1000 cap) after the debit has run, catching a debit-first implementation without a transaction. `main_rejected_write_leaves_both_accounts_untouched` passes a negative amount, so the sender's own update is what the cap rejects — in the reference order it fails before anything moved, and in the opposite order it fails after the receiver was credited, which is the case that catches credit-first implementations. Neither test can force a transaction (a fully pre-validating implementation passes both), and the README is explicit about that rather than overclaiming.
> - The balance is read back AFTER the UPDATE, which is why the error reports `balance + amount` — the sender's balance before the transfer.
> - `thiserror`'s `#[from]` (Lesson 14) means `?` on a `sqlx::Error` becomes `TransferError::Database` for free, while the overdraft stays a distinct, matchable variant.
> - No `#[must_use]`: every function returns a `Result` or a `Future`, both already `#[must_use]`, so adding it would trip `clippy::double_must_use`.

- [ ] **Step 4: Overwrite `lessons/24-persistence/solutions/tests/exercise.rs`**

Identical to the exercises copy except the crate ident in the `use` line:

```rust
use persistence_solutions::{
    Account, TransferError, all_accounts, connect, insert_account, transfer,
};

// Warm-up: insert_account

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

// Main: transfer

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
    assert_eq!(
        rows[0].balance, 70,
        "the sender should have been debited - if nothing moved, did you tx.commit()?"
    );
    assert_eq!(
        rows[1].balance, 80,
        "the receiver should have been credited"
    );
}

#[tokio::test]
async fn main_overdraft_is_rejected_and_writes_nothing() {
    let pool = seeded().await;
    let err = transfer(&pool, 1, 2, 500).await.unwrap_err();
    match err {
        TransferError::InsufficientFunds {
            id,
            balance,
            amount,
        } => assert_eq!(
            (id, balance, amount),
            (1, 100, 500),
            "report the sender's balance BEFORE the transfer"
        ),
        other @ TransferError::Database(_) => panic!("expected InsufficientFunds, got {other:?}"),
    }
    let rows = all_accounts(&pool).await.unwrap();
    assert_eq!(rows[0].balance, 100, "the sender must be untouched");
    assert_eq!(rows[1].balance, 50, "the receiver must be untouched too");
}

#[tokio::test]
async fn main_failed_credit_rolls_back_the_debit() {
    let pool = connect().await.unwrap();
    insert_account(&pool, "alice", 100).await.unwrap();
    // "vault" sits at the 1000 cap from migration 0002, so crediting it
    // anything fails - after the sender has already been debited.
    insert_account(&pool, "vault", 1000).await.unwrap();
    let err = transfer(&pool, 1, 2, 10).await.unwrap_err();
    let rows = all_accounts(&pool).await.unwrap();
    assert_eq!(
        rows[0].balance, 100,
        "the credit failed with `{err}` - the debit must be rolled back too"
    );
    assert_eq!(rows[1].balance, 1000);
}

#[tokio::test]
async fn main_rejected_write_leaves_both_accounts_untouched() {
    let pool = seeded().await;
    // A negative amount makes the SENDER's update the one the cap rejects
    // (100 - -2000 = 2100). Debit first and it fails before anything moved;
    // credit first and the receiver is already at -1950 when it fails.
    let err = transfer(&pool, 1, 2, -2000).await.unwrap_err();
    let rows = all_accounts(&pool).await.unwrap();
    assert_eq!(rows[0].balance, 100, "the sender must be untouched");
    assert_eq!(
        rows[1].balance, 50,
        "the transfer failed with `{err}` - the receiver must be untouched too"
    );
}
```

- [ ] **Step 5: Verify the duplicated files are byte-identical**

`sqlx::migrate!()` resolves per crate, so both copies must match exactly:

```bash
diff lessons/24-persistence/exercises/build.rs lessons/24-persistence/solutions/build.rs
diff -r lessons/24-persistence/exercises/migrations lessons/24-persistence/solutions/migrations
```

Expected: both commands print nothing and exit 0.

- [ ] **Step 6: Verify solution tests pass**

```bash
cargo test --package persistence-solutions
```

Expected: `test result: ok. 8 passed`.

- [ ] **Step 7: Verify lint passes on the solutions crate**

```bash
cargo clippy --package persistence-solutions --all-targets -- -D warnings
cargo fmt --check --package persistence-solutions
```

Expected: both exit 0. The code above is exactly correct as written — do NOT modify it. If clippy or rustfmt fires, do NOT add an `#[allow]` and do NOT change the code; STOP and report the exact output.

- [ ] **Step 8: Commit**

```bash
git add lessons/24-persistence/solutions
git commit -m "feat(lesson-24): add reference solutions"
```

---

## Task 6: Lesson README

**Files:**
- Overwrite: `lessons/24-persistence/README.md`

**Interfaces:**
- Consumes: the names from Tasks 4-5.
- Produces: nothing code-facing.

- [ ] **Step 1: Overwrite `lessons/24-persistence/README.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Lesson 24` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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

`transfer` doesn't reject odd values of `amount` up front — a negative
amount is a legal call. You check the sender's balance yourself; the
database's `CHECK` catches the rest. Two tests make the
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
````

- [ ] **Step 2: Spot-check the README**

```bash
head -1 lessons/24-persistence/README.md
grep -c '^### ' lessons/24-persistence/README.md
grep -c '^```' lessons/24-persistence/README.md
```

Expected:
- First line: `# Lesson 24 — Persistence`
- `grep -c '^### '` returns `9` (five subsections under self-study + four under exercises)
- `grep -c '^```'` returns `16` (8 code blocks × 2 fence lines — the "Compile-fail" exercise subsection is prose only)

If either count is wrong, the file content is off — re-check it against the content above and fix before committing.

- [ ] **Step 3: Commit**

```bash
git add lessons/24-persistence/README.md
git commit -m "docs(lesson-24): write self-study notes"
```

---

## Task 7: Slide deck

**Files:**
- Overwrite: `lessons/24-persistence/slides/slides.md`

**Interfaces:**
- Consumes: the names from Tasks 4-5.
- Produces: nothing code-facing.

- [ ] **Step 1: Overwrite `lessons/24-persistence/slides/slides.md`**

The complete file content is below, delimited by an OUTER quadruple-backtick fence (` ```` `). That outer fence is ONLY a delimiter for this plan — do NOT write it into the file. The file must start with `# Persistence` on line 1 and contain only PLAIN triple-backtick (` ``` `) code fences.

````markdown
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
````

- [ ] **Step 2: Verify `make slides-build` succeeds and includes lesson 24**

```bash
make slides-build
test -f dist/lessons/24-persistence/slides/slides.md
test -f dist/lessons/24-persistence/slides/index.html
grep -c "24-persistence" dist/index.html
```

Expected: both files copied into dist; `grep -c "24-persistence"` returns at least 1. (The build-index registry already has lesson 24 registered with slug `persistence`, matching this directory, so it renders as a clickable link.)

- [ ] **Step 3: Spot-check slide separators**

```bash
grep -c '^---$' lessons/24-persistence/slides/slides.md
```

Expected: `9` (between 10 slides).

- [ ] **Step 4: Commit**

```bash
git add lessons/24-persistence/slides/slides.md
git commit -m "feat(lesson-24): write slide deck"
```

---

## Task 8: End-to-end verification + push

**Interfaces:**
- Consumes: everything from Tasks 1-7.
- Produces: lesson 24 live on the deployed site.

- [ ] **Step 1: `make ci` is green**

```bash
make ci
```

Expected: exit 0. Clippy clean, fmt clean, workspace builds, default-members tests pass (now including the 8 tests in `persistence-solutions`), compile-fail `--expect broken` passes for lesson 24.

- [ ] **Step 2: `make verify LESSON=24-persistence` fails (the exercise is undone — intentional)**

```bash
make verify LESSON=24-persistence || echo "expected: exercise tests fail with todo!() panic"
```

Expected: `make` reports an error from the `cargo test` recipe — `test result: FAILED. 0 passed; 8 failed`, every failure a `not yet implemented` panic — then the `expected: ...` echo line prints, so the combined command itself exits 0.

- [ ] **Step 3: `make slides-build` final state**

```bash
make slides-build
ls dist/lessons/
grep -c "24-persistence" dist/index.html
```

Expected: `dist/lessons/` contains all twenty-four lessons. `grep -c "24-persistence"` ≥ 1.

- [ ] **Step 4: Push**

```bash
git push
```

Expected: push succeeds. The CI cache key hashes `**/Cargo.toml`, which changed, so the first run refetches and rebuilds the whole dependency tree on both the stable and beta legs — a slow first run is expected, not a failure.

- [ ] **Step 5: Smoke-test the deployed site**

After the push, list the runs for the pushed commit. If fewer than two rows (`CI` and `Deploy`) appear, wait ~10 seconds and re-run it:

```bash
gh run list --commit "$(git rev-parse HEAD)" --json databaseId,workflowName,status
```

Wait for each run using the `databaseId` values from that output:

```bash
gh run watch <CI databaseId> --exit-status
gh run watch <Deploy databaseId> --exit-status
gh run list --commit "$(git rev-parse HEAD)" --json workflowName,conclusion
```

Expected: both `gh run watch` commands exit 0, and both `CI` and `Deploy` show `"conclusion":"success"`. Then:

```bash
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/
curl -sS -o /dev/null -w "%{http_code}\n" https://rust.ristkari.dev/lessons/24-persistence/slides/
```

Expected: both return `200`.

---

## Done criteria

- `rust-toolchain.toml`, root `Cargo.toml`, `clippy.toml`, `deploy/Dockerfile` and `lessons/01-hello-rust/README.md` all name 1.98; `make ci` green on lessons 01-23 with no clippy MSRV warning and no lesson source change
- `lessons/24-persistence/` exists with all four parts, plus `build.rs` and `migrations/` in both crates
- Root `Cargo.toml` `[workspace.dependencies]` includes the `sqlx` entry with `default-features = false` and exactly four features; both lesson `Cargo.toml`s declare `sqlx`, `thiserror` and `tokio` as workspace dependencies
- Cargo manifests use the package names `persistence-exercises` and `persistence-solutions`
- Both crates' `migrations/` hold the same two `.sql` files
- `exercises/src/lib.rs` and `solutions/src/lib.rs` define the same `Account`, `TransferError`, `connect`, `all_accounts`, `insert_account` and `transfer`; the exercise ships `todo!()` bodies for the last two, the solution ships real ones
- `cargo test --package persistence-solutions` → 8 passing tests
- `cargo test --manifest-path lessons/24-persistence/exercises/Cargo.toml` → compiles, 8 panicking tests (intentional)
- `cargo run --package compile-fails -- --expect broken lessons/24-persistence` → ok
- `cargo run --package compile-fails -- --expect compiles lessons/24-persistence` → fails (intentional)
- `make ci` → green
- `make slides-build` → produces `dist/lessons/24-persistence/slides/index.html`
- `dist/index.html` lists lesson 24 as a clickable link
- All changes committed and pushed (plain commit messages, no co-author trailer)
- The push triggers a green CI run (stable + beta) and a green Deploy run
- Deployed site returns HTTP 200 for `/` and `/lessons/24-persistence/slides/`
