import sys

with open('srcs/server/orchestration/mesh/BUILD.bazel', 'r') as f:
    content = f.read()

# Delete local_mesh.go, mesh.go, redis_mesh.go
import os
try:
    os.remove('srcs/server/orchestration/mesh/local_mesh.go')
    os.remove('srcs/server/orchestration/mesh/mesh.go')
    os.remove('srcs/server/orchestration/mesh/redis_mesh.go')
except:
    pass

# Remove old mesh_test.go
try:
    os.remove('srcs/server/orchestration/mesh/mesh_test.go')
except:
    pass
