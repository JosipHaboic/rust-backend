# Service Docs

## Auth service for access control

## Database

### Initialize database

#### Windows

```cmd
SET DATABASE_URL=sqlite://./sqlite/db/main.sqlite
sqlx db create
sqlx migrate run
```

#### Unix

```bash
# initDB.sh
export DATABASE_URL=sqlite://./sqlite/db/main.sqlite
sqlx db create
sqlx migrate run
```

## Tests

```bash
cargo test -- --nocapture
```
