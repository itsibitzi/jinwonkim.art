#!/usr/bin/env bash

DB_PATH='/tmp/jinwonkim-art.db'

echo "VACUUM;" | sqlite3 "$DB_PATH"
export DATABASE_URL="sqlite://${DB_PATH}"

cargo sqlx migrate run
cargo sqlx prepare

rm "${DB_PATH}"
