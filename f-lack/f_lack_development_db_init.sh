#!/bin/bash

# Create db directory
mkdir -p db

# Check if sqlx-cli is installed, install only if not present
if ! command -v sqlx &>/dev/null; then
    cargo install sqlx-cli --no-default-features --features sqlite
fi

# Remove existing database if it exists
rm -f db/flack.db

# Create fresh database
sqlx database create --database-url sqlite:db/flack.db

# Run migrations
sqlx migrate run --database-url sqlite:db/flack.db

echo "Database setup complete!"
