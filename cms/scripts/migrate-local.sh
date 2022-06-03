SCRIPT_PATH=$( cd $(dirname $0) ; pwd -P )

DATABASE_URL="sqlite://${SCRIPT_PATH}/../jinwonkim.db"

sqlx migrate run