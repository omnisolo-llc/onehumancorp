import re

with open('srcs/server/sync/autodream_sync.go', 'r') as f:
    content = f.read()

# Change `var client *orchestration.MinimaxClient` to `var client orchestration.MinimaxClient`
content = content.replace('var client *orchestration.MinimaxClient', 'var client orchestration.MinimaxClient')

with open('srcs/server/sync/autodream_sync.go', 'w') as f:
    f.write(content)
