CREATE TABLE product(
    id SERIAL PRIMARY KEY,
    content TEXT ,
    is_sold BOOLEAN NOT NULL ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);