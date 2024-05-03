-- Add migration script here
CREATE TABLE IF NOT EXISTS users(
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100),
    email VARCHAR(100)
)