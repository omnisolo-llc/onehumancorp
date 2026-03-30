import re

with open('srcs/orchestration/sip.go', 'r') as f:
    content = f.read()

if '"strings"' not in content:
    content = content.replace('"sync"', '"strings"\n\t"sync"')
    with open('srcs/orchestration/sip.go', 'w') as f:
        f.write(content)
