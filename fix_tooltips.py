import os
import re

for root, dirs, files in os.walk('srcs/app/lib'):
    for file in files:
        if file.endswith('.dart'):
            filepath = os.path.join(root, file)
            with open(filepath, 'r') as f:
                content = f.read()

            # Find IconButton without tooltip
            lines = content.split('\n')
            for i, line in enumerate(lines):
                if 'IconButton(' in line:
                    has_tooltip = False
                    for j in range(i, min(i+10, len(lines))):
                        if 'tooltip:' in lines[j]:
                            has_tooltip = True
                            break
                        if ');' in lines[j] or '),' in lines[j] or '] )' in lines[j] or '])' in lines[j] or ')' in lines[j]:
                            # maybe the end of the constructor
                            pass

                    if not has_tooltip:
                        print(f"Possible missing tooltip in {filepath}:{i+1}")
