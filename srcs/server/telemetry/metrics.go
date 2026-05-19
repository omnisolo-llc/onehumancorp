package telemetry

import (
	"github.com/prometheus/client_golang/prometheus"
	"github.com/prometheus/client_golang/prometheus/promauto"
)

var (
	SyncCompletedCount = promauto.NewCounter(prometheus.CounterOpts{
		Name: "sync_completed_count",
		Help: "Total number of completed syncs",
	})
	SyncFailedCount = promauto.NewCounter(prometheus.CounterOpts{
		Name: "sync_failed_count",
		Help: "Total number of failed syncs",
	})
)
