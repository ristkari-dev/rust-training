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

// `transfer` does not validate `amount`: the database's CHECK is the only
// guard. A negative amount is a legal call, and a transfer the database
// rejects must leave both balances unchanged.
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
