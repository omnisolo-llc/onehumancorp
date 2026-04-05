with open("srcs/server/orchestration/service.go", "r") as f:
    service = f.read()

if '"github.com/onehumancorp/mono/srcs/server/auth"' not in service:
    service = service.replace(
        '"github.com/onehumancorp/mono/srcs/server/db"',
        '"github.com/onehumancorp/mono/srcs/server/auth"\n\t"github.com/onehumancorp/mono/srcs/server/db"'
    )

with open("srcs/server/orchestration/service.go", "w") as f:
    f.write(service)

with open("srcs/server/orchestration/tasks.go", "r") as f:
    tasks = f.read()

tasks = tasks.replace(
    'rows, err := tm.db.Query(ctx, query, organizationID)',
    'rows, err := tm.db.Query(ctx, query)'
)

with open("srcs/server/orchestration/tasks.go", "w") as f:
    f.write(tasks)
