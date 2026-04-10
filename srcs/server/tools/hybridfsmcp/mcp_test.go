package hybridfsmcp

import (
    "context"
    "testing"
)

type MockProvider struct{}

func (m *MockProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
    return []byte("mock data"), nil
}
func (m *MockProvider) WriteFile(ctx context.Context, path string, data []byte) error {
    return nil
}
func (m *MockProvider) ListDir(ctx context.Context, path string) ([]string, error) {
    return []string{"mock.txt"}, nil
}

func TestHybridFSMCP(t *testing.T) {
    mcp := NewHybridFSMCP(&MockProvider{})
    tools := mcp.ListTools()
    if len(tools) != 3 {
        t.Errorf("expected 3 tools, got %d", len(tools))
    }

    ctx := context.Background()

    // Read
    res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    resMap := res.(map[string]interface{})
    if resMap["content"] != "mock data" {
        t.Errorf("expected mock data, got %v", resMap["content"])
    }

    // Write
    res, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "content": "mock data"})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    resMap = res.(map[string]interface{})
    if resMap["status"] != "success" {
        t.Errorf("expected success, got %v", resMap["status"])
    }

    // List
    res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "."})
    if err != nil {
        t.Fatalf("unexpected error: %v", err)
    }
    resMap = res.(map[string]interface{})
    entries := resMap["entries"].([]string)
    if entries[0] != "mock.txt" {
        t.Errorf("expected mock.txt, got %v", entries)
    }
}
