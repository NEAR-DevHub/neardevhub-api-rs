#!/bin/bash
sudo apt update
sudo apt install -y postgresql
sudo apt install libudev-dev

cargo install sqlx-cli

# Start PostgreSQL service
sudo service postgresql start

# Switch to postgres user to set up roles and database
sudo su - postgres -c "
    psql -c \"CREATE DATABASE devhub_cache_api_rs\"
    psql -c \"CREATE ROLE devhub_cache_api_rs WITH LOGIN PASSWORD 'password';\"
    psql -c \"ALTER ROLE devhub_cache_api_rs CREATEDB;\"
    psql -c \"GRANT ALL PRIVILEGES ON DATABASE devhub_cache_api_rs TO devhub_cache_api_rs;\"
"

# Export database URL for SQLx
echo "export DATABASE_URL=postgres://devhub_cache_api_rs:password@127.0.0.1:5432/devhub_cache_api_rs" >> ~/.bashrc
source ~/.bashrc

# Create the database using SQLx
sqlx database create

# Run migrations
sqlx migrate run
