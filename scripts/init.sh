#!/usr/bin/env bash


# visit https://gist.github.com/vncsna/64825d5609c146e80de8b1fd623011ca to understant "set -eo"

# This script sets up a Postgres database and runs migrations using sqlx-cli

# Exit immediately if a command exits with a non-zero status
set -eo pipefail

# Check if psql and sqlx are installed
if ! [ -x "$(command -v psql)" ]; then
    echo >&2 "Error: psql is not installed."
    exit 1
fi
if ! [ -x "$(command -v sqlx)" ]; then
    echo >&2 "Error: sqlx is not installed."
    echo >&2 "Use:"
    echo >&2 " cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
    echo >&2 "to install it."
    exit 1
fi

# Set default values for environment variables
DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="${POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"

# Allow to skip Docker if a dockerized Postgres database is already running
if [[ -z "${SKIP_DOCKER}" ]]
then
    # Start a Docker container with Postgres
    docker run \
        -e POSTGRES_USER=${DB_USER} \
        -e POSTGRES_PASSWORD=${DB_PASSWORD} \
        -e POSTGRES_DB=${DB_NAME} \
        -p "${DB_PORT}":5432 \
        -d postgres \
        postgres -N 1000
fi

# Set the password for the Postgres user
export PGPASSWORD="${DB_PASSWORD}"

# Wait for Postgres to start
until psql -h "localhost" -U "${DB_USER}" -p "${DB_PORT}" -d "postgres" -c '\q'; do
    >&2 echo "Postgres is still unavailable - sleeping"
    sleep 1
done

# Postgres is up and running, set the DATABASE_URL environment variable
>&2 echo "Postgres is up and running on port ${DB_PORT} - running migrations now!"
export DATABASE_URL=postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}

# Create the database and run migrations
sqlx database create
sqlx migrate run

# Migrations have been run successfully
>&2 echo "Postgres has been migrated, ready to go!"