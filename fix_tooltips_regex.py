import os
import re

for root, dirs, files in os.walk('srcs/app/lib'):
    for file in files:
        if file.endswith('.dart'):
            filepath = os.path.join(root, file)
            with open(filepath, 'r') as f:
                content = f.read()

            # Simple regex replace for IconButton without tooltip
            # It will match the entire block `IconButton( ... )`
            # and append `, tooltip: 'Action'` before the last `)` if `tooltip:` is missing.

            def add_tooltip(m):
                block = m.group(0)
                if 'tooltip:' not in block and 'IconButton.filled(' not in block:
                    return block[:-1] + ", tooltip: 'Action')"
                return block

            # A regex that matches IconButton( ... ) accounting for balanced parentheses is hard.
            # Let's match until a `)` that corresponds to IconButton.
            # Better: read character by character.

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
