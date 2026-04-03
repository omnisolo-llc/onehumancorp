import glob
import re

files = glob.glob(".agent-task/missions/*.md") + glob.glob(".agent-task/missions/*.yml")

for f in files:
    with open(f, 'r') as file:
        content = file.read()
    if 'status: PENDING' in content or 'status: OPEN' in content:
        print(f"Blocking {f}")
        content = re.sub(r'status:\s*(PENDING|OPEN)', 'status: BLOCKED\nblockers: Domain mismatch. I am a Flutter agent (Palette) and this mission requires a Go Backend Implementer.', content)
        with open(f, 'w') as file:
            file.write(content)
