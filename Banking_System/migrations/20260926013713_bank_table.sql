CREATE TABLE nid(
    id_no TEXT NOT NULL UNIQUE PRIMARY KEY,
    name TEXT NOT NULL,
    age INT NOT NULL,
    addr  TEXT NOT NULL,
    father_name TEXT NOT NULL,
    mother_name TEXT NOT NULL
);

CREATE TABLE users(
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    nid_no TEXT NOT NULL UNIQUE REFERENCES nid(id_no),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);


CREATE TABLE bank_information(
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
    balance BIGINT NOT NULL DEFAULT 0 CHECK (balance >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
