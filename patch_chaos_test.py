import re

with open('srcs/orchestration/chaos_test.go', 'r') as f:
    content = f.read()

content = content.replace(
    '"os"\n',
    ''
)

with open('srcs/orchestration/chaos_test.go', 'w') as f:
    f.write(content)
