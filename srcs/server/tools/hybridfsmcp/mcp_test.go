package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type mockProvider struct {
	readPath    string
	writePath   string
	writeContent []byte
}

func (m *mockProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	m.readPath = path
	return []byte("mock data"), nil
}

func (m *mockProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	m.writePath = path
	m.writeContent = data
	return nil
}

func (m *mockProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	return []string{"file1.txt"}, nil
}

func (m *mockProvider) SearchFiles(ctx context.Context, claims *auth.Claims, path string, pattern string) ([]string, error) {
	return []string{"match.txt"}, nil
}

func TestHybridFSMCP_ListTools(t *testing.T) {
	server := NewHybridFSMCPServer(NewLocalFSProvider("."))
	tools := server.ListTools()

	if len(tools) != 4 {
		t.Fatalf("expected 4 tools, got %d", len(tools))
	}

	expectedNames := []string{"read_file", "write_file", "list_directory", "search_files"}
	for i, name := range expectedNames {
		if tools[i].Name != name {
			t.Errorf("expected tool name %s, got %s", name, tools[i].Name)
		}
	}
}

func TestHybridFSMCP_CallTool_ReadFile(t *testing.T) {
	mock := &mockProvider{}
	server := NewHybridFSMCPServer(mock)

	args := map[string]interface{}{
		"path": "test.txt",
	}

	result, err := server.CallTool(context.Background(), "read_file", args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mock.readPath != "test.txt" {
		t.Errorf("expected provider to be called with path 'test.txt', got %s", mock.readPath)
	}

	resMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatalf("expected result to be a map")
	}

	if resMap["content"] != "mock data" {
		t.Errorf("expected content 'mock data', got %v", resMap["content"])
	}
}

func TestHybridFSMCP_CallTool_WriteFile(t *testing.T) {
	mock := &mockProvider{}
	server := NewHybridFSMCPServer(mock)

	args := map[string]interface{}{
		"path":    "test.txt",
		"content": "new data",
	}

	result, err := server.CallTool(context.Background(), "write_file", args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if mock.writePath != "test.txt" {
		t.Errorf("expected provider to be called with path 'test.txt', got %s", mock.writePath)
	}

	if string(mock.writeContent) != "new data" {
		t.Errorf("expected provider to be called with content 'new data', got %s", string(mock.writeContent))
	}

	resMap, ok := result.(map[string]interface{})
	if !ok {
		t.Fatalf("expected result to be a map")
	}

	if resMap["status"] != "success" {
		t.Errorf("expected status 'success', got %v", resMap["status"])
	}
}
