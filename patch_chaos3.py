import sys

with open('lib/resilience/chaos/chaos_test.go', 'r') as f:
    content = f.read()

import_block = """import (
	"context"
	"errors"
	"os"
	"testing"
)"""

if "os" not in content[:200]:
    content = content.replace('import (\n\t"context"\n\t"errors"\n\t"testing"\n)', import_block)

with open('lib/resilience/chaos/chaos_test.go', 'w') as f:
    f.write(content)
