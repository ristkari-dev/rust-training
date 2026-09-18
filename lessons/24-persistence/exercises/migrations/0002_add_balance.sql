-- The 1000 cap is a teaching device: a rule the database enforces on
-- every write, so a transfer can fail on its SECOND update, after the
-- first has already moved money. That is why `transfer` needs a
-- transaction. Migrations are append-only: to change this, add 0003.
ALTER TABLE accounts ADD COLUMN balance INTEGER NOT NULL DEFAULT 0 CHECK (balance <= 1000);
