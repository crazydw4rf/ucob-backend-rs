CREATE TYPE user_role AS ENUM ('Admin', 'User');
CREATE TYPE transaction_status AS ENUM ('Pending', 'Verified', 'Rejected', 'Accepted');
CREATE TYPE oil_price_type AS ENUM ('BUY', 'SELL');

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

CREATE TABLE oil_purchases (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  oil_volume REAL NOT NULL,
  delivery_address TEXT NOT NULL,
  status transaction_status NOT NULL DEFAULT 'Pending',
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),

  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE oil_sales (
  id SERIAL PRIMARY KEY,
  user_id INTEGER NOT NULL,
  oil_volume REAL NOT NULL,
  pickup_address TEXT NOT NULL,
  status transaction_status NOT NULL DEFAULT 'Pending',
  created_at TIMESTAMP DEFAULT NOW(),
  updated_at TIMESTAMP DEFAULT NOW(),

  FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE purchases_payment (
  id SERIAL PRIMARY KEY
);

CREATE TABLE sales_payment (
  id SERIAL PRIMARY KEY
);

CREATE TABLE oil_prices (
  id SERIAL PRIMARY KEY,
  price_per_liter REAL NOT NULL,
  price_type oil_price_type NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
);

-- NOTE: apakah perlu di catat atau tambahkan column id transaksi untuk transaksi terkahir yang mengubah nilai stock?
CREATE TABLE oil_stocks (
  id SERIAL PRIMARY KEY,
  delta REAL NOT NULL,
  created_at TIMESTAMP DEFAULT NOW()
);
