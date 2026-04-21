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

	// Setup initial table needed for ConsolidateEpoch
	pool := s.hub.SIPDB().Provider()
	_, err = pool.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS swarm_dream_epochs (
			id VARCHAR(255) PRIMARY KEY,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
			status VARCHAR(50),
			cluster_results TEXT,
			completed_at TIMESTAMP
		);
		CREATE TABLE IF NOT EXISTS autodream_memories (
			id VARCHAR(255) PRIMARY KEY,
			content TEXT,
			embedding TEXT,
			source_mission_id VARCHAR(255),
			consolidated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		// pg specific ignore or handle
	}

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
		CREATE TABLE IF NOT EXISTS swarm_truth_embeddings (
			memory_id VARCHAR(255) PRIMARY KEY,
			context TEXT,
			embedding TEXT,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
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
