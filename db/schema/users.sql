-- Users table - declarative schema
-- This file represents the desired state of the users table
-- Liquibase diffChangeLog will generate the necessary changes

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    
    -- Authentication
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    
    -- Account status
    account_status VARCHAR(50) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    
    -- Profile - Public
    username VARCHAR(100) NOT NULL UNIQUE,
    display_name VARCHAR(100) NOT NULL,
    public_profile_picture_url VARCHAR(500),
    public_bio VARCHAR(500),
    public_location VARCHAR(100),
    public_website VARCHAR(500),
    
    -- Profile - Settings
    searchable BOOLEAN NOT NULL DEFAULT TRUE,
    show_organizations BOOLEAN NOT NULL DEFAULT TRUE,
    show_join_date BOOLEAN NOT NULL DEFAULT TRUE,
    
    -- Profile - Private (for contacts)
    private_display_name VARCHAR(100),
    private_profile_picture_url VARCHAR(500),
    email_visible_to_contacts BOOLEAN NOT NULL DEFAULT FALSE,
    phone_visible_to_contacts BOOLEAN NOT NULL DEFAULT FALSE,
    private_bio TEXT,
    
    -- Personal information
    date_of_birth DATE NOT NULL,
    phone VARCHAR(50),
    
    -- Constraints
    CONSTRAINT users_email_check CHECK (email ~* '^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$'),
    CONSTRAINT users_account_status_check CHECK (account_status IN ('active', 'suspended', 'banned', 'deleted'))
);

-- Create indexes for common queries
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_created_at ON users(created_at);
CREATE INDEX idx_users_account_status ON users(account_status);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create trigger for users table
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Comments for documentation
COMMENT ON TABLE users IS 'User accounts and profiles for cwrdd';
COMMENT ON COLUMN users.id IS 'Unique user identifier (UUID)';
COMMENT ON COLUMN users.email IS 'User email address (also used as username for login)';
COMMENT ON COLUMN users.password_hash IS 'Argon2id password hash';
COMMENT ON COLUMN users.account_status IS 'Account status: active, suspended, banned, deleted';
COMMENT ON COLUMN users.date_of_birth IS 'User date of birth (for age verification)';
