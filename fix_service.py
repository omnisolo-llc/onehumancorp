with open("srcs/server/orchestration/tasks.go", "r") as f:
    content = f.read()

content = content.replace("status = \\'PENDING\\'", "status = 'PENDING'")
content = content.replace("status = \\'IN_PROGRESS\\'", "status = 'IN_PROGRESS'")
content = content.replace("status IN (\\'IN_PROGRESS\\', \\'REVIEW\\')", "status IN ('IN_PROGRESS', 'REVIEW')")

with open("srcs/server/orchestration/tasks.go", "w") as f:
    f.write(content)
