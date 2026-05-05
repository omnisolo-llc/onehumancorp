package telemetry

import (
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter = otel.Meter("autodream")

	// AutodreamRecordsSyncedTotal tracks the number of records synced
	AutodreamRecordsSyncedTotal metric.Int64Counter

	// AutodreamSyncErrorsTotal tracks the number of sync errors
	AutodreamSyncErrorsTotal metric.Int64Counter
)

func init() {
	var err error
	AutodreamRecordsSyncedTotal, err = meter.Int64Counter(
		"autodream_records_synced_total",
		metric.WithDescription("Number of autodream records successfully synced"),
	)
	if err != nil {
		panic(err)
	}

	AutodreamSyncErrorsTotal, err = meter.Int64Counter(
		"autodream_sync_errors_total",
		metric.WithDescription("Number of autodream sync errors"),
	)
	if err != nil {
		panic(err)
	}
}
