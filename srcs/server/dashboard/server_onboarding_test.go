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
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
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
	if len(checklist) != 2 {
		t.Errorf("Expected 2 checklist items for standalone, got %v", len(checklist))
	}

	item1 := checklist[0].(map[string]interface{})
	if item1["id"] != "sqlite_db" {
		t.Errorf("Expected sqlite_db checklist item, got %v", item1["id"])
	}

	item2 := checklist[1].(map[string]interface{})
	if item2["id"] != "sqlite_standalone" {
		t.Errorf("Expected sqlite_standalone checklist item, got %v", item2["id"])
	}
}
func TestHandleHybridHealthCheck_Cloud(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
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
		t.Errorf("Expected 2 checklist items for cloud, got %v", len(checklist))
	}

	item1 := checklist[0].(map[string]interface{})
	if item1["id"] != "postgres_db" {
		t.Errorf("Expected postgres_db checklist item, got %v", item1["id"])
	}

	item2 := checklist[1].(map[string]interface{})
	if item2["id"] != "redis_cache" {
		t.Errorf("Expected redis_cache checklist item, got %v", item2["id"])
	}
}