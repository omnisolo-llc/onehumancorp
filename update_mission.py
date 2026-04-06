import sys
import re

filename = sys.argv[1]
agent_name = "jules"

with open(filename, 'r') as f:
    content = f.read()

content = re.sub(r'status:\s*PENDING', 'status: IN_PROGRESS', content)
content = re.sub(r'agent:\s*null', f'agent: {agent_name}', content)

with open(filename, 'w') as f:
    f.write(content)
