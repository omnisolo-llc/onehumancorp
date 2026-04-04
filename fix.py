with open('srcs/server/orchestration/task_orchestrator.go', 'r') as f:
    c = f.read()
import re
c = re.sub(r'^\s*"fmt"\n', '\t"fmt"\n\t"os"\n', c, flags=re.MULTILINE)
with open('srcs/server/orchestration/task_orchestrator.go', 'w') as f:
    f.write(c)

with open('srcs/server/orchestration/ultraplan.go', 'r') as f:
    c = f.read()
c = c.replace('m.hub.PublishAgentNotification("system", msg)', 'm.hub.PublishAgentNotification("system", msg)') # wait the error says `m.hub.PublishAgentNotification("system", msg) (no value) used as value` because it's inside `_ =` ? No I replaced `_ = m.hub.Publish` with `m.hub.PublishAgentNotification`.
# Wait, my previous replacement might have left `_ = ` ? Let's check:
c = c.replace('_ = m.hub.PublishAgentNotification', 'm.hub.PublishAgentNotification')
with open('srcs/server/orchestration/ultraplan.go', 'w') as f:
    f.write(c)

with open('srcs/server/db/BUILD.bazel', 'r') as f:
    c = f.read()
c = c.replace('"sqlite_provider.go",', '"sqlite_provider.go",\n        "test_provider.go",')
with open('srcs/server/db/BUILD.bazel', 'w') as f:
    f.write(c)

import os
try:
    os.rename('srcs/server/db/test_provider_test.go', 'srcs/server/db/test_provider.go')
except:
    pass

with open('srcs/server/db/test_provider.go', 'r') as f:
    c = f.read()
c = c.replace('func NewTestProvider(t *testing.T)', 'func NewTestProvider(t testing.TB)')
with open('srcs/server/db/test_provider.go', 'w') as f:
    f.write(c)
