import re

def fix(filepath):
    with open(filepath, 'r') as f:
        content = f.read()

    # Find the library directive and move it to the top before imports
    lines = content.split('\n')
    new_lines = []

    library_line = None
    for line in lines:
        if line.strip() == "library;":
            library_line = line
            continue
        # Also let's handle the specific `library;` problem in advanced_widget_test.dart
        if line.startswith("import 'dart:io';") and library_line:
            new_lines.append(library_line)
            library_line = None
            new_lines.append(line)
        else:
            new_lines.append(line)

    # If the first line is import dart:io, we should put library; BEFORE IT.

    content = '\n'.join(lines)

    # Actually just remove "library;" from advanced_widget_test.dart since it's not strictly needed for test execution!
    content = content.replace("library;", "")

    with open(filepath, 'w') as f:
        f.write(content)

fix('srcs/app/lib/screens/advanced_widget_test.dart')
fix('srcs/app/lib/screens/widget_interactions_test.dart')
