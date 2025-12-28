-- Example query scripts for development and testing

-- Get user by email
SELECT 
    id, 
    email, 
    username, 
    display_name,
    account_status,
    created_at
FROM users
WHERE email = 'user@example.com';

-- Get user with profile information
SELECT 
    id,
    email,
    username,
    display_name,
    public_profile_picture_url,
    public_bio,
    public_location,
    searchable,
    created_at
FROM users
WHERE id = 'USER_ID_HERE';

-- Count active users
SELECT 
    account_status,
    COUNT(*) as count
FROM users
GROUP BY account_status;

-- Recent user registrations
SELECT 
    username,
    display_name,
    email,
    created_at
FROM users
WHERE account_status = 'active'
ORDER BY created_at DESC
LIMIT 10;
