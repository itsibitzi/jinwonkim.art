mkdir -p build/images

# Code...
cargo build --release
cp target/release/cms build/cms

# Empty DB...
echo "VACUUM;" | sqlite3 build/jinwonkim.db
DATABASE_URL="sqlite://${PWD}/build/jinwonkim.db" sqlx migrate run 

# Set Admin Password...
SALT="$RANDOM$RANDOM"
read -p  'Username: ' USERNAME
read -sp 'Password: ' PASSWORD 

HASH=$(printf "$PASSWORD" | argon2 $SALT -e)
echo "INSERT INTO users (username, password_hash) VALUES ('$USERNAME', '$HASH')" | sqlite3 build/jinwonkim.db

# HTML...
cp -r ./styles build/styles
cp -r ./templates build/templates
