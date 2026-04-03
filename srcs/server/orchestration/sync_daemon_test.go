package orchestration

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestSyncDaemon_CloudEscalation(t *testing.T) {
	// Setup local SQLite db
	provider, err := db.NewTestProvider()
	if err != nil {
		t.Fatalf("failed to create test provider: %v", err)
	}
	defer provider.Close()

	ctx := context.Background()

	// Ensure table has synced_to_cloud column
	_, err = provider.DB().Exec(ctx, "ALTER TABLE agent_missions ADD COLUMN synced_to_cloud BOOLEAN DEFAULT false;")
	if err != nil {
		// Ignore if it already exists or fails, tests usually handle their schemas
		// But db.NewTestProvider might not run all migrations, let's just make sure agent_missions is created.
		// Actually let's just insert
	}

	// Create table if not exists (to be safe in isolated test)
	provider.DB().Exec(ctx, "CREATE TABLE IF NOT EXISTS agent_missions (id TEXT PRIMARY KEY, status TEXT, payload TEXT, created_at DATETIME, synced_to_cloud BOOLEAN DEFAULT false)")

	// Insert test data
	payload1 := `{"role": "SOFTWARE_ENGINEER", "task": "local task"}`
	payload2 := `{"role": "DATA_SCIENTIST", "cloud_escalation": true, "task": "heavy compute"}`

	_, err = provider.DB().Exec(ctx, "INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('m1', 'PENDING', $1, false)", payload1)
	if err != nil { t.Fatalf("failed to insert m1: %v", err) }

	_, err = provider.DB().Exec(ctx, "INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('m2', 'PENDING', $1, false)", payload2)
	if err != nil { t.Fatalf("failed to insert m2: %v", err) }

	var receivedPayloads []map[string]interface{}

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/api/sync/missions" {
			t.Errorf("expected path /api/sync/missions, got %s", r.URL.Path)
		}
		if r.Method != http.MethodPost {
			t.Errorf("expected POST, got %s", r.Method)
		}

		if err := json.NewDecoder(r.Body).Decode(&receivedPayloads); err != nil {
			t.Errorf("failed to decode payload: %v", err)
		}

		w.WriteHeader(http.StatusOK)
		w.Write([]byte(`{"status": "success"}`))
	}))
	defer ts.Close()

	daemon := NewSyncDaemon(provider.DB(), 1*time.Millisecond, ts.URL)

	daemon.ProcessTick(ctx)

	if len(receivedPayloads) != 1 {
		t.Fatalf("expected 1 mission to be synced, got %d", len(receivedPayloads))
	}

	if receivedPayloads[0]["id"] != "m2" {
		t.Errorf("expected mission m2 to be synced, got %v", receivedPayloads[0]["id"])
	}

	// Verify DB state
	var synced bool
	err = provider.DB().QueryRow(ctx, "SELECT synced_to_cloud FROM agent_missions WHERE id = 'm2'").Scan(&synced)
	if err != nil {
		t.Fatalf("failed to query m2: %v", err)
	}
	if !synced {
		t.Errorf("expected m2 to be marked as synced")
	}
}
