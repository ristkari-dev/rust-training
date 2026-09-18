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

// `transfer` does not validate `amount`: the database's CHECK is the only
// guard. A negative amount is a legal call, and a transfer the database
// rejects must leave both balances unchanged.
pub async fn transfer(
    _pool: &SqlitePool,
    _from: i64,
    _to: i64,
    _amount: i64,
) -> Result<(), TransferError> {
    todo!("move the money inside one transaction; roll back an overdraft")
}
