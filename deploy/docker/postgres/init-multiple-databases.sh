#!/bin/bash
# Creates additional PostgreSQL databases listed in POSTGRES_MULTIPLE_DATABASES
# (comma-separated). The main database from POSTGRES_DB is already created by
# the official postgres entrypoint.
set -e

function create_database() {
    local database="$1"
    echo "Creating database: $database"
    psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" <<-EOSQL
        CREATE DATABASE "$database";
        GRANT ALL PRIVILEGES ON DATABASE "$database" TO "$POSTGRES_USER";
EOSQL
}

if [ -n "$POSTGRES_MULTIPLE_DATABASES" ]; then
    for db in $(echo "$POSTGRES_MULTIPLE_DATABASES" | tr ',' ' '); do
        if [ "$db" != "$POSTGRES_DB" ]; then
            create_database "$db"
        fi
    done
fi

# Create publication and roles for PowerSync
PS_USER="${POWERSYNC_DB_USER:-powersync}"
PS_PASS="${POWERSYNC_DB_PASSWORD:-powersync}"

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
    CREATE PUBLICATION powersync FOR ALL TABLES;
    -- "Create a PowerSync user and granting necessary replication roles"
    CREATE ROLE $PS_USER WITH LOGIN PASSWORD '$PS_PASS' REPLICATION;
    GRANT ALL PRIVILEGES ON DATABASE "$POSTGRES_DB" TO $PS_USER;
    -- PowerSync needs schema permissions
    GRANT ALL PRIVILEGES ON SCHEMA public TO $PS_USER;
    GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO $PS_USER;
    ALTER DEFAULT PRIVILEGES IN SCHEMA public GRANT ALL PRIVILEGES ON TABLES TO $PS_USER;
EOSQL
