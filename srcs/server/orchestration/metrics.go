package orchestration

import (
	"context"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter            = otel.Meter("github.com/onehumancorp/mono/srcs/server/orchestration")
	ragSyncCount     metric.Int64Counter
	ragSyncErrCount  metric.Int64Counter
)

func init() {
	var err error
	ragSyncCount, err = meter.Int64Counter("ohc.rag.sync.success", metric.WithDescription("Number of RAG memories successfully synced to the cloud"))
	if err != nil {
		panic(err)
	}

	ragSyncErrCount, err = meter.Int64Counter("ohc.rag.sync.error", metric.WithDescription("Number of RAG memories failed to sync to the cloud"))
	if err != nil {
		panic(err)
	}
}

func recordRAGSync(ctx context.Context, success bool) {
	if success {
		ragSyncCount.Add(ctx, 1)
	} else {
		ragSyncErrCount.Add(ctx, 1)
	}
}
