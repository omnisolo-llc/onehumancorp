import os
import re

missions_dir = '.agent-task/missions/'
for filename in os.listdir(missions_dir):
    filepath = os.path.join(missions_dir, filename)
    if os.path.isfile(filepath):
        try:
            with open(filepath, 'r') as f:
                content = f.read()
                if re.search(r'^status:\s*PENDING', content, re.MULTILINE):
                    print(f"Found match in {filepath}")
        except Exception as e:
            pass
