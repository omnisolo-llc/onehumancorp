package hybridcrdtmcp

import (
	"context"
	"os"
	"testing"

	"github.com/mark3labs/mcp-go/mcp"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type mockDB struct {
    db.Provider
    queryRowFunc func(ctx context.Context, query string, args ...interface{}) db.Row
    execFunc func(ctx context.Context, query string, args ...interface{}) (int64, error)
}

func (m *mockDB) QueryRow(ctx context.Context, query string, args ...interface{}) db.Row {
    return m.queryRowFunc(ctx, query, args...)
}

func (m *mockDB) Exec(ctx context.Context, query string, args ...interface{}) (int64, error) {
    return m.execFunc(ctx, query, args...)
}

type mockRow struct {
    scanFunc func(dest ...interface{}) error
}

func (m mockRow) Scan(dest ...interface{}) error {
    return m.scanFunc(dest...)
}

func TestHandleMerge(t *testing.T) {
	tool := NewCRDTTool(nil)

	req := mcp.CallToolRequest{}
	req.Params.Arguments = map[string]interface{}{
		"vector_a": `{"a": 1, "b": 2}`,
		"vector_b": `{"b": 3, "c": 1}`,
	}

	res, err := tool.handleMerge(context.Background(), req)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if res.IsError {
		t.Fatalf("unexpected error result: %v", res.Content)
	}

	content := res.Content[0].(mcp.TextContent).Text
	expected := `{"a":1,"b":3,"c":1}`
	if content != expected {
		t.Errorf("expected %s, got %s", expected, content)
	}
}

func TestCheckAuth(t *testing.T) {
    os.Setenv("OHC_MULTITENANT", "true")
    defer os.Unsetenv("OHC_MULTITENANT")

    tool := NewCRDTTool(nil)

    _, err := tool.checkAuth(context.Background())
    if err == nil {
        t.Error("expected error for missing org id in multitenant mode")
    }

    ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})
    orgID, err := tool.checkAuth(ctx)
    if err != nil {
        t.Errorf("unexpected error: %v", err)
    }
    if orgID != "test-org" {
        t.Errorf("expected test-org, got %s", orgID)
    }
}
