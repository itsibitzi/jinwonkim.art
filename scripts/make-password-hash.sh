SALT="$RANDOM$RANDOM"
read -p  'Username: ' USERNAME
read -sp 'Password: ' PASSWORD 

printf "$PASSWORD" | argon2 $SALT -e