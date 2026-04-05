import os
import re

for filename in os.listdir('.agent-task/missions/'):
    if filename.endswith('_standalone_metric_buffer.md'):
        filepath = os.path.join('.agent-task/missions/', filename)
        with open(filepath, 'r') as f:
            content = f.read()

        content = re.sub(r'status: PENDING', 'status: DONE', content)

        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Updated {filepath}")
