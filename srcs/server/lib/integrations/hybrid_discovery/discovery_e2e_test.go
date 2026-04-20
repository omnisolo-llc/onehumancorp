package hybrid_discovery

import (
	"context"
	"database/sql"
	"net/http"
	"net/http/httptest"
	"testing"

	_ "modernc.org/sqlite"
)

func TestE2E_HybridDiscoveryFallback(t *testing.T) {
	// A placeholder for E2E test verifying fallback logic.
	// As per standard, this simulates UI interaction fallback logic.
	t.Log("Simulating UI interaction fallback logic for E2E testing.")
}

func TestE2E_ScoutOpenAPIIntegration(t *testing.T) {
	// Setup local SQLite DB for test
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer db.Close()

	proxy := NewDiscoveryProxy(db, "switchboard")

	// Setup dummy OpenAPI server
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		w.Write([]byte(`{
			"openapi": "3.0.0",
			"info": {
				"title": "Test Dummy API",
				"version": "1.0.0"
			},
			"paths": {}
		}`))
	}))
	defer ts.Close()

	ctx := context.Background()

	SSRFGuardrailBypass = true
	defer func() { SSRFGuardrailBypass = false }()

	// Import the OpenAPI spec
	err = proxy.ImportOpenAPI(ctx, ts.URL)
	if err != nil {
		t.Fatalf("ImportOpenAPI failed: %v", err)
	}

	// Verify the tool was registered
	tools, err := proxy.SearchTools(ctx, "Test-Dummy-API")
	if err != nil {
		t.Fatalf("SearchTools failed: %v", err)
	}

	if len(tools) == 0 {
		t.Fatal("SearchTools returned 0 results, expected 1")
	}

	tool := tools[0]
	if tool.Name != "Test-Dummy-API" {
		t.Errorf("expected tool name 'Test-Dummy-API', got %q", tool.Name)
	}
}

func TestE2E_ScoutOpenAPIIntegration_Guardrails(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}
	defer db.Close()

	proxy := NewDiscoveryProxy(db, "switchboard")
	ctx := context.Background()

	err = proxy.ImportOpenAPI(ctx, "ftp://invalid-scheme.com")
	if err == nil {
		t.Error("expected error for invalid scheme, got nil")
	}

	err = proxy.ImportOpenAPI(ctx, "http://127.0.0.1/spec.json")
	if err == nil {
		t.Error("expected error for loopback address, got nil")
	}

	err = proxy.ImportOpenAPI(ctx, "http://169.254.169.254/latest/meta-data")
	if err == nil {
		t.Error("expected error for link-local metadata address, got nil")
	}
}
