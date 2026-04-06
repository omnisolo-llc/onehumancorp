import re

with open("srcs/server/dashboard/server.go", "r") as f:
    content = f.read()

# Replace auth.RequireRole("system", server.handleHybridSyncMissions) with auth.RequireRole("system", api.HandleHybridSyncMissions(server.hub))
content = content.replace('auth.RequireRole("system", server.handleHybridSyncMissions)', 'auth.RequireRole("system", api.HandleHybridSyncMissions(server.hub))')

# Add "github.com/onehumancorp/mono/srcs/server/api" to imports
if '"github.com/onehumancorp/mono/srcs/server/api"' not in content:
    content = content.replace('"github.com/onehumancorp/mono/srcs/server/auth"', '"github.com/onehumancorp/mono/srcs/server/api"\n\t"github.com/onehumancorp/mono/srcs/server/auth"')

with open("srcs/server/dashboard/server.go", "w") as f:
    f.write(content)

with open("srcs/server/dashboard/BUILD.bazel", "r") as f:
    build_content = f.read()

if '"//srcs/server/api",' not in build_content:
    build_content = build_content.replace('deps = [', 'deps = [\n        "//srcs/server/api",')
    with open("srcs/server/dashboard/BUILD.bazel", "w") as f:
        f.write(build_content)
