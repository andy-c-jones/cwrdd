# Generated Migrations

This directory contains Liquibase changesets **automatically generated** by the `diffChangeLog` command.

## Important

**DO NOT MANUALLY CREATE FILES HERE**

Changesets in this directory are created automatically by:
```bash
cwrdd-make migrate-diff
```

## How It Works

1. You edit schema files in `../schema/`
2. Run `cwrdd-make migrate-diff`
3. Liquibase compares schema files to current database
4. Liquibase generates XML changeset with the necessary changes
5. Generated changeset is placed in this directory
6. You review the generated changeset
7. Run `cwrdd-make migrate` to apply

## What Gets Generated

Liquibase will generate changesets that include:

- `CREATE TABLE` statements for new tables
- `ALTER TABLE` statements for modified columns
- `CREATE INDEX` statements for new indexes
- `DROP` statements for removed objects
- Rollback instructions (automatic)

## Naming Convention

Generated files follow this pattern:
```
YYYYMMDD-HHMMSS-description.xml
```

Example:
```
20251228-143022-initial-schema.xml
20251229-091545-add-organizations-table.xml
```

## Reviewing Changes

Always review generated changesets before applying:

```bash
# Generate the diff
cwrdd-make migrate-diff

# Review what was generated
cat db/migrations/20251228-143022-initial-schema.xml

# If it looks good, apply it
cwrdd-make migrate
```

## Master Changelog

The `changelog.xml` file includes all generated changesets in order. This file is automatically updated when you generate a new changeset.

## Version Control

**Commit these files!** They provide an audit trail of all database changes over time.

Even though they're auto-generated, they should be version controlled because:
- They document the history of schema evolution
- They allow rollback to previous states
- They enable database recreation from scratch
- They show who changed what and when
