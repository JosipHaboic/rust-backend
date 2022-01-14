# Service Docs

## Database

### Initialize database

#### Windows

``` cmd
rem initDB.bat
SET DATABASE_URL=sqlite://./sqlite/db/main.db
sqlx db create
sqlx migrate run
```

#### Unix

``` bash
# initDB.sh
export DATABASE_URL=sqlite://./sqlite/db/main.db
sqlx db create
sqlx migrate run
```
