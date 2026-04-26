package scout_test

import (
	"context"
	"database/sql"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/src/server/lib/integrations/hybrid_discovery"
	"github.com/onehumancorp/mono/src/server/lib/integrations/scout"
	_ "modernc.org/sqlite"
)

func TestScout_ParseAndRegister_E2E(t *testing.T) {
	// Start a local HTTP server that serves a dummy OpenAPI JSON response
	openAPIJSON := `{
		"paths": {
			"/users": {
				"get": {
					"operationId": "getUsers",
					"summary": "Get all users",
					"description": "Returns a list of users"
				}
			}
		}
	}`

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		w.Write([]byte(openAPIJSON))
	}))
	defer server.Close()

	ctx := context.Background()
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer db.Close()

	proxy := hybrid_discovery.NewDiscoveryProxy(db, "")
	scoutAgent := scout.NewScout(proxy)

	err = scoutAgent.ParseAndRegister(ctx, server.URL)
	if err != nil {
		t.Fatalf("ParseAndRegister failed: %v", err)
	}

	tools, err := proxy.SearchTools(ctx, "getUsers")
	if err != nil {
		t.Fatalf("SearchTools failed: %v", err)
	}

	if len(tools) == 0 {
		t.Fatalf("expected at least one tool, got 0")
	}

	found := false
	for _, tool := range tools {
		if tool.Name == "getUsers" {
			found = true
			if tool.Description != "Get all users Returns a list of users" {
				t.Errorf("unexpected description: %s", tool.Description)
			}
			expectedEndpoint := server.URL + "/users"
			if tool.Endpoint != expectedEndpoint {
				t.Errorf("unexpected endpoint: expected %s, got %s", expectedEndpoint, tool.Endpoint)
			}
		}
	}

	if !found {
		t.Errorf("getUsers tool was not found in search results")
	}
}
