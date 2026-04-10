import os
import re

MATRIX_PATTERN = re.compile(
    r'(ColorFilter\.matrix\((?:const\s+)?<double>\[\n)'
    r'(?:[ \t]*2\.168,[ \t]*-0\.153,[ \t]*-0\.015,[ \t]*0,[ \t]*0,[ \n\t]*)'
    r'(?:[ \t]*-0\.046,[ \t]*2\.061,[ \t]*-0\.015,[ \t]*0,[ \t]*0,[ \n\t]*)'
    r'(?:[ \t]*-0\.046,[ \t]*-0\.152,[ \t]*2\.198,[ \t]*0,[ \t]*0,[ \n\t]*)'
    r'(?:[ \t]*0,[ \t]*0,[ \t]*0,[ \t]*1,[ \t]*0,?\n?)'
    r'([ \t]*\]\))',
    re.MULTILINE
)

FORMATTED_MATRIX = r'\1                  2.168, -0.153, -0.015, 0, 0,\n                  -0.046, 2.061, -0.015, 0, 0,\n                  -0.046, -0.152, 2.198, 0, 0,\n                  0, 0, 0, 1, 0,\n\2'

count = 0
for root, dirs, files in os.walk('srcs/app/lib'):
    for f in files:
        if f.endswith('.dart'):
            path = os.path.join(root, f)
            with open(path, 'r') as file:
                content = file.read()

            if MATRIX_PATTERN.search(content):
                new_content = MATRIX_PATTERN.sub(FORMATTED_MATRIX, content)
                with open(path, 'w') as file:
                    file.write(new_content)
                count += 1
                print(f"Formatted {path}")

print(f"Total formatted: {count}")
