-- Add migration script here
CREATE TABLE users (
  id UUID PRIMARY KEY DEFAULT UUIDV7(),
  email VARCHAR(255) UNIQUE NOT NULL,
  username VARCHAR(255) NOT NULL,
  password_hash TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE refresh_tokens (
  id UUID PRIMARY KEY DEFAULT UUIDV7(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  token_hash TEXT NOT NULL,
  expires_at TIMESTAMP NOT NULL,
  revoked_at TIMESTAMP,
  device_name TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW(),
  last_used_at TIMESTAMP NOT NULL
);

CREATE TYPE account_types AS ENUM (
  'Cash',
  'Bank',
  'Credit Card',
  'Savings',
  'Investment',
  'E-Wallet',
  'Crypto',
  'Loan'
);

CREATE TABLE accounts (
  id UUID PRIMARY KEY DEFAULT UUIDV7(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  type account_types NOT NULL,
  currency VARCHAR(3) NOT NULL,
  balance_cache DECIMAL(36, 18) NOT NULL DEFAULT 0.000000000000000000,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TYPE category_types AS ENUM (
  'Income',
  'Expense',
  'Transfer'
);

CREATE TABLE categories (
  id UUID PRIMARY KEY DEFAULT UUIDV7(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  type category_types NOT NULL
);

CREATE TABLE transactions (
  id UUID PRIMARY KEY DEFAULT UUIDV7(),
  user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  description TEXT NOT NULL,
  date DATE DEFAULT CURRENT_DATE,
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TYPE direction_types AS ENUM (
  'IN',
  'OUT'
);

CREATE TABLE transactions_entries (
  id UUID PRIMARY KEY DEFAULT UUIDV7(),
  transaction_id UUID NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
  account_id UUID NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
  category_id UUID REFERENCES categories(id) ON DELETE SET NULL,
  amount DECIMAL(36, 18) NOT NULL,
  direction direction_types NOT NULL
);