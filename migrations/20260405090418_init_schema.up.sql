CREATE TYPE user_role AS ENUM ('ADMIN', 'USER');

CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  email TEXT NOT NULL UNIQUE,
  role user_role NOT NULL DEFAULT 'USER',
  first_name TEXT NOT NULL,
  last_name TEXT NULL,
  password TEXT NOT NULL
);
