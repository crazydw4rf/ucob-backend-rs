CREATE TYPE user_role AS ENUM ('Admin', 'User');
CREATE TYPE transaction_status AS ENUM ('Pending', 'Verified', 'Rejected', 'Accepted');
CREATE TYPE transaction_type AS ENUM ('Purchase', 'Sale');
CREATE TYPE payment_method AS ENUM ('Qris', 'Cod');
CREATE TYPE payment_status AS ENUM ('Pending','Completed');
CREATE TYPE price_type AS ENUM ('Buy', 'Sell');

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  username TEXT NOT NULL,
  role user_role NOT NULL DEFAULT 'User',
  password TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE address (
  id SERIAL PRIMARY KEY,
  user_id INT UNIQUE NOT NULL,
  district VARCHAR(32) NOT NULL, -- Purwokerto Utara
  village VARCHAR(32) NOT NULL, -- Purwanegara
  details TEXT NOT NULL, -- nama jalan, nomor rumah, nama gang, dll

  FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE payment (
  id SERIAL PRIMARY KEY,
  amount INT NOT NULL,
  order_id VARCHAR(64) UNIQUE NOT NULL,
  payment_method payment_method NOT NULL DEFAULT 'Qris'::payment_method,
  status payment_status NOT NULL DEFAULT 'Pending'::payment_status,
  completed_at TIMESTAMP NULL
);

CREATE TABLE transaction (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  payment_id INT UNIQUE NULL,
  oil_volume REAL NOT NULL,
  status transaction_status NOT NULL DEFAULT 'Pending'::transaction_status,
  type transaction_type NOT NULL DEFAULT 'Purchase'::transaction_type,
  created_at TIMESTAMP DEFAULT NOW(),

  FOREIGN KEY(user_id) REFERENCES users(id),
  FOREIGN KEY(payment_id) REFERENCES payment(id)
);

CREATE TABLE transaction_address (
  id SERIAL PRIMARY KEY,
  district VARCHAR(32) NOT NULL, -- Purwokerto Utara
  village VARCHAR(32) NOT NULL, -- Purwanegara
  details TEXT NOT NULL -- nama jalan, nomor rumah, nama gang, dll
);

CREATE TABLE oil (
  id SERIAL PRIMARY KEY,
  delta REAL NOT NULL,
  created_at TIMESTAMP NOT NULL
);

CREATE TABLE oil_prices (
  id SERIAL PRIMARY KEY,
  type price_type NOT NULL DEFAULT 'Buy'::price_type,
  price_per_liter INT NOT NULL
);

