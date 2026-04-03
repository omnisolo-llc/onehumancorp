package dashboard

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
)

func TestHandleHybridHealthCheck_Standalone(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	os.Setenv("DATABASE_URL", "")

	server := &Server{}
	req, _ := http.NewRequest("GET", "/api/health/hybrid", nil)
	rr := httptest.NewRecorder()

	server.handleHybridHealthCheck(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp map[string]interface{}
	if err := json.Unmarshal(rr.Body.Bytes(), &resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp["mode"] != "standalone" {
		t.Errorf("Expected mode standalone, got %v", resp["mode"])
	}

	checklist := resp["checklist"].([]interface{})
	if len(checklist) != 1 {
		t.Errorf("Expected 1 checklist item, got %v", len(checklist))
	}

	item := checklist[0].(map[string]interface{})
	if item["id"] != "sqlite_db" {
		t.Errorf("Expected sqlite_db checklist item, got %v", item["id"])
	}
}

func TestHandleHybridHealthCheck_Cloud(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")
	os.Setenv("DATABASE_URL", "postgres://user:pass@localhost:5432/db")
	defer os.Unsetenv("DATABASE_URL")
	os.Setenv("REDIS_URL", "redis://localhost:6379")
	defer os.Unsetenv("REDIS_URL")

	server := &Server{}
	req, _ := http.NewRequest("GET", "/api/health/hybrid", nil)
	rr := httptest.NewRecorder()

	server.handleHybridHealthCheck(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp map[string]interface{}
	if err := json.Unmarshal(rr.Body.Bytes(), &resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if resp["mode"] != "cloud" {
		t.Errorf("Expected mode cloud, got %v", resp["mode"])
	}

	checklist := resp["checklist"].([]interface{})
	if len(checklist) != 2 {
		t.Errorf("Expected 2 checklist items, got %v", len(checklist))
	}
}
