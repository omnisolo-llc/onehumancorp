package api

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type mockCRDTProvider struct {
	db.Provider
}

func (m *mockCRDTProvider) IsSQLite() bool { return false }
func (m *mockCRDTProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	return 1, nil
}
func (m *mockCRDTProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	return &mockCRDTRows{count: 2}, nil
}

type mockCRDTRows struct {
	count int
	idx   int
}

func (r *mockCRDTRows) Next() bool {
	if r.idx < r.count {
		r.idx++
		return true
	}
	return false
}

func (r *mockCRDTRows) Scan(dest ...any) error {
	// Need to mock data scan correctly according to time.Time
	// HandleCRDTPull expects string, string, string, time.Time
	return errors.New("scan mock unhandled")
}

func (r *mockCRDTRows) Close() {}
func (r *mockCRDTRows) Columns() ([]string, error) { return nil, nil }
func (r *mockCRDTRows) Err() error { return nil }


func TestHandleCRDTSync_MethodNotAllowed(t *testing.T) {
	req := httptest.NewRequest(http.MethodGet, "/api/v1/sync/mcp-deltas", nil)
	rr := httptest.NewRecorder()

	handler := HandleCRDTSync(nil)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusMethodNotAllowed {
		t.Errorf("expected %v got %v", http.StatusMethodNotAllowed, status)
	}
}

func TestHandleCRDTSync_InvalidJSON(t *testing.T) {
	req := httptest.NewRequest(http.MethodPost, "/api/v1/sync/mcp-deltas", bytes.NewBuffer([]byte("invalid json")))
	rr := httptest.NewRecorder()

	handler := HandleCRDTSync(nil)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusBadRequest {
		t.Errorf("expected %v got %v", http.StatusBadRequest, status)
	}
}

func TestHandleCRDTSync_Success(t *testing.T) {
	payload := `{"deltas": [{"id": "1", "entity_id": "e1", "data": "{}", "updated_at": "2026-04-17T12:00:00Z"}]}`
	req := httptest.NewRequest(http.MethodPost, "/api/v1/sync/mcp-deltas", bytes.NewBuffer([]byte(payload)))
	rr := httptest.NewRecorder()

	hub := orchestration.NewHub()

	sipDB, _ := orchestration.NewSIPDBWithProvider(&mockCRDTProvider{}, "test-org")
	hub.SetSIPDB(sipDB)
	handler := HandleCRDTSync(hub)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("expected %v got %v", http.StatusOK, status)
	}

	var res map[string]interface{}
	json.NewDecoder(rr.Body).Decode(&res)

	if res["status"] != "success" {
		t.Errorf("expected success, got %v", res["status"])
	}
}

func TestHandleCRDTPull_MethodNotAllowed(t *testing.T) {
	req := httptest.NewRequest(http.MethodPost, "/api/v1/sync/mcp-deltas", nil)
	rr := httptest.NewRecorder()

	handler := HandleCRDTPull(nil)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusMethodNotAllowed {
		t.Errorf("expected %v got %v", http.StatusMethodNotAllowed, status)
	}
}

func TestHandleCRDTPull_Success(t *testing.T) {
	req := httptest.NewRequest(http.MethodGet, "/api/v1/sync/mcp-deltas", nil)
	rr := httptest.NewRecorder()

	hub := orchestration.NewHub()

	sipDB, _ := orchestration.NewSIPDBWithProvider(&mockCRDTProvider{}, "test-org")
	hub.SetSIPDB(sipDB)
	handler := HandleCRDTPull(hub)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("expected %v got %v", http.StatusOK, status)
	}
}
