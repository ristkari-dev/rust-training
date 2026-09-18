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
