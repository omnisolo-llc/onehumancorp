import sys

with open('srcs/server/orchestration/BUILD.bazel', 'r') as f:
    content = f.read()

# Add tasks_db.go, tasks_store.go to srcs if not exist
if '"tasks_db.go"' not in content:
    content = content.replace('srcs = [', 'srcs = [\n        "tasks_db.go",\n        "tasks_store.go",')

# Add tasks_db_test.go, tasks_store_test.go to test srcs if not exist
if '"tasks_store_test.go"' not in content:
    content = content.replace('srcs = [', 'srcs = [\n        "tasks_store_test.go",')

# Make sure uuid is in deps
if '"@com_github_google_uuid//:uuid"' not in content:
    content = content.replace('deps = [', 'deps = [\n        "@com_github_google_uuid//:uuid",')

with open('srcs/server/orchestration/BUILD.bazel', 'w') as f:
    f.write(content)
