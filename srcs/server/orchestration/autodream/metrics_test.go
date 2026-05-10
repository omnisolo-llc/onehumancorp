package autodream

import (
	"testing"
	"github.com/prometheus/client_golang/prometheus"
	"github.com/stretchr/testify/assert"
)

func TestMetricsRegistration(t *testing.T) {
	// If the metrics are registered, trying to register them again should cause a panic.
	// Since they are registered in init(), we can check if they are non-nil.
	assert.NotNil(t, MemoriesProcessedTotal)
	assert.NotNil(t, BatchProcessingDuration)
	assert.NotNil(t, ConsolidationErrorsTotal)

	// Ensure we can interact with them
	MemoriesProcessedTotal.WithLabelValues("test_mode").Inc()
	BatchProcessingDuration.WithLabelValues("test_mode").Observe(1.0)
	ConsolidationErrorsTotal.WithLabelValues("test_mode", "test_error").Inc()

	// Verify using the DefaultRegisterer (would panic if not registered or already registered with different type)
	// Just collecting is enough.
	metricFamilies, err := prometheus.DefaultGatherer.Gather()
	assert.NoError(t, err)

	foundMemories := false
	for _, mf := range metricFamilies {
		if mf.GetName() == "ohc_autodream_memories_processed_total" {
			foundMemories = true
		}
	}
	assert.True(t, foundMemories)
}
