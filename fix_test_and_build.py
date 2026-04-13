import os

with open("srcs/server/orchestration/tasks.go", "r") as f:
    content = f.read()

# remove unused imports
content = content.replace('\t"github.com/onehumancorp/mono/srcs/server/memory"\n\t"github.com/onehumancorp/mono/srcs/server/memory/autodream"', "")

with open("srcs/server/orchestration/tasks.go", "w") as f:
    f.write(content)

with open("srcs/server/orchestration/BUILD.bazel", "r") as f:
    content = f.read()

content = content.replace('        "//srcs/server/memory",\n        "//srcs/server/memory/autodream",\n', "")

with open("srcs/server/orchestration/BUILD.bazel", "w") as f:
    f.write(content)
