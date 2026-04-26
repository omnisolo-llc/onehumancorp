package sync

import (
	"context"
	"os"
	"testing"
	"time"
)

type mockStore struct {
	driver string
	queries []string
}

func (m *mockStore) Exec(ctx context.Context, query string, args ...interface{}) error {
	m.queries = append(m.queries, query)
	return nil
}

func (m *mockStore) Driver() string {
	return m.driver
}

func TestSyncDeltas_Postgres(t *testing.T) {
	store := &mockStore{driver: "postgres"}
	syncer := NewMCPSyncer(store)

	err := syncer.SyncDeltas(context.Background(), []SyncDelta{
		{TenantID: "tenant_1", EntityID: "1", Data: "{}", UpdatedAt: time.Now()},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(store.queries) != 1 {
		t.Fatalf("expected 1 query, got %d", len(store.queries))
	}

	expectedQuery := `INSERT INTO mcp_deltas (tenant_id, entity_id, data, updated_at)
VALUES ($1, $2, $3, $4)
ON CONFLICT (tenant_id, entity_id) DO UPDATE
SET data = EXCLUDED.data, updated_at = EXCLUDED.updated_at
WHERE mcp_deltas.updated_at < EXCLUDED.updated_at`

	if store.queries[0] != expectedQuery {
		t.Errorf("unexpected query:\nGot: %v\nExp: %v", store.queries[0], expectedQuery)
	}
}

func TestSyncDeltas_SQLite(t *testing.T) {
	store := &mockStore{driver: "sqlite3"}
	syncer := NewMCPSyncer(store)

	err := syncer.SyncDeltas(context.Background(), []SyncDelta{
		{TenantID: "tenant_1", EntityID: "2", Data: "{}", UpdatedAt: time.Now()},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(store.queries) != 1 {
		t.Fatalf("expected 1 query, got %d", len(store.queries))
	}

	expectedQuery := `INSERT INTO mcp_deltas (tenant_id, entity_id, data, updated_at)
VALUES (?, ?, ?, ?)
ON CONFLICT (tenant_id, entity_id) DO UPDATE
SET data = EXCLUDED.data, updated_at = EXCLUDED.updated_at
WHERE mcp_deltas.updated_at < EXCLUDED.updated_at`

	if store.queries[0] != expectedQuery {
		t.Errorf("unexpected query:\nGot: %v\nExp: %v", store.queries[0], expectedQuery)
	}
}

func TestSyncDeltas_Telemetry(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_TELEMETRY_ENABLED", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_TELEMETRY_ENABLED")

	store := &mockStore{driver: "sqlite"}
	syncer := NewMCPSyncer(store)
	err := syncer.SyncDeltas(context.Background(), []SyncDelta{
		{TenantID: "tenant_1", EntityID: "3", Data: "{}", UpdatedAt: time.Now()},
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}
