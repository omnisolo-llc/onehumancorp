# Ah, I see. `test_provider.go` in `srcs/server/db` HAS NOT BEEN imported!
# Because I removed `import "github.com/onehumancorp/mono/srcs/server/db"` from `autodream_kairos_test.go`!!
import glob

with open('srcs/server/orchestration/autodream_kairos_test.go', 'r') as f:
    content = f.read()

content = content.replace('NewTestProvider(t)', 'db.NewTestProvider(t)')
content = content.replace('import (\n\t"context"\n\t"os"\n\t"testing"\n)', 'import (\n\t"context"\n\t"os"\n\t"testing"\n\t"github.com/onehumancorp/mono/srcs/server/db"\n)')

with open('srcs/server/orchestration/autodream_kairos_test.go', 'w') as f:
    f.write(content)
