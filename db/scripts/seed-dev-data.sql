-- Seed data for local development
-- This script inserts test users for development purposes

-- Note: This should NOT be run in production

-- Test user 1
INSERT INTO users (
    email,
    password_hash,
    email_verified,
    username,
    display_name,
    date_of_birth,
    account_status
) VALUES (
    'alice@example.com',
    '$argon2id$v=19$m=19456,t=2,p=1$randomsalt$hashedpassword', -- Replace with actual hash
    TRUE,
    'alice',
    'Alice Smith',
    '1990-01-15',
    'active'
) ON CONFLICT (email) DO NOTHING;

-- Test user 2
INSERT INTO users (
    email,
    password_hash,
    email_verified,
    username,
    display_name,
    date_of_birth,
    account_status,
    public_bio
) VALUES (
    'bob@example.com',
    '$argon2id$v=19$m=19456,t=2,p=1$randomsalt$hashedpassword',
    TRUE,
    'bob',
    'Bob Johnson',
    '1988-06-20',
    'active',
    'Software developer interested in privacy and democracy.'
) ON CONFLICT (email) DO NOTHING;

-- Test user 3
INSERT INTO users (
    email,
    password_hash,
    email_verified,
    username,
    display_name,
    date_of_birth,
    account_status
) VALUES (
    'charlie@example.com',
    '$argon2id$v=19$m=19456,t=2,p=1$randomsalt$hashedpassword',
    FALSE,
    'charlie',
    'Charlie Brown',
    '1995-03-10',
    'active'
) ON CONFLICT (email) DO NOTHING;

SELECT 'Seed data inserted successfully' AS status;
