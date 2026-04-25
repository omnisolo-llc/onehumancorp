package orchestration

import (
	"os"
	"testing"

	"github.com/onehumancorp/mono/src/server/telemetry"
)

func TestMain(m *testing.M) {
	cleanup, err := telemetry.InitTelemetry()
	if err == nil && cleanup != nil {
		defer cleanup()
	}
	os.Exit(m.Run())
}
