with open("srcs/server/db/database.go", "r") as f:
    content = f.read()

content = content.replace("sqlStr = strings.ReplaceAll(sqlStr, \"NOW()\", \"CURRENT_TIMESTAMP\")", "sqlStr = strings.ReplaceAll(sqlStr, \"NOW()\", \"CURRENT_TIMESTAMP\")\n\t\t\tsqlStr = strings.ReplaceAll(sqlStr, \"VARCHAR(50)\", \"TEXT\")")

with open("srcs/server/db/database.go", "w") as f:
    f.write(content)
