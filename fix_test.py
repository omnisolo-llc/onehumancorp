import re

with open('srcs/server/orchestration/tasks_store_test.go', 'r') as f:
    c = f.read()

c = c.replace('<<<<<<< HEAD\n\t"context"\n\t"github.com/onehumancorp/mono/srcs/server/auth"\n=======\n>>>>>>> origin/main\n', '\t"context"\n\t"github.com/onehumancorp/mono/srcs/server/auth"\n')

c = re.sub(r'<<<<<<< HEAD\n\n\nfunc TestDecompositionTaskStore\(t \*testing\.T\) \{.*?\n=======\n>>>>>>> origin/main\n', lambda m: m.group(0).replace('<<<<<<< HEAD\n', '').replace('\n=======\n>>>>>>> origin/main\n', '\n'), c, flags=re.DOTALL)

with open('srcs/server/orchestration/tasks_store_test.go', 'w') as f:
    f.write(c)
