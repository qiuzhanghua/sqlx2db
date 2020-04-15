# Test connection to DB
Postgres & MySQL support

use sqlx

## Under linux:

### MySQL

```bash
DATABASE_URL="mysql://app:app@localhost:3306/app" sqlx2db
```

### PostgreSQL
```bash
DATABASE_URL="postgres://app:app@localhost:5432/app" sqlx2db
```