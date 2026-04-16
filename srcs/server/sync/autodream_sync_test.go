package sync

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestAutoDreamSyncEngine_ProcessForecastTick(t *testing.T) {
	// Initialize telemetry to avoid nil pointer issues
	// telemetry is mocked/ignored if not initialized, but InitTelemetry() initializes the prometheus metrics.
	_, err := telemetry.InitTelemetry()
	if err != nil {
		t.Logf("failed to init telemetry: %v", err)
	}

	// 1. Setup InMemory SQLite DB
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()
	dbWrapper, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer dbWrapper.Close()

	// 2. Setup schema
	if err := dbWrapper.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	// Wait, run migrations might not run 011 properly if it doesn't support the ADD COLUMN correctly,
	// but let's test it. SQLite does support standard ADD COLUMN.
	// We explicitly create the tables needed for the test just in case.
	// Actually, dbWrapper.RunMigrations runs everything, including our 011 migration.

	// Insert test data
	_, err = dbWrapper.Exec(ctx, `
		INSERT INTO embedding_cache (content_hash, embedding, synced_to_cloud)
		VALUES ('hash1', 'vec1', false), ('hash2', 'vec2', true)
	`)
	if err != nil {
		t.Fatalf("failed to insert embedding_cache test data: %v", err)
	}

	_, err = dbWrapper.Exec(ctx, `
		INSERT INTO agent_missions (id, status, payload, created_at, synced_to_cloud)
		VALUES ('m1', 'PENDING', '{}', CURRENT_TIMESTAMP, false)
	`)
	if err != nil {
		t.Fatalf("failed to insert agent_missions test data: %v", err)
	}

	// 3. Setup mock cloud API
	var receivedPayloads []AutoDreamPayload
	cloudServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			w.WriteHeader(http.StatusMethodNotAllowed)
			return
		}
		var payloads []AutoDreamPayload
		if err := json.NewDecoder(r.Body).Decode(&payloads); err != nil {
			w.WriteHeader(http.StatusBadRequest)
			return
		}
		receivedPayloads = append(receivedPayloads, payloads...)
		w.WriteHeader(http.StatusOK)
	}))
	defer cloudServer.Close()

	// 4. Run the sync engine synchronously
	os.Setenv("OHC_STANDALONE", "true") // Not strictly needed because Setenv("DATABASE_URL") controls IsSQLite()
	engine := NewAutoDreamSyncEngine(dbWrapper, 1*time.Minute, cloudServer.URL)

	engine.ProcessForecastTick(ctx)

	// 5. Verify the payloads
	// Note: previous migrations seeded some tasks, so we expect exactly 4 payloads
	if len(receivedPayloads) != 4 {
		t.Fatalf("expected 4 payloads synced, got %d", len(receivedPayloads))
	}

	embeddingSynced := false
	missionSynced := false
	for _, p := range receivedPayloads {
		if p.Type == "embedding" && p.ID == "hash1" {
			embeddingSynced = true
		}
		if p.Type == "mission" && p.ID == "m1" {
			missionSynced = true
		}
	}

	if !embeddingSynced || !missionSynced {
		t.Errorf("expected both embedding hash1 and mission m1 to be synced. Got: %+v", receivedPayloads)
	}

	// 6. Verify database state is updated
	var count int
	err = dbWrapper.QueryRow(ctx, "SELECT COUNT(*) FROM embedding_cache WHERE synced_to_cloud = false").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count embedding_cache: %v", err)
	}
	if count != 0 {
		t.Errorf("expected 0 unsynced embeddings, got %d", count)
	}

	err = dbWrapper.QueryRow(ctx, "SELECT COUNT(*) FROM agent_missions WHERE synced_to_cloud = false").Scan(&count)
	if err != nil {
		t.Fatalf("failed to count agent_missions: %v", err)
	}
	if count != 0 {
		t.Errorf("expected 0 unsynced missions, got %d", count)
	}
}

func TestAutoDreamSyncEngine_syncAgentMissions_PIIRedaction(t *testing.T) {
	// Initialize telemetry to avoid nil pointer issues
	_, err := telemetry.InitTelemetry()
	if err != nil {
		t.Logf("failed to init telemetry: %v", err)
	}

	// 1. Setup InMemory SQLite DB
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()
	dbWrapper, err := db.New(ctx)
	if err != nil {
		t.Fatalf("failed to init db: %v", err)
	}
	defer dbWrapper.Close()

	if err := dbWrapper.RunMigrations(ctx); err != nil {
		t.Fatalf("failed to run migrations: %v", err)
	}

	// Insert test data with invalid JSON containing PII
	piiPayload := "this is an invalid json string with email test@example.com"
	_, err = dbWrapper.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ($1, $2, $3, false)", "mission1", "failed", piiPayload)
	if err != nil {
		t.Fatalf("failed to insert mission: %v", err)
	}

	// Mock server to catch the payload
	var capturedPayload []AutoDreamPayload
	mockServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if err := json.NewDecoder(r.Body).Decode(&capturedPayload); err != nil {
			t.Errorf("failed to decode payload: %v", err)
		}
		w.WriteHeader(http.StatusOK)
	}))
	defer mockServer.Close()

	engine := NewAutoDreamSyncEngine(dbWrapper, 1*time.Minute, mockServer.URL)
	engine.syncAgentMissions(ctx)

	if len(capturedPayload) == 0 {
		t.Fatalf("expected at least 1 payload, got %d", len(capturedPayload))
	}

	expected := "this is an invalid json string with email [REDACTED_EMAIL]"
	var found bool
	for _, p := range capturedPayload {
		if p.Data == expected {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("expected redacted data %q, got %q", expected, capturedPayload[0].Data)
	}
}
