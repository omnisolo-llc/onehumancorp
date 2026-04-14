package telemetry

import (
	"go.opentelemetry.io/otel/metric"
)

var (
	AutoDreamRecordsSyncedTotal metric.Int64Counter
	AutoDreamSyncErrorsTotal    metric.Int64Counter
)

func initAutoDreamSyncMetrics(m mockableMeter) error {
	var err error
	AutoDreamRecordsSyncedTotal, err = m.Int64Counter(
		"autodream_records_synced_total",
		metric.WithDescription("Total number of AutoDream records successfully synced"),
	)
	if err != nil {
		return err
	}

	AutoDreamSyncErrorsTotal, err = m.Int64Counter(
		"autodream_sync_errors_total",
		metric.WithDescription("Total number of AutoDream sync errors"),
	)
	return err
}
