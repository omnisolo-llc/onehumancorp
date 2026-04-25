package searchmcp

import (
	"context"
	"errors"
	"os"
	"reflect"
	"testing"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/db"
)

// mockRows implements db.Rows for testing
type mockRows struct {
	rows [][]interface{}
	idx  int
	err  error
}

func (m *mockRows) Next() bool {
	if m.idx < len(m.rows) {
		m.idx++
		return true
	}
	return false
}

func (m *mockRows) Scan(dest ...interface{}) error {
	if m.err != nil {
		return m.err
	}
	row := m.rows[m.idx-1]
	for i, v := range row {
		ptr := dest[i].(*string)
		*ptr = v.(string)
	}
	return nil
}

func (m *mockRows) Close() {}
func (m *mockRows) Columns() ([]string, error) { return []string{"id", "title", "content"}, nil }
func (m *mockRows) Err() error { return m.err }

// mockProvider implements db.Provider for testing
type mockProvider struct {
	db.Provider
	execFunc  func(ctx context.Context, sql string, arguments ...any) (int64, error)
	queryFunc func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error)
}

func (m *mockProvider) Exec(ctx context.Context, sql string, arguments ...any) (int64, error) {
	if m.execFunc != nil {
		return m.execFunc(ctx, sql, arguments...)
	}
	return 0, nil
}

func (m *mockProvider) Query(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
	if m.queryFunc != nil {
		return m.queryFunc(ctx, sql, optionsAndArgs...)
	}
	return &mockRows{}, nil
}

func TestLocalSearchProvider(t *testing.T) {
	mp := &mockProvider{
		queryFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
			if sql != "SELECT id, title, content FROM local_search_index WHERE local_search_index MATCH ? ORDER BY rank LIMIT 10" {
				return nil, errors.New("unexpected query")
			}
			return &mockRows{
				rows: [][]interface{}{
					{"1", "Doc 1", "Content 1"},
					{"2", "Doc 2", "Content 2"},
				},
			}, nil
		},
		execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
			if sql != "INSERT INTO local_search_index(id, title, content) VALUES(?, ?, ?)" {
				return 0, errors.New("unexpected query")
			}
			return 1, nil
		},
	}

	p := NewLocalSearchProvider(mp)

	// Test Search
	results, err := p.Search(context.Background(), "query")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(results) != 2 {
		t.Fatalf("expected 2 results, got %d", len(results))
	}
	if results[0].ID != "1" || results[1].ID != "2" {
		t.Errorf("unexpected results: %+v", results)
	}

	// Test Index
	err = p.Index(context.Background(), Document{ID: "3", Title: "Doc 3", Content: "Content 3"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestCloudSearchProvider(t *testing.T) {
	mp := &mockProvider{
		queryFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
			if sql != "SELECT id, title, content FROM cloud_search_index WHERE tenant_id = $1 AND content ILIKE $2 LIMIT 10" {
				return nil, errors.New("unexpected query")
			}
			if len(optionsAndArgs) != 2 || optionsAndArgs[0] != "org1" || optionsAndArgs[1] != "%query%" {
				return nil, errors.New("unexpected args")
			}
			return &mockRows{
				rows: [][]interface{}{
					{"1", "Doc 1", "Content 1"},
				},
			}, nil
		},
		execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
			if sql != "INSERT INTO cloud_search_index(tenant_id, id, title, content) VALUES($1, $2, $3, $4)" {
				return 0, errors.New("unexpected query")
			}
			if len(arguments) != 4 || arguments[0] != "org1" {
				return 0, errors.New("unexpected args")
			}
			return 1, nil
		},
	}

	p := NewCloudSearchProvider(mp)

	// Test Search unauthorized
	_, err := p.Search(context.Background(), "query")
	if err == nil {
		t.Fatal("expected unauthorized error")
	}

	// Test Search authorized
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	results, err := p.Search(ctx, "query")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(results) != 1 {
		t.Fatalf("expected 1 result, got %d", len(results))
	}
	if results[0].ID != "1" {
		t.Errorf("unexpected result: %+v", results)
	}

	// Test Index unauthorized
	err = p.Index(context.Background(), Document{})
	if err == nil {
		t.Fatal("expected unauthorized error")
	}

	// Test Index authorized
	err = p.Index(ctx, Document{ID: "3", Title: "Doc 3", Content: "Content 3"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestHybridSearchMCP(t *testing.T) {
	mp := &mockProvider{
		queryFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
			return &mockRows{
				rows: [][]interface{}{
					{"1", "Doc 1", "Content 1"},
				},
			}, nil
		},
		execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
			return 1, nil
		},
	}

	p := NewLocalSearchProvider(mp)
	mcpServer := NewHybridSearchMCP(p)

	// Test ListTools
	tools := mcpServer.ListTools()
	if len(tools) != 2 {
		t.Fatalf("expected 2 tools, got %d", len(tools))
	}

	// Test unified_search tool
	res, err := mcpServer.CallTool(context.Background(), "unified_search", map[string]interface{}{"query": "test"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap := res.(map[string]interface{})
	results := resMap["results"].([]SearchResult)
	if len(results) != 1 {
		t.Fatalf("expected 1 result, got %d", len(results))
	}

	// Test index_document tool
	res, err = mcpServer.CallTool(context.Background(), "index_document", map[string]interface{}{"id": "1", "title": "t", "content": "c"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Fatalf("expected success, got %v", resMap["status"])
	}
}

func TestNewProviderFactory(t *testing.T) {
	mp := &mockProvider{}

	os.Setenv("OHC_STANDALONE", "true")
	p := NewProviderFactory(mp)
	if reflect.TypeOf(p) != reflect.TypeOf(&LocalSearchProvider{}) {
		t.Fatalf("expected LocalSearchProvider, got %T", p)
	}

	os.Setenv("OHC_STANDALONE", "false")
	p = NewProviderFactory(mp)
	if reflect.TypeOf(p) != reflect.TypeOf(&CloudSearchProvider{}) {
		t.Fatalf("expected CloudSearchProvider, got %T", p)
	}
	os.Unsetenv("OHC_STANDALONE")
}

func TestHybridSearchMCP_Errors(t *testing.T) {
	mp := &mockProvider{
		queryFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
			return nil, errors.New("db error")
		},
		execFunc: func(ctx context.Context, sql string, arguments ...any) (int64, error) {
			return 0, errors.New("db error")
		},
	}

	p := NewLocalSearchProvider(mp)
	mcpServer := NewHybridSearchMCP(p)

	// unified_search invalid query
	_, err := mcpServer.CallTool(context.Background(), "unified_search", map[string]interface{}{"query": 123})
	if err == nil || err.Error() != "missing or invalid 'query' argument" {
		t.Fatalf("expected error missing or invalid 'query' argument, got %v", err)
	}

	// unified_search db error
	_, err = mcpServer.CallTool(context.Background(), "unified_search", map[string]interface{}{"query": "test"})
	if err == nil || err.Error() != "db error" {
		t.Fatalf("expected db error, got %v", err)
	}

	// index_document invalid id
	_, err = mcpServer.CallTool(context.Background(), "index_document", map[string]interface{}{"id": 123})
	if err == nil || err.Error() != "missing or invalid 'id' argument" {
		t.Fatalf("expected missing or invalid 'id' argument, got %v", err)
	}

	// index_document invalid content
	_, err = mcpServer.CallTool(context.Background(), "index_document", map[string]interface{}{"id": "1", "content": 123})
	if err == nil || err.Error() != "missing or invalid 'content' argument" {
		t.Fatalf("expected missing or invalid 'content' argument, got %v", err)
	}

	// index_document db error
	_, err = mcpServer.CallTool(context.Background(), "index_document", map[string]interface{}{"id": "1", "content": "c"})
	if err == nil || err.Error() != "db error" {
		t.Fatalf("expected db error, got %v", err)
	}

	// unknown tool
	_, err = mcpServer.CallTool(context.Background(), "unknown_tool", nil)
	if err == nil || err.Error() != "unknown tool: unknown_tool" {
		t.Fatalf("expected unknown tool error, got %v", err)
	}
}

func TestEnvBoolDefault(t *testing.T) {
	os.Setenv("TEST_KEY", "true")
	if !envBoolDefault("TEST_KEY", false) {
		t.Fatal("expected true")
	}

	os.Setenv("TEST_KEY", "1")
	if !envBoolDefault("TEST_KEY", false) {
		t.Fatal("expected true")
	}

	os.Setenv("TEST_KEY", "false")
	if envBoolDefault("TEST_KEY", true) {
		t.Fatal("expected false")
	}

	os.Unsetenv("TEST_KEY")
	if envBoolDefault("TEST_KEY", true) != true {
		t.Fatal("expected true fallback")
	}
}

func TestLocalSearchProvider_Errors(t *testing.T) {
	mp := &mockProvider{
		queryFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
			return nil, errors.New("query error")
		},
	}
	p := NewLocalSearchProvider(mp)
	_, err := p.Search(context.Background(), "query")
	if err == nil || err.Error() != "query error" {
		t.Fatalf("expected query error, got %v", err)
	}
}

func TestCloudSearchProvider_Errors(t *testing.T) {
	mp := &mockProvider{
		queryFunc: func(ctx context.Context, sql string, optionsAndArgs ...any) (db.Rows, error) {
			return nil, errors.New("query error")
		},
	}
	p := NewCloudSearchProvider(mp)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org1"})
	_, err := p.Search(ctx, "query")
	if err == nil || err.Error() != "query error" {
		t.Fatalf("expected query error, got %v", err)
	}
}
