package benchmarks

import (
	"context"
	"fmt"
	"io"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
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
		for j := 0; j < 50; j++ {
			_ = db.BufferMetric(ctx, "benchmark_metric", fmt.Sprintf(`{"index":%d,"agent_id":"a1"}`, j))
		}
		b.StartTimer()

		_, err := db.SyncBufferedMetrics(ctx, ts.URL)
		if err != nil {
			b.Fatalf("SyncBufferedMetrics failed: %v", err)
		}
	}
}

func BenchmarkSIPDB_SyncContextSync(b *testing.B) {
	dbPath := filepath.Join(b.TempDir(), "bench_sync_context.db")
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
		for j := 0; j < 50; j++ {
			err := db.UpdateMemory(ctx, fmt.Sprintf("mem-%d-%d", i, j), fmt.Sprintf(`{"data":"payload-%d"}`, j))
            if err != nil {
                b.Fatalf("Failed to insert: %v", err)
            }
		}
		b.StartTimer()

		_, err := db.SyncContextSync(ctx, ts.URL)
		if err != nil {
			b.Fatalf("SyncContextSync failed: %v", err)
		}
	}
}

func BenchmarkSIPDB_SyncMissions(b *testing.B) {
	dbPath := filepath.Join(b.TempDir(), "bench_sync_missions.db")
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
		for j := 0; j < 50; j++ {
			msg := orchestration.Message{
				ID: fmt.Sprintf("mission-%d-%d", i, j),
                ToAgent: "TEST_AGENT",
                Content: fmt.Sprintf(`{"data":"payload-%d"}`, j),
			}
			err := db.DelegateMission(ctx, msg.ID, msg.ToAgent, msg)
            if err != nil {
                b.Fatalf("Failed to insert: %v", err)
            }
		}
		b.StartTimer()

		_, err := db.SyncMissions(ctx, ts.URL)
		if err != nil {
			b.Fatalf("SyncMissions failed: %v", err)
		}
	}
}
