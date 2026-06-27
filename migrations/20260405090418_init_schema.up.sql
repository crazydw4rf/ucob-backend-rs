CREATE TYPE user_role AS ENUM ('Admin', 'User');
CREATE TYPE transaction_status AS ENUM ('Unpaid','Pending','Processing','Rejected','Delivered','Done');
CREATE TYPE transaction_type AS ENUM ('Purchase', 'Sale');
CREATE TYPE payment_method AS ENUM ('QRIS', 'COD');
CREATE TYPE payment_status AS ENUM ('Pending','Completed');
CREATE TYPE price_type AS ENUM ('Buy', 'Sell');

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  username TEXT NOT NULL,
  role user_role NOT NULL DEFAULT 'User',
  password TEXT NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE address (
  id SERIAL PRIMARY KEY,
  user_id INT UNIQUE NOT NULL,
  district VARCHAR(32) NOT NULL, -- Purwokerto Utara
  village VARCHAR(32) NOT NULL, -- Purwanegara
  details TEXT NOT NULL, -- nama jalan, nomor rumah, nama gang, dll

  FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE transaction (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL,
  oil_volume REAL NOT NULL, -- liter
  price_per_liter INT NOT NULL,
  payment_method payment_method NOT NULL DEFAULT 'QRIS'::payment_method,
  status transaction_status NOT NULL DEFAULT 'Pending'::transaction_status,
  transaction_type transaction_type NOT NULL DEFAULT 'Purchase'::transaction_type,
  created_at TIMESTAMPTZ DEFAULT NOW(),

  FOREIGN KEY(user_id) REFERENCES users(id)
);

CREATE TABLE transaction_details (
  id SERIAL PRIMARY KEY,
  transaction_id INT UNIQUE NOT NULL,
  address_district VARCHAR(32) NOT NULL, -- Purwokerto Utara
  address_village VARCHAR(32) NOT NULL, -- Purwanegara
  address_details TEXT NOT NULL, -- nama jalan, nomor rumah, nama gang, dll
  sale_image_url TEXT NULL,

  created_at TIMESTAMPTZ DEFAULT NOW(),
  FOREIGN KEY(transaction_id) REFERENCES transaction(id)
);

CREATE TABLE payment (
  id SERIAL PRIMARY KEY,
  transaction_id INT UNIQUE NULL,
  amount INT NOT NULL,
  order_id VARCHAR(64) UNIQUE NOT NULL,
  status payment_status NOT NULL DEFAULT 'Pending'::payment_status,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  completed_at TIMESTAMPTZ NULL,

  FOREIGN KEY(transaction_id) REFERENCES transaction(id)
);

CREATE TABLE oil (
  delta REAL NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE oil_prices (
  price_type price_type NOT NULL DEFAULT 'Buy'::price_type,
  price_per_liter INT NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
