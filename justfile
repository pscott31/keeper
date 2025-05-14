TEST_DB_URL := "sqlite://test.db"

default:
  just --list

sqlx-create:
    @echo "Creating SQLx database at {{TEST_DB_URL}}..."
    @sqlx database create --database-url {{TEST_DB_URL}}

sqlx-migrate: sqlx-create
    @echo "Running SQLx database migration on {{TEST_DB_URL}}..."
    @sqlx migrate run --database-url {{TEST_DB_URL}}

sqlx-prepare: sqlx-migrate
    cargo sqlx prepare  --database-url {{TEST_DB_URL}} --workspace -- --features ssr