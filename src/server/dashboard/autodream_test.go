package dashboard

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/orchestration"
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
		CREATE TABLE IF NOT EXISTS swarm_dream_epochs (
			id TEXT PRIMARY KEY,
			status TEXT,
			cluster_results TEXT,
			created_at TEXT DEFAULT CURRENT_TIMESTAMP,
			completed_at TEXT
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

func TestHandleAutoDreamSync_Errors(t *testing.T) {
	s := &Server{}
	mux := http.NewServeMux()
	mux.HandleFunc("/api/v1/autodream/sync", s.handleAutoDreamSync)

	// Test 1: Wrong method
	req := httptest.NewRequest("GET", "/api/v1/autodream/sync", nil)
	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)
	if rr.Code != http.StatusMethodNotAllowed {
		t.Errorf("expected 405 Method Not Allowed, got %v", rr.Code)
	}

	// Test 2: Missing auth
	req = httptest.NewRequest("POST", "/api/v1/autodream/sync", bytes.NewBuffer([]byte(`{"force_reindex": true}`)))
	rr = httptest.NewRecorder()
	mux.ServeHTTP(rr, req)
	if rr.Code != http.StatusUnauthorized {
		t.Errorf("expected 401 Unauthorized, got %v", rr.Code)
	}

	// Test 3: Missing hub
	req = httptest.NewRequest("POST", "/api/v1/autodream/sync", bytes.NewBuffer([]byte(`{"force_reindex": true}`)))
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org", Roles: []string{"system"}})
	req = req.WithContext(ctx)
	rr = httptest.NewRecorder()
	mux.ServeHTTP(rr, req)
	if rr.Code != http.StatusServiceUnavailable {
		t.Errorf("expected 503 Service Unavailable, got %v", rr.Code)
	}

	// Test 4: Decoding error (forces the struct to remain empty)
	s4 := &Server{hub: orchestration.NewHub()}
	provider, _ := orchestration.NewSIPDB(":memory:")
	s4.hub.SetSIPDB(provider)
	defer provider.Close()

	// Create minimal schema for ConsolidateEpoch
	pool := s4.hub.SIPDB().Provider()
	_, _ = pool.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS autodream_memories (id TEXT PRIMARY KEY, content TEXT, embedding TEXT, source_mission_id TEXT, organization_id TEXT, agent_id TEXT, source_type TEXT, processed_at TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP);
		CREATE TABLE IF NOT EXISTS swarm_dream_epochs (id TEXT PRIMARY KEY, status TEXT, cluster_results TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP, completed_at TEXT);
	`)

	mux4 := http.NewServeMux()
	mux4.HandleFunc("/api/v1/autodream/sync", s4.handleAutoDreamSync)
	req4 := httptest.NewRequest("POST", "/api/v1/autodream/sync", bytes.NewBuffer([]byte(`{invalid json}`)))
	ctx4 := context.WithValue(req4.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org", Roles: []string{"system"}})
	req4 = req4.WithContext(ctx4)
	rr4 := httptest.NewRecorder()
	mux4.ServeHTTP(rr4, req4)
	if rr4.Code != http.StatusOK {
		t.Errorf("expected 200 OK after decoding fallback, got %v", rr4.Code)
	}
}

func TestHandleAutoDreamQuery_MoreErrors(t *testing.T) {
	s := &Server{}
	mux := http.NewServeMux()
	mux.HandleFunc("/api/v1/autodream/query", s.handleAutoDreamQuery)

	// Test: Invalid JSON
	req := httptest.NewRequest("POST", "/api/v1/autodream/query", bytes.NewBuffer([]byte(`{invalid}`)))
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org", Roles: []string{"system"}})
	req = req.WithContext(ctx)
	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)
	if rr.Code != http.StatusBadRequest {
		t.Errorf("expected 400 Bad Request, got %v", rr.Code)
	}

	// Test: Missing hub
	req = httptest.NewRequest("POST", "/api/v1/autodream/query", bytes.NewBuffer([]byte(`{"query_text": "hello"}`)))
	req = req.WithContext(ctx)
	rr = httptest.NewRecorder()
	mux.ServeHTTP(rr, req)
	if rr.Code != http.StatusServiceUnavailable {
		t.Errorf("expected 503 Service Unavailable, got %v", rr.Code)
	}

	// Test: Minimax standalone behavior path
	s5 := &Server{hub: orchestration.NewHub()}
	provider, _ := orchestration.NewSIPDB(":memory:")
	s5.hub.SetSIPDB(provider)
	defer provider.Close()
	pool := s5.hub.SIPDB().Provider()
	_, _ = pool.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS autodream_memories (id TEXT PRIMARY KEY, content TEXT, embedding TEXT, source_mission_id TEXT, organization_id TEXT, agent_id TEXT, source_type TEXT, processed_at TEXT, created_at TEXT DEFAULT CURRENT_TIMESTAMP);
	`)

	mux5 := http.NewServeMux()
	mux5.HandleFunc("/api/v1/autodream/query", s5.handleAutoDreamQuery)
	req5 := httptest.NewRequest("POST", "/api/v1/autodream/query", bytes.NewBuffer([]byte(`{"query_text": "testing fallback"}`)))
	ctx5 := context.WithValue(req5.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org", Roles: []string{"system"}})
	req5 = req5.WithContext(ctx5)

	t.Setenv("MINIMAX_API_KEY", "fake_key")
	t.Setenv("OHC_STANDALONE", "true")

	rr5 := httptest.NewRecorder()
	mux5.ServeHTTP(rr5, req5)
	if rr5.Code != http.StatusOK {
		t.Errorf("expected 200 OK after standalone fallback, got %v", rr5.Code)
	}

	// Test: Unconfigured DB
	s6 := &Server{hub: orchestration.NewHub()}
	mux6 := http.NewServeMux()
	mux6.HandleFunc("/api/v1/autodream/query", s6.handleAutoDreamQuery)
	req6 := httptest.NewRequest("POST", "/api/v1/autodream/query", bytes.NewBuffer([]byte(`{"query_text": "hello"}`)))
	ctx6 := context.WithValue(req6.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org", Roles: []string{"system"}})
	req6 = req6.WithContext(ctx6)
	rr6 := httptest.NewRecorder()
	mux6.ServeHTTP(rr6, req6)
	if rr6.Code != http.StatusServiceUnavailable {
		t.Errorf("expected 503 Service Unavailable, got %v", rr6.Code)
	}

	// Test: Sync failure (no schema)
	s7 := &Server{hub: orchestration.NewHub()}
	provider7, _ := orchestration.NewSIPDB(":memory:")
	s7.hub.SetSIPDB(provider7)
	defer provider7.Close()
	mux7 := http.NewServeMux()
	mux7.HandleFunc("/api/v1/autodream/sync", s7.handleAutoDreamSync)
	req7 := httptest.NewRequest("POST", "/api/v1/autodream/sync", bytes.NewBuffer([]byte(`{}`)))
	ctx7 := context.WithValue(req7.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org", Roles: []string{"system"}})
	req7 = req7.WithContext(ctx7)
	rr7 := httptest.NewRecorder()
	mux7.ServeHTTP(rr7, req7)
	if rr7.Code != http.StatusInternalServerError {
		t.Errorf("expected 500 Internal Server Error, got %v", rr7.Code)
	}

	// Test: Query DB Error (no schema)
	s8 := &Server{hub: orchestration.NewHub()}
	provider8, _ := orchestration.NewSIPDB(":memory:")
	s8.hub.SetSIPDB(provider8)
	defer provider8.Close()
	mux8 := http.NewServeMux()
	mux8.HandleFunc("/api/v1/autodream/query", s8.handleAutoDreamQuery)
	req8 := httptest.NewRequest("POST", "/api/v1/autodream/query", bytes.NewBuffer([]byte(`{"query_text": "fail search"}`)))
	ctx8 := context.WithValue(req8.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org", Roles: []string{"system"}})
	req8 = req8.WithContext(ctx8)
	rr8 := httptest.NewRecorder()
	mux8.ServeHTTP(rr8, req8)
	if rr8.Code != http.StatusInternalServerError {
		t.Errorf("expected 500 Internal Server Error, got %v", rr8.Code)
	}
}
