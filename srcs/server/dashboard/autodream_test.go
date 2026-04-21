package dashboard

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleAutoDreamSync(t *testing.T) {
	provider, err := orchestration.NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer provider.Close()

	hub := orchestration.NewHub()
	hub.SetSIPDB(provider)
	s := &Server{
		hub: hub,
	}

	pool := s.hub.SIPDB().Provider()
	_, err = pool.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS shared_tasks (id TEXT PRIMARY KEY, title TEXT, payload TEXT, status TEXT);
		CREATE TABLE IF NOT EXISTS swarm_tasks (id TEXT PRIMARY KEY, title TEXT, payload TEXT, status TEXT);
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT,
			embedding TEXT,
			source_mission_id TEXT,
			organization_id TEXT,
			agent_id TEXT,
			source_type TEXT,
			processed_at TEXT,
			created_at TEXT DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to setup schema: %v", err)
	}

	_, _ = pool.Exec(context.Background(), "INSERT INTO shared_tasks (id, title, payload, status) VALUES ('st1', 'T', '{}', 'COMPLETED')")

	mux := http.NewServeMux()
	mux.HandleFunc("/api/v1/autodream/sync", s.handleAutoDreamSync)

	req := httptest.NewRequest("POST", "/api/v1/autodream/sync", bytes.NewBuffer([]byte(`{"force_reindex": true}`)))

	// Inject claims to bypass auth
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org", Roles: []string{"system"}})
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v body %s", status, http.StatusOK, rr.Body.String())
	}

	var resp map[string]string
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v body %s", err, rr.Body.String())
	}

	if resp["status"] != "success" {
		t.Errorf("expected status success, got %v", resp["status"])
	}
}

func TestHandleAutoDreamQuery(t *testing.T) {
	provider, err := orchestration.NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("failed to create db: %v", err)
	}
	defer provider.Close()

	hub := orchestration.NewHub()
	hub.SetSIPDB(provider)
	s := &Server{
		hub: hub,
	}

	// Make sure table exists
	pool := s.hub.SIPDB().Provider()
	_, _ = pool.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id TEXT PRIMARY KEY,
			content TEXT,
			embedding TEXT,
			source_mission_id TEXT,
			organization_id TEXT,
			agent_id TEXT,
			source_type TEXT,
			processed_at TEXT,
			created_at TEXT DEFAULT CURRENT_TIMESTAMP
		);
	`)

	mux := http.NewServeMux()
	mux.HandleFunc("/api/v1/autodream/query", s.handleAutoDreamQuery)

	body := []byte(`{"query_text": "What is the hybrid architecture?", "limit": 2}`)
	req := httptest.NewRequest("POST", "/api/v1/autodream/query", bytes.NewBuffer(body))

	// Inject claims
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org", Roles: []string{"system"}})
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v body %s", status, http.StatusOK, rr.Body.String())
	}

	var resp AutoDreamQueryResult
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v body %s", err, rr.Body.String())
	}

	// It may be empty because we didn't insert any records, but it should not fail
	if resp.Results == nil {
		// allow nil if empty in tests
	}
}

func TestHandleAutoDreamQuery_Errors(t *testing.T) {
	s := &Server{}
	mux := http.NewServeMux()
	mux.HandleFunc("/api/v1/autodream/query", s.handleAutoDreamQuery)

	// Test 1: Wrong method
	req := httptest.NewRequest("GET", "/api/v1/autodream/query", nil)
	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)
	if rr.Code != http.StatusMethodNotAllowed {
		t.Errorf("expected 405 Method Not Allowed, got %v", rr.Code)
	}

	// Test 2: Missing auth
	req = httptest.NewRequest("POST", "/api/v1/autodream/query", bytes.NewBuffer([]byte(`{"query_text": "hello"}`)))
	rr = httptest.NewRecorder()
	mux.ServeHTTP(rr, req)
	if rr.Code != http.StatusUnauthorized {
		t.Errorf("expected 401 Unauthorized, got %v", rr.Code)
	}

	// Test 3: Missing query_text
	req = httptest.NewRequest("POST", "/api/v1/autodream/query", bytes.NewBuffer([]byte(`{}`)))
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org", Roles: []string{"system"}})
	req = req.WithContext(ctx)
	rr = httptest.NewRecorder()
	mux.ServeHTTP(rr, req)
	if rr.Code != http.StatusBadRequest {
		t.Errorf("expected 400 Bad Request, got %v", rr.Code)
	}
}
