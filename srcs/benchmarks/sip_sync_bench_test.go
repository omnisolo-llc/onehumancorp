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
		for j := 0; j < 500; j++ {
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
		// Setup mock records in db directly or rely on the function handling 0 records quickly.
		// Since this is a test, we want to measure throughput with payload
		_, err := db.Provider().Exec(ctx, "DELETE FROM swarm_memory_embeddings")
		if err != nil {
			b.Fatalf("Failed to reset db: %v", err)
		}

		for j := 0; j < 100; j++ {
			_, err = db.Provider().Exec(ctx, "INSERT INTO swarm_memory_embeddings (organization_id, memory_id, context) VALUES ($1, $2, $3)",
				"default-org", fmt.Sprintf("mem-%d", j), `{"key":"value"}`)
			if err != nil {
				b.Fatalf("Failed to insert context: %v", err)
			}
		}

		b.StartTimer()
		_, err = db.SyncContextSync(ctx, ts.URL)
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
		_, err := db.Provider().Exec(ctx, "DELETE FROM agent_missions")
		if err != nil {
			b.Fatalf("Failed to reset db: %v", err)
		}

		for j := 0; j < 100; j++ {
			_, err = db.Provider().Exec(ctx, "INSERT INTO agent_missions (id, organization_id, status, payload) VALUES ($1, $2, $3, $4)",
				fmt.Sprintf("miss-%d", j), "default-org", "PENDING", `{"key":"value"}`)
			if err != nil {
				b.Fatalf("Failed to insert mission: %v", err)
			}
		}

		b.StartTimer()
		_, err = db.SyncMissions(ctx, ts.URL)
		if err != nil {
			b.Fatalf("SyncMissions failed: %v", err)
		}
	}
}
