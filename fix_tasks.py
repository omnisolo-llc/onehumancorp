with open("srcs/server/orchestration/tasks.go", "r") as f:
    content = f.read()

content = content.replace("rows, err := tm.db.Query(ctx, query, organizationID)", "rows, err := tm.db.Query(ctx, query)")

with open("srcs/server/orchestration/tasks.go", "w") as f:
    f.write(content)
