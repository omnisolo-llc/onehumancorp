import os
import re

directory = '.agent-task/missions/'
mission_file = '2026-04-05T12-40-00Z_kairos_orchestrator.md'

path = os.path.join(directory, mission_file)

with open(path, 'r') as f:
    content = f.read()

# Replace status: IN_PROGRESS or status: PENDING with status: DONE
content = re.sub(r'status:\s*(IN_PROGRESS|PENDING)', 'status: DONE', content)
content = re.sub(r'agent:\s*.*', 'agent: jules', content)

with open(path, 'w') as f:
    f.write(content)

print(f"Updated {mission_file} to DONE")
