-- =========================================================
-- USERS
-- =========================================================

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    email VARCHAR(255) NOT NULL UNIQUE,

    username VARCHAR(255) NOT NULL,

    password_hash TEXT NOT NULL,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- =========================================================
-- REFRESH TOKENS
-- =========================================================

CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    user_id UUID NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE,

    token_hash TEXT NOT NULL UNIQUE,

    device_name TEXT NOT NULL,

    expires_at TIMESTAMPTZ NOT NULL,

    revoked_at TIMESTAMPTZ,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    last_used_at TIMESTAMPTZ
);

CREATE INDEX idx_refresh_tokens_user_id
ON refresh_tokens(user_id);

-- =========================================================
-- ACCOUNT TYPES
-- =========================================================

CREATE TYPE account_type AS ENUM (
    'Cash',
    'Bank',
    'Credit Card',
    'Savings',
    'Investment',
    'E-Wallet',
    'Crypto',
    'Loan'
);

-- =========================================================
-- ACCOUNTS
-- =========================================================

CREATE TABLE accounts (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    user_id UUID NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE,

    name TEXT NOT NULL,

    account_type account_type NOT NULL,

    currency CHAR(3) NOT NULL,

    balance_cache NUMERIC(36,18) NOT NULL DEFAULT 0,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT uq_account_name
        UNIQUE(user_id, name)
);

CREATE INDEX idx_accounts_user_id
ON accounts(user_id);

-- =========================================================
-- CATEGORY TYPES
-- =========================================================

CREATE TYPE category_type AS ENUM (
    'Income',
    'Expense',
    'Transfer'
);

-- =========================================================
-- CATEGORIES
-- =========================================================

CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    user_id UUID NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE,

    name TEXT NOT NULL,

    category_type category_type NOT NULL,

    CONSTRAINT uq_category_name
        UNIQUE(user_id, name)
);

CREATE INDEX idx_categories_user_id
ON categories(user_id);

-- =========================================================
-- TRANSACTIONS
-- =========================================================

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    user_id UUID NOT NULL
        REFERENCES users(id)
        ON DELETE CASCADE,

    description TEXT,

    transaction_date DATE NOT NULL DEFAULT CURRENT_DATE,

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX idx_transactions_user_id
ON transactions(user_id);

CREATE INDEX idx_transactions_date
ON transactions(transaction_date);

-- =========================================================
-- DIRECTION TYPES
-- =========================================================

CREATE TYPE direction_type AS ENUM (
    'IN',
    'OUT'
);

-- =========================================================
-- TRANSACTION ENTRIES
-- =========================================================

CREATE TABLE transaction_entries (
    id UUID PRIMARY KEY DEFAULT uuidv7(),

    transaction_id UUID NOT NULL
        REFERENCES transactions(id)
        ON DELETE CASCADE,

    account_id UUID NOT NULL
        REFERENCES accounts(id)
        ON DELETE CASCADE,

    category_id UUID
        REFERENCES categories(id)
        ON DELETE SET NULL,

    amount NUMERIC(36,18) NOT NULL
        CHECK (amount > 0),

    direction direction_type NOT NULL
);

CREATE INDEX idx_transaction_entries_transaction_id
ON transaction_entries(transaction_id);

CREATE INDEX idx_transaction_entries_account_id
ON transaction_entries(account_id);

CREATE INDEX idx_transaction_entries_category_id
ON transaction_entries(category_id);