package orchestration

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/src/server/orchestration"
)

func BenchmarkSIPDB_SyncBufferedMetrics(b *testing.B) {
	dbPath := filepath.Join(b.TempDir(), "bench_sync_metrics.db")
	db, err := orchestration.NewSIPDB(dbPath)
	if err != nil {
		b.Fatalf("Failed to create db: %v", err)
	}
	defer db.Close()

	ctx := context.Background()

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		io.ReadAll(r.Body)
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	b.ResetTimer()
	for i := 0; i < b.N; i++ {
		b.StopTimer()
		for j := 0; j < 500; j++ {
			_ = db.BufferMetric(ctx, "benchmark_metric", fmt.Sprintf(`{"index":%d,"agent_id":"a1"}`, j))
		}
		b.StartTimer()

		_, err := db.SyncBufferedMetrics(ctx, ts.URL, 500)
		if err != nil {
			b.Fatalf("SyncBufferedMetrics failed: %v", err)
		}
	}
}
