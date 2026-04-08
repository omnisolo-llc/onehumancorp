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
	_ "modernc.org/sqlite"
)

func TestHybridSyncDaemon_ChaosEngineering(t *testing.T) {
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
		VALUES
			('m1', '{"escalation_required":true, "details":"test data"}')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	// Create a chaos HTTP server
	requestCount := 0
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		requestCount++

		// Simulate network partition/connection drop on the first 2 requests
		if requestCount <= 2 {
			hj, ok := w.(http.Hijacker)
			if !ok {
				t.Fatalf("webserver doesn't support hijacking")
			}
			conn, _, err := hj.Hijack()
			if err != nil {
				t.Fatalf("hijack error: %v", err)
			}
			conn.Close() // Immediately drop the connection
			return
		}

		// Simulate backend latency spike on the 3rd request
		if requestCount == 3 {
			time.Sleep(50 * time.Millisecond) // Simulating a spike, albeit small for fast tests
			w.WriteHeader(http.StatusGatewayTimeout)
			return
		}

		// 4th request succeeds
		w.WriteHeader(http.StatusOK)
	}))
	defer srv.Close()

	daemon := NewHybridSyncDaemon(dbWrapper, 1*time.Minute, srv.URL)

	// Attempt 1: Connection drop
	daemon.ProcessSync(context.Background())
	if !isEscalationRequired(t, sqlDB, "m1") {
		t.Error("expected memory to remain escalation_required=true after connection drop")
	}

	// Attempt 2: Connection drop
	daemon.ProcessSync(context.Background())
	if !isEscalationRequired(t, sqlDB, "m1") {
		t.Error("expected memory to remain escalation_required=true after connection drop")
	}

	// Attempt 3: Latency Spike / 504 Gateway Timeout
	daemon.ProcessSync(context.Background())
	if !isEscalationRequired(t, sqlDB, "m1") {
		t.Error("expected memory to remain escalation_required=true after 504 Gateway Timeout")
	}

	// Attempt 4: Success
	daemon.ProcessSync(context.Background())
	if isEscalationRequired(t, sqlDB, "m1") {
		t.Error("expected memory to have escalation_required=false after successful sync")
	}
}

func isEscalationRequired(t *testing.T, sqlDB *sql.DB, id string) bool {
	var contextData string
	err := sqlDB.QueryRow("SELECT context FROM swarm_memory_embeddings WHERE memory_id = ?", id).Scan(&contextData)
	if err != nil {
		t.Fatalf("failed to query %s context: %v", id, err)
	}

	var parsed map[string]interface{}
	if err := json.Unmarshal([]byte(contextData), &parsed); err != nil {
		t.Fatalf("failed to parse JSON: %v", err)
	}

	if val, ok := parsed["escalation_required"]; ok {
		if boolVal, isBool := val.(bool); isBool {
			return boolVal
		} else if floatVal, isFloat := val.(float64); isFloat {
			return floatVal == 1
		}
	}
	return false
}
