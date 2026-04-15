with open("srcs/server/interop/mesh.go", "r") as f:
    content = f.read()

import re

# Add variables to mesh.go if they don't exist
if "var meshMessagesPublished" not in content:
    replacement = """
import (
	"context"
	"fmt"
	"log/slog"
	"os"
	"sync"

	"github.com/redis/rueidis"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter                 = otel.Meter("github.com/onehumancorp/mono/srcs/server/interop")
	meshMessagesPublished, _ = meter.Int64Counter("mesh.messages.published")
	meshMessagesReceived, _  = meter.Int64Counter("mesh.messages.received")
)
"""
    content = re.sub(r'import \([\s\S]*?\)', replacement, content, count=1)

    with open("srcs/server/interop/mesh.go", "w") as f:
        f.write(content)
