# The tests fail because `ag["name"] == "Claude SWE"` but the name in JSON was corrupted?
# Let's check `handlers_agent.go` where `hire` endpoint handles it.

import os

with open('srcs/dashboard/handlers_agent.go', 'r') as f:
    content = f.read()

print("Handlers Agent Go Code:")
print(content[:500])
