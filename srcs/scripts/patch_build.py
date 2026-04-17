import re

with open('srcs/server/db/BUILD.bazel', 'r') as f:
    content = f.read()

if "20260416040000_shared_tasks_v2.sql" not in content:
    content = content.replace('"migrations/010_ultraplan.sql",', '"migrations/010_ultraplan.sql",\n        "migrations/20260416040000_shared_tasks_v2.sql",')

with open('srcs/server/db/BUILD.bazel', 'w') as f:
    f.write(content)
