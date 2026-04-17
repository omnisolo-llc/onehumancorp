package telemetry

import (
	"go.opentelemetry.io/otel/metric"
)

func initSandboxViolationMetrics(m mockableMeter) error {
	var err error
	SandboxViolationTotal, err = m.Int64Counter(
		"telemetry.sandbox_violation_total",
		metric.WithDescription("Total number of sandbox violations"),
	)
	if err != nil {
		return err
	}
	return nil
}
