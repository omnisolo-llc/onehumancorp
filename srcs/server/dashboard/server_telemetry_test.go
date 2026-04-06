package dashboard

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleTelemetrySync(t *testing.T) {
	// Create mock Hub and SIPDB
	dbMock := &mockDBProvider{}
	sipDB := orchestration.NewSIPDB("org-123", dbMock)
	hub := orchestration.NewHub()
	hub.SetSIPDB(sipDB)

	s := &Server{
		hub: hub,
	}

	metrics := []map[string]interface{}{
		{"metric_type": "swarm_task_completed", "mission_id": "123"},
	}
	bJSON, _ := json.Marshal(metrics)

	req := httptest.NewRequest("POST", "/api/telemetry/sync", bytes.NewReader(bJSON))
	ctx := auth.ContextWithClaims(req.Context(), &auth.Claims{OrganizationID: "org-123"})
	req = req.WithContext(ctx)
	w := httptest.NewRecorder()

	s.handleTelemetrySync(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", w.Code)
	}

	if len(dbMock.queries) != 1 {
		t.Fatalf("expected 1 insert query, got %d", len(dbMock.queries))
	}
}

func TestHandleTelemetrySync_NoAuth(t *testing.T) {
	s := &Server{}
	metrics := []map[string]interface{}{}
	b, _ := json.Marshal(metrics)

	req := httptest.NewRequest("POST", "/api/telemetry/sync", bytes.NewReader(b))
	w := httptest.NewRecorder()

	s.handleTelemetrySync(w, req)

	if w.Code != http.StatusUnauthorized {
		t.Fatalf("expected 401, got %d", w.Code)
	}
}

type mockDBProvider struct {
	db.Provider
	queries []string
}

func (m *mockDBProvider) Begin(ctx context.Context) (db.Tx, error) {
	return &mockTx{m}, nil
}

type mockTx struct {
	m *mockDBProvider
}

func (t *mockTx) Exec(ctx context.Context, sqlQuery string, args ...interface{}) (int64, error) {
	t.m.queries = append(t.m.queries, sqlQuery)
	return 1, nil
}

func (t *mockTx) Query(ctx context.Context, sqlQuery string, args ...interface{}) (db.Rows, error) {
	return nil, nil
}

func (t *mockTx) QueryRow(ctx context.Context, sqlQuery string, args ...interface{}) db.Row {
	return nil
}

func (t *mockTx) Commit(ctx context.Context) error {
	return nil
}

func (t *mockTx) Rollback(ctx context.Context) error {
	return nil
}
