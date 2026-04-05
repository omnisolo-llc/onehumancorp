with open("srcs/server/orchestration/service.go", "r") as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "func handlePollTasks" in line:
        lines.insert(i+1, '\tclaims := auth.ClaimsFromContext(r.Context())\n\tif claims == nil {\n\t\thttp.Error(w, "unauthorized", http.StatusUnauthorized)\n\t\treturn\n\t}\n')
        break

for i, line in enumerate(lines):
    if "func handleUpdateTaskStatus" in line:
        lines.insert(i+1, '\tclaims := auth.ClaimsFromContext(r.Context())\n\tif claims == nil {\n\t\thttp.Error(w, "unauthorized", http.StatusUnauthorized)\n\t\treturn\n\t}\n')
        break

# fix imports
for i, line in enumerate(lines):
    if '"github.com/onehumancorp/mono/srcs/server/db"' in line:
        lines.insert(i, '\t"github.com/onehumancorp/mono/srcs/server/auth"\n')
        break

with open("srcs/server/orchestration/service.go", "w") as f:
    f.writelines(lines)
