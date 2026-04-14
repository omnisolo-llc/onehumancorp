with open('srcs/server/orchestration/tasks_store.go', 'r') as f:
    c = f.read()

c = c.replace('<<<<<<< HEAD\n\t"sync"\n\t"errors"\n\t"fmt"\n\t"github.com/onehumancorp/mono/srcs/server/auth"\n=======\n>>>>>>> origin/main\n', '\t"sync"\n\t"errors"\n\t"fmt"\n\t"github.com/onehumancorp/mono/srcs/server/auth"\n')

import re
c = re.sub(r'<<<<<<< HEAD\n\n\n\ntype DecompositionTaskStore interface \{.*?\n=======\n>>>>>>> origin/main\n', lambda m: m.group(0).replace('<<<<<<< HEAD\n', '').replace('\n=======\n>>>>>>> origin/main\n', '\n'), c, flags=re.DOTALL)

with open('srcs/server/orchestration/tasks_store.go', 'w') as f:
    f.write(c)
