import os
import re

missions_dir = '.agent-task/missions/'
for filename in os.listdir(missions_dir):
    filepath = os.path.join(missions_dir, filename)
    if os.path.isfile(filepath):
        try:
            with open(filepath, 'r') as f:
                content = f.read()
                m = re.search(r'^status:\s*(.*)', content, re.MULTILINE)
                if m:
                    print(f"{filepath} -> {m.group(1).strip()}")
        except Exception as e:
            pass
