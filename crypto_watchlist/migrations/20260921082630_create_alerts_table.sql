CREATE TABLE alerts(
    id  SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    coin_id TEXT NOT NULL,
    target_price  DOUBLE PRECISION NOT NULL,
    direction TEXT NOT NULL CHECK (direction IN ('above','below')),
    triggered BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);