import re

with open('srcs/server/sync/BUILD.bazel', 'r') as f:
    content = f.read()

# Add orchestration to deps if not present
if '"//srcs/server/orchestration"' not in content:
    content = content.replace('        "//srcs/server/db",\n', '        "//srcs/server/db",\n        "//srcs/server/orchestration",\n')

with open('srcs/server/sync/BUILD.bazel', 'w') as f:
    f.write(content)
