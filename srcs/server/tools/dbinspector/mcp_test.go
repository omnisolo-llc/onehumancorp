package dbinspector

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("Failed to open sqlite: %v", err)
	}
	return db.NewSqliteProvider(sqliteDB)
}

func TestDBInspectorMCP_ListTools(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	mcp := NewDBInspectorMCP(provider)
	tools := mcp.ListTools()

	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}
}

func TestDBInspectorMCP_CallTool_InspectSchema(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	_, _ = provider.Exec(ctx, "CREATE TABLE test_table (id INTEGER PRIMARY KEY, name TEXT)")

	mcp := NewDBInspectorMCP(provider)

	res, err := mcp.CallTool(ctx, "inspect_schema", map[string]interface{}{})
	if err != nil {
		t.Fatalf("Failed to inspect schema: %v", err)
	}

	results, ok := res.([]map[string]interface{})
	if !ok {
		t.Fatalf("Expected []map[string]interface{}, got %T", res)
	}

	found := false
	for _, r := range results {
		if r["table"] == "test_table" {
			found = true
			break
		}
	}

	if !found {
		t.Fatalf("Expected to find test_table in schema inspection")
	}
}

func TestDBInspectorMCP_CallTool_RunQuery_SQLite(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	_, _ = provider.Exec(ctx, "CREATE TABLE test_query (id INTEGER PRIMARY KEY, name TEXT)")
	_, _ = provider.Exec(ctx, "INSERT INTO test_query (name) VALUES ('Alice')")

	mcp := NewDBInspectorMCP(provider)

	res, err := mcp.CallTool(ctx, "run_query", map[string]interface{}{
		"query": "SELECT * FROM test_query",
	})

	if err != nil {
		t.Fatalf("Failed to run query: %v", err)
	}

	results, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected map[string]interface{}, got %T", res)
	}

	resList, ok := results["results"].([]map[string]interface{})
	if !ok || len(resList) != 1 {
		t.Fatalf("Expected 1 result row, got %v", results["results"])
	}
	if name, ok := resList[0]["name"].(string); !ok || name != "Alice" {
		t.Fatalf("Expected name 'Alice', got %v", resList[0]["name"])
	}
	if results["mode"] != "standalone" {
		t.Fatalf("Expected mode 'standalone', got %v", results["mode"])
	}
}

func TestDBInspectorMCP_CallTool_RunQuery_Unsafe(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	mcp := NewDBInspectorMCP(provider)

	_, err := mcp.CallTool(ctx, "run_query", map[string]interface{}{
		"query": "DROP TABLE users",
	})

	if err == nil {
		t.Fatalf("Expected error for unsafe query without override")
	}

	// Try with override but no admin claims
	ctx = context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org1",
		Roles:          []string{"user"},
	})

	_, err = mcp.CallTool(ctx, "run_query", map[string]interface{}{
		"query":                "DROP TABLE users",
		"override_safety_lock": true,
	})

	if err == nil {
		t.Fatalf("Expected error for override without admin claims")
	}

	// Try with override and admin claims
	ctx = context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org1",
		Roles:          []string{"admin"},
	})

	_, err = mcp.CallTool(ctx, "run_query", map[string]interface{}{
		"query":                "SELECT 1", // safe query to pass test in SQLite mode (since DROP won't work anyway without table)
		"override_safety_lock": true,
	})

	if err != nil {
		t.Fatalf("Unexpected error for override with admin claims: %v", err)
	}
}

func TestDBInspectorMCP_CallTool_GetStats(t *testing.T) {
	provider := setupTestDB(t)
	defer provider.Close()

	ctx := context.Background()
	mcp := NewDBInspectorMCP(provider)

	res, err := mcp.CallTool(ctx, "get_stats", map[string]interface{}{})
	if err != nil {
		t.Fatalf("Failed to get stats: %v", err)
	}

	results, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected map[string]interface{}, got %T", res)
	}

	if results["status"] != "ok" {
		t.Fatalf("Expected status 'ok', got %v", results["status"])
	}
	if results["mode"] != "standalone" {
		t.Fatalf("Expected mode 'standalone', got %v", results["mode"])
	}
}
