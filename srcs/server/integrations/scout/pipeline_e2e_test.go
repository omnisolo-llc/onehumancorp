package scout

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/lib/integrations/hybrid_discovery"
	_ "modernc.org/sqlite"
)

func TestE2E_ScoutPipeline(t *testing.T) {
	// Create a dummy API Server for the scout to discover
	mux := http.NewServeMux()
	mux.HandleFunc("/openapi.json", func(w http.ResponseWriter, r *http.Request) {
		spec := map[string]interface{}{
			"openapi": "3.0.0",
			"info": map[string]interface{}{
				"title":   "Dummy API",
				"version": "1.0.0",
			},
			"paths": map[string]interface{}{
				"/hello": map[string]interface{}{
					"get": map[string]interface{}{
						"operationId": "getHello",
						"description": "Returns a greeting",
					},
				},
				"/dangerous": map[string]interface{}{
					"delete": map[string]interface{}{
						"operationId": "delete",
						"description": "Deletes everything",
					},
				},
			},
		}
		json.NewEncoder(w).Encode(spec)
	})
	srv := httptest.NewServer(mux)
	defer srv.Close()

	// Create SQLite database for Hybrid Discovery
	dbFile := "test_scout.db"
	defer os.Remove(dbFile)

	db, err := sql.Open("sqlite", dbFile)
	if err != nil {
		t.Fatalf("Failed to open sqlite db: %v", err)
	}
	defer db.Close()

	proxy := hybrid_discovery.NewDiscoveryProxy(db, "switchboard.local")
	pipeline := NewPipeline(proxy)

	ctx := context.Background()
	err = pipeline.ParseAndRegister(ctx, srv.URL+"/openapi.json")
	if err != nil {
		t.Fatalf("ParseAndRegister failed: %v", err)
	}

	// Verify tools
	tools, err := proxy.SearchTools(ctx, "greeting")
	if err != nil {
		t.Fatalf("SearchTools failed: %v", err)
	}

	if len(tools) == 0 {
		t.Errorf("Expected to find getHello tool")
	} else if tools[0].Name != "getHello" {
		t.Errorf("Expected 'getHello', got '%s'", tools[0].Name)
	}

	// Verify dangerous tool is NOT registered
	dangerousTools, _ := proxy.SearchTools(ctx, "Deletes everything")
	if len(dangerousTools) > 0 && dangerousTools[0].Name == "delete" {
		t.Errorf("Dangerous tool 'delete' should not have been registered")
	}
}
