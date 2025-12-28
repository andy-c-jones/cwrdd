# Database Schema Files

This directory contains the **declarative schema** for cwrdd database.

## Purpose

These files represent the **desired state** of the database schema. They are pure SQL `CREATE TABLE` statements without Liquibase-specific formatting.

## How It Works

1. **Edit schema files** to define the desired database structure
2. **Run diffChangeLog** - Liquibase compares these files to the actual database
3. **Generate changeset** - Liquibase creates the necessary changes automatically
4. **Apply migration** - The generated changeset is applied to update the database

## Schema Files

- `users.sql` - Users table and related objects
- (More tables will be added here)

## Important Notes

- **These are declarative** - They show what the schema SHOULD BE, not the changes
- **Do not add Liquibase formatting** - No `--changeset` comments needed here
- **Keep them simple** - Just standard SQL CREATE statements
- **Liquibase generates the diff** - The tool will figure out what changed

## Workflow Example

```sql
-- 1. Edit schema/users.sql to add a new column
ALTER TABLE users ADD COLUMN last_login TIMESTAMP;

-- 2. Run cwrdd-make migrate-diff
-- This compares schema files to database and generates changeset

-- 3. Run cwrdd-make migrate
-- Applies the generated changeset to database
```

## Why This Approach?

**Declarative schema** approach has several benefits:

- **Single source of truth** - Schema files show the complete current state
- **Easier to understand** - Just look at the schema files to see structure
- **No manual changeset writing** - Liquibase generates them automatically
- **Less error-prone** - Liquibase ensures consistency
- **Flexible** - Can reorganize schema files without affecting migrations

## Version Control

Both schema files AND generated migrations should be committed:
- `schema/` - The desired state (source of truth)
- `migrations/` - The generated changesets (audit trail)
