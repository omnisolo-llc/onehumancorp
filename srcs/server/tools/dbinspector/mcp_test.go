package dbinspector

import (
	"context"
	"testing"
	"strings"
	"encoding/json"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestInspectSchemaTool_SQLite(t *testing.T) {
	provider := db.NewTestProvider(t)
	// Create a dummy table to test schema output
	_, err := provider.Exec(context.Background(), "CREATE TABLE dummy (id INTEGER PRIMARY KEY, name TEXT);")
	if err != nil {
		t.Fatalf("failed to create dummy table: %v", err)
	}

	tool := &InspectSchemaTool{DB: provider}

	res, err := tool.Execute(context.Background(), "", map[string]interface{}{})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if !strings.Contains(res, "dummy") {
		t.Errorf("expected result to contain 'dummy', got %s", res)
	}
}

func TestRunQueryTool_SQLite(t *testing.T) {
	provider := db.NewTestProvider(t)
	_, _ = provider.Exec(context.Background(), "CREATE TABLE data (id INTEGER PRIMARY KEY, val TEXT);")
	_, _ = provider.Exec(context.Background(), "INSERT INTO data (val) VALUES ('test1'), ('test2');")

	tool := &RunQueryTool{DB: provider}

	// Test read query
	res, err := tool.Execute(context.Background(), "", map[string]interface{}{
		"query": "SELECT val FROM data ORDER BY val ASC",
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	var parsed []map[string]interface{}
	if err := json.Unmarshal([]byte(res), &parsed); err != nil {
		t.Fatalf("failed to parse json: %v", res)
	}

	if len(parsed) != 2 || parsed[0]["val"] != "test1" {
		t.Errorf("unexpected query result: %v", parsed)
	}

	// Test blocked mutating query
	_, err = tool.Execute(context.Background(), "", map[string]interface{}{
		"query": "DELETE FROM data;",
	})
	if err == nil || !strings.Contains(err.Error(), "READ-ONLY") {
		t.Errorf("expected READ-ONLY error, got %v", err)
	}

	// Test allowed mutating query with admin claims
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org1",
		Roles:          []string{"admin"},
	})

	res, err = tool.Execute(ctx, "", map[string]interface{}{
		"query": "DELETE FROM data;",
		"override_safety_lock": true,
	})
	if err != nil {
		t.Fatalf("unexpected error for admin override: %v", err)
	}
	if !strings.Contains(res, "Rows affected") {
		t.Errorf("expected rows affected message, got: %s", res)
	}
}

func TestGetStatsTool_SQLite(t *testing.T) {
	provider := db.NewTestProvider(t)
	tool := &GetStatsTool{DB: provider}

	res, err := tool.Execute(context.Background(), "", map[string]interface{}{})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if res == "" {
		t.Errorf("expected stats output")
	}
}
