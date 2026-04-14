import re

# Fix 1: Update the DB Migration script
with open('srcs/server/db/migrations/20260412_shared_tasks_decomposition.sql', 'r') as f:
    sql_content = f.read()

# Change UUID PRIMARY KEY DEFAULT gen_random_uuid() -> UUID PRIMARY KEY
# (We need to let the application handle the uuid to support sqlite properly without functions, or use TEXT PRIMARY KEY but since we're using string in Go struct anyway it handles strings. UUID PRIMARY KEY is handled by the db layer, but gen_random_uuid() is pg only.)
sql_content = sql_content.replace('id UUID PRIMARY KEY DEFAULT gen_random_uuid()', 'id VARCHAR PRIMARY KEY')
# Note: wait, in database.go there's a replacement rule:
# sqlStr = strings.ReplaceAll(sqlStr, "UUID PRIMARY KEY DEFAULT gen_random_uuid()", "TEXT PRIMARY KEY")
# If it's already exactly that string, it's replaced!
# But to be safe and let the application generate the ID (since it's not being provided in the DB), I'll make it `VARCHAR PRIMARY KEY` and modify Go code if needed, but actually the struct has `ID string`. If we just remove default, we can generate it in Go. Wait, the DB layer generates it? The `ClaimTask` and `TransitionTask` don't INSERT anything. The orchestration layer inserts it via `ArchitectAgent`.
with open('srcs/server/db/migrations/20260412_shared_tasks_decomposition.sql', 'w') as f:
    f.write(sql_content)


# Fix 2: Add //srcs/server/auth to deps of go_library in BUILD.bazel
with open('srcs/server/orchestration/BUILD.bazel', 'r') as f:
    build_content = f.read()

# I will find the `go_library(` block and add `//srcs/server/auth` in deps.
# Actually, looking at the previous output, `//srcs/server/auth` is already in `go_library` deps! Wait...
# Let me look closely. In the previous grep, `go_library` deps had:
#        "//srcs/server/scheduler",
#        "//srcs/server/auth",
#        "//srcs/server/settings",
# It seems it was already there. Let me run `git diff` or `grep` to be sure.
