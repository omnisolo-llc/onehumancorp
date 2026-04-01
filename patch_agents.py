import re

with open('srcs/app/lib/screens/agents_screen.dart', 'r') as f:
    content = f.read()

content = re.sub(r'excludeSemantics:\s*true,\s*child:\s*Card\(', r'child: Card(', content)

with open('srcs/app/lib/screens/agents_screen.dart', 'w') as f:
    f.write(content)
