package hybridfsmcp

import (
	"context"
	"testing"
)

type mockProvider struct {
	readFileFunc  func(ctx context.Context, path string) ([]byte, error)
	writeFileFunc func(ctx context.Context, path string, data []byte) error
	listDirFunc   func(ctx context.Context, path string) ([]string, error)
}

func (m *mockProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	return m.readFileFunc(ctx, path)
}

func (m *mockProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	return m.writeFileFunc(ctx, path, data)
}

func (m *mockProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	return m.listDirFunc(ctx, path)
}

func (m *mockProvider) IsLocal() bool { return true }

func TestHybridFSMCP_CallTool_ReadFile(t *testing.T) {
	provider := &mockProvider{
		readFileFunc: func(ctx context.Context, path string) ([]byte, error) {
			if path == "test.txt" {
				return []byte("test content"), nil
			}
			return nil, nil
		},
	}
	mcp := NewHybridFSMCP(provider)
	res, err := mcp.CallTool(context.Background(), "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	m := res.(map[string]interface{})
	if m["content"] != "test content" {
		t.Errorf("expected 'test content', got %v", m["content"])
	}
}

func TestHybridFSMCP_ListTools(t *testing.T) {
	mcp := NewHybridFSMCP(&mockProvider{})
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}
}
