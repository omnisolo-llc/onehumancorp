package telemetry

import (
	"context"
	"os"
	"testing"
	"github.com/stretchr/testify/assert"
)

func TestBufferMetricHelper(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	globalSyncEngine = nil
	attrs := map[string]interface{}{"email": "test@example.com"}
	assert.NotPanics(t, func() {
		bufferMetricHelper(context.Background(), "test_metric", 1.0, attrs)
	})
}
