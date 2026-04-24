package hybrid_sync

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/lib/resilience/chaos"
	_ "modernc.org/sqlite"
)

func TestHybridSyncDaemon_Chaos(t *testing.T) {
	// Setup SQLite in-memory db
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE swarm_memory_embeddings (
			memory_id TEXT PRIMARY KEY,
			context TEXT NOT NULL
		)
	`)
	if err != nil {
		t.Fatalf("failed to create swarm_memory_embeddings table: %v", err)
	}

	_, err = sqlDB.Exec(`
		INSERT INTO swarm_memory_embeddings (memory_id, context)
		VALUES ('m-chaos', '{"escalation_required":true, "details":"chaos test"}')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	// Mock cloud API that might fail
	chaosMode := chaos.ConnectionDrop
	inj := chaos.NewInjector(chaos.ChaosMode(chaosMode), 123)

	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if err := inj.Inject(r.Context()); err != nil {
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer srv.Close()

	daemon := NewHybridSyncDaemon(dbWrapper, 100*time.Millisecond, srv.URL)

	// Attempt sync with chaos
	daemon.ProcessSync(context.Background())

	// Verify that if it failed, the data is still marked for escalation
	var contextData string
	err = sqlDB.QueryRow("SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'm-chaos'").Scan(&contextData)
	if err != nil {
		t.Fatalf("failed to query db: %v", err)
	}

	var parsed map[string]interface{}
	json.Unmarshal([]byte(contextData), &parsed)
	if req, ok := parsed["escalation_required"].(bool); !ok || !req {
		t.Errorf("expected escalation_required to remain true after failed sync, context: %s", contextData)
	}

	// Now remove chaos and sync again
	inj = chaos.NewInjector(chaos.NoChaos, 0)
	daemon.ProcessSync(context.Background())

	err = sqlDB.QueryRow("SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'm-chaos'").Scan(&contextData)
	if err != nil {
		t.Fatalf("failed to query db: %v", err)
	}

	json.Unmarshal([]byte(contextData), &parsed)
	if req, ok := parsed["escalation_required"].(bool); ok && req {
		t.Errorf("expected escalation_required to be false after successful sync, context: %s", contextData)
	}
}

func TestHybridSyncDaemon_SyncLagChaos(t *testing.T) {
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, _ = sqlDB.Exec(`CREATE TABLE swarm_memory_embeddings (memory_id TEXT PRIMARY KEY, context TEXT NOT NULL)`)
	_, _ = sqlDB.Exec(`INSERT INTO swarm_memory_embeddings (memory_id, context) VALUES ('m-lag', '{"escalation_required":true}')`)

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusOK)
	}))
	defer srv.Close()

	daemon := NewHybridSyncDaemon(dbWrapper, 100*time.Millisecond, srv.URL)

	inj := chaos.NewInjector(chaos.SyncLag, 456)

	start := time.Now()
	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	_ = inj.Inject(ctx)
	daemon.ProcessSync(ctx)

	duration := time.Since(start)
	if duration < 500*time.Millisecond {
		t.Errorf("expected SyncLag to delay process, but took only %v", duration)
	}
}
