SCRIPT_PATH=$( cd $(dirname $0) ; pwd -P )

DB_PATH="${SCRIPT_PATH}/../jinwonkim.db"

if [ -f "$DB_PATH" ];
then
    rm "$DB_PATH"
fi

echo "VACUUM;" | sqlite3 "$DB_PATH" 
DATABASE_URL="sqlite://${DB_PATH}" sqlx migrate run 

# Password is: password
PASSWORD='$argon2i$v=19$m=16,t=2,p=1$Y01Ybk5ydnlHYlRUaEtETw$ZnuE0j+6IFJ/9EUlbKKzgA'
echo "INSERT INTO users (username, password_hash) VALUES ('user', '$PASSWORD')" | sqlite3 "$DB_PATH" 
