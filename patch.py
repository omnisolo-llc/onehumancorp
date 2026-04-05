with open('srcs/server/orchestration/service.go', 'r') as f:
    data = f.read()

# Since orchestration_test imports proto, and I added it to service.go (orchestration package), I need to remove pb references or update BUILD.bazel. Let's see what is using pb. Wait, the pb types in service.go were originally there.
data = data.replace('pb "github.com/onehumancorp/mono/srcs/proto/orchestrationpb"\n', '')

with open('srcs/server/orchestration/service.go', 'w') as f:
    f.write(data)
