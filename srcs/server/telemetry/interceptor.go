package telemetry

import (
	"context"
	"os"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// InitStandaloneInterceptor configures the buffer logic for standalone mode if telemetry is enabled.
func InitStandaloneInterceptor(provider db.Provider) {
	if os.Getenv("OHC_STANDALONE") == "true" {
		if os.Getenv("OHC_TELEMETRY_ENABLED") == "true" {
			BufferMetricFunc = func(ctx context.Context, metricType string, payload string) error {
				query := `INSERT INTO telemetry_buffer (metric_type, payload) VALUES ($1, $2)`
				_, err := provider.Exec(ctx, query, metricType, payload)
				return err
			}
		} else {
			// Privacy Compliance Guardrail: Explicitly disable if telemetry is not opted in
			BufferMetricFunc = nil
		}
	}
}
