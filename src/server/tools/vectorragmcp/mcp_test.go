package vectorragmcp

import (
	"context"
	"testing"
    "time"

	"github.com/onehumancorp/mono/src/server/db"
)

type mockProvider struct {
	isSQLite bool
	queries  []string
    results  []string
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) { return 0, nil }
func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) { return nil, nil }
func (m *mockProvider) QueryRow(ctx context.Context, sql string, optionsAndArgs ...any) db.Row { return nil }
func (m *mockProvider) Begin(ctx context.Context) (db.Tx, error) { return nil, nil }
func (m *mockProvider) Close() {}
func (m *mockProvider) Ping(ctx context.Context) error { return nil }
func (m *mockProvider) IsSQLite() bool { return m.isSQLite }
func (m *mockProvider) AcquireTask(ctx context.Context, organizationID, agentID string) (*db.TaskRecord, error) { return nil, nil }
func (m *mockProvider) SearchMemories(ctx context.Context, organizationID string, queryText string, limit int) ([]string, error) {
    m.queries = append(m.queries, queryText)
    return m.results, nil
}

func TestVectorRAGMCP_CallTool(t *testing.T) {
	provider := &mockProvider{
		isSQLite: true,
        results: []string{"test_memory"},
	}
	mcp := NewVectorRAGMCP(provider)

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	args := map[string]interface{}{
		"query":           "test_query",
		"organization_id": "test_org",
		"limit":           float64(10),
	}

	result, err := mcp.CallTool(ctx, "semantic_search", args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	resMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatalf("unexpected result type")
	}

	results, ok := resMap["results"].([]string)
	if !ok || len(results) != 1 || results[0] != "test_memory" {
		t.Fatalf("unexpected results: %v", resMap["results"])
	}

    if len(provider.queries) != 1 || provider.queries[0] != "test_query" {
        t.Fatalf("unexpected queries passed to provider: %v", provider.queries)
    }
}
