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

func TestHybridSyncDaemon_ProcessSync(t *testing.T) {
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
			('m1', '{"escalation_required":true, "details":" email is test@example.com"}'),
			('m2', '{"escalation_required":false, "details":"should be ignored"}'),
			('m3', '{"escalation_required":1, "data":"some public data"}')
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	// Mock cloud API
	var receivedMission AgentMissionPayload
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/sync/escalation" && r.Method == http.MethodPost {
			if err := json.NewDecoder(r.Body).Decode(&receivedMission); err != nil {
				w.WriteHeader(http.StatusBadRequest)
				return
			}
			w.WriteHeader(http.StatusOK)
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer srv.Close()

	daemon := NewHybridSyncDaemon(dbWrapper, 1*time.Minute, srv.URL)

	// Process sync manually for testing
	daemon.ProcessSync(context.Background())

	// Validate received payload
	if len(receivedMission.ContextData) != 2 {
		t.Fatalf("expected 2 memories to be synced, got %d", len(receivedMission.ContextData))
	}

	hasM1 := false
	hasM3 := false
	for _, p := range receivedMission.ContextData {
		if p.MemoryID == "m1" {
			hasM1 = true
			expectedPayload := `{"details":" email is [REDACTED_EMAIL]","escalation_required":true}`
			if p.Context != expectedPayload {
				t.Errorf("expected sanitized context %q, got %q", expectedPayload, p.Context)
			}
		} else if p.MemoryID == "m3" {
			hasM3 = true
		}
	}

	if !hasM1 || !hasM3 {
		t.Errorf("expected to sync m1 and m3")
	}

	if receivedMission.Title != "Hybrid MCP RAG Protocol Escalation" {
		t.Errorf("unexpected mission title: %s", receivedMission.Title)
	}

	// Validate db status updated
	var contextData string
	err = sqlDB.QueryRow("SELECT context FROM swarm_memory_embeddings WHERE memory_id = 'm1'").Scan(&contextData)
	if err != nil {
		t.Fatalf("failed to query m1 context: %v", err)
	}

	var parsedContext map[string]interface{}
	json.Unmarshal([]byte(contextData), &parsedContext)

	if val, ok := parsedContext["escalation_required"]; ok {
		if boolVal, isBool := val.(bool); isBool && boolVal {
			t.Error("expected m1 escalation_required to be false, but was true")
		} else if floatVal, isFloat := val.(float64); isFloat && floatVal == 1 {
			t.Error("expected m1 escalation_required to be false, but was 1")
		}
	}
}
