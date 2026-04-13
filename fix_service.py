with open("srcs/server/memory/autodream/service.go", "r") as f:
    content = f.read()

import_patch = """
	"context"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/srcs/server/memory"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
"""

content = content.replace('	"github.com/onehumancorp/mono/srcs/server/memory"\n', import_patch)

metrics_patch = """
	// Track metric via OpenTelemetry
	start := time.Now()
	telemetry.RecordLLMRequest(ctx, "summarize", float64(time.Since(start).Milliseconds()))

	slog.Info("AutoDreamService: consolidated task memory", "task_id", taskID)
"""

content = content.replace('\t// Track metric via OpenTelemetry would go here\n\tslog.Info("AutoDreamService: consolidated task memory", "task_id", taskID)', metrics_patch)

with open("srcs/server/memory/autodream/service.go", "w") as f:
    f.write(content)
