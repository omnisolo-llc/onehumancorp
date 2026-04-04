sed -i 's/rows, err := res.RowsAffected()//g' srcs/server/db/sqlite_provider.go
sed -i 's/\treturn rows, nil/\treturn 0, nil/g' srcs/server/db/sqlite_provider.go
