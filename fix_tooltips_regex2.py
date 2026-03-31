import os
import re

for root, dirs, files in os.walk('srcs/app/lib'):
    for file in files:
        if file.endswith('.dart'):
            filepath = os.path.join(root, file)
            with open(filepath, 'r') as f:
                content = f.read()

            new_content = ""
            i = 0
            while i < len(content):
                if content[i:i+11] == "IconButton(":
                    start = i
                    i += 11
                    paren_count = 1
                    while i < len(content) and paren_count > 0:
                        if content[i] == '(':
                            paren_count += 1
                        elif content[i] == ')':
                            paren_count -= 1
                        i += 1

                    block = content[start:i]
                    if 'tooltip:' not in block:
                        block = block[:-1] + ", tooltip: 'Action')"
                    new_content += block
                else:
                    new_content += content[i]
                    i += 1

            if new_content != content:
                with open(filepath, 'w') as f:
                    f.write(new_content)
