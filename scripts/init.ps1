# Define the database parameters
$DB_USER = ${POSTGRES_USER:=postgres}
$DB_PASSWORD = ${POSTGRES_PASSWORD:=password}
$DB_NAME = ${POSTGRES_DB:=newsletter}
$DB_PORT = ${POSTGRES_PORT:=5432}

# Check if psql and sqlx are installed
if (!(Get-Command psql -ErrorAction SilentlyContinue)) {
    Write-Host "Error: psql is not installed."
    exit
}
if (!(Get-Command sqlx -ErrorAction SilentlyContinue)) {
    Write-Host "Error: sqlx is not installed."
    Write-Host "Use:"
    Write-Host " cargo install --version=0.5.7 sqlx-cli --no-default-features --features postgres"
    Write-Host "to install it."
    exit
}

# Allow to skip Docker if a dockerized Postgres database is already running
if (!$env:SKIP_DOCKER) {
    # Start a Docker container with Postgres
    docker run `
        -e POSTGRES_USER=$DB_USER `
        -e POSTGRES_PASSWORD=$DB_PASSWORD `
        -e POSTGRES_DB=$DB_NAME `
        -p "${DB_PORT}:5432" `
        -d postgres `
        postgres -N 1000
}

# Set the password for the Postgres user
$env:PGPASSWORD = $DB_PASSWORD

# Wait for Postgres to start
do {
    try {
        $null = psql -h "localhost" -U $DB_USER -p $DB_PORT -d "postgres" -c '\q'
        Write-Host "Postgres is up and running on port $DB_PORT - running migrations now!"
    }
    catch {
        Write-Host "Postgres is still unavailable - sleeping"
        Start-Sleep -Seconds 1
    }
}
until ($? -eq $true)

# Postgres is up and running, set the DATABASE_URL environment variable
$env:DATABASE_URL = "postgres://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}"

# Create the database and run migrations
sqlx database create
sqlx migrate run

# Migrations have been run successfully
Write-Host "Postgres has been migrated, ready to go!"