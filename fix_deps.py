import sys

with open('srcs/server/dashboard/BUILD.bazel', 'r') as f:
    content = f.read()

# Add to deps
dep = '"//srcs/server/orchestration/mesh",'
if dep not in content:
    content = content.replace('deps = [', 'deps = [\n        ' + dep)

with open('srcs/server/dashboard/BUILD.bazel', 'w') as f:
    f.write(content)

with open('srcs/server/orchestration/BUILD.bazel', 'r') as f:
    content = f.read()

# check if mesh is a package or if we need to expose it
