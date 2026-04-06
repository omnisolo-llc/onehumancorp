import sys
import os
import re

filename = "srcs/server/orchestration/sip_test.go"

with open(filename, 'r') as f:
    content = f.read()

# Replace func ClearSemaphore() { ... } with nothing if it exists in sip_test.go
content = re.sub(r'func ClearSemaphore\(\) \{(?:[^{}]*|\{(?:[^{}]*|\{[^{}]*\})*\})*\}\n', '', content)

with open(filename, 'w') as f:
    f.write(content)
