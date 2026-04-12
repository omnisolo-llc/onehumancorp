import sys

content = open('srcs/server/orchestration/task_orchestrator.go').read()
if "github.com/onehumancorp/mono/srcs/server/memory" not in content:
    content = content.replace('"github.com/onehumancorp/mono/srcs/server/models"', '"github.com/onehumancorp/mono/srcs/server/models"\n\t"github.com/onehumancorp/mono/srcs/server/memory"\n\t"github.com/onehumancorp/mono/srcs/server/memory/autodream"')

open('srcs/server/orchestration/task_orchestrator.go', 'w').write(content)
