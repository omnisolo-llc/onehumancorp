package hybridfsmcp

import (
	"context"
	"testing"
)

type MockFSProvider struct {
	readErr   error
	writeErr  error
	listErr   error
	searchErr error
	data      []byte
	files     []string
}

func (m *MockFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	if m.readErr != nil {
		return nil, m.readErr
	}
	return m.data, nil
}

func (m *MockFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	return m.writeErr
}

func (m *MockFSProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	if m.listErr != nil {
		return nil, m.listErr
	}
	return m.files, nil
}

func (m *MockFSProvider) SearchFiles(ctx context.Context, query string) ([]string, error) {
	if m.searchErr != nil {
		return nil, m.searchErr
	}
	return m.files, nil
}

func TestHybridFSMCP_ListTools(t *testing.T) {
	mcp := NewHybridFSMCP(&MockFSProvider{})
	tools := mcp.ListTools()

	if len(tools) != 4 {
		t.Fatalf("Expected 4 tools, got %d", len(tools))
	}
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	mockProvider := &MockFSProvider{
		data:  []byte("hello world"),
		files: []string{"file1.txt", "file2.txt"},
	}
	mcp := NewHybridFSMCP(mockProvider)
	ctx := context.Background()

	// Test read_file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); !ok || resMap["data"] != "hello world" {
		t.Fatalf("read_file returned unexpected data: %v", res)
	}

	// Test write_file
	res, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "data": "new data"})
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); !ok || resMap["status"] != "success" {
		t.Fatalf("write_file returned unexpected status: %v", res)
	}

	// Test list_directory
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "."})
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}
	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("list_directory returned unexpected type")
	}
	files, ok := resMap["files"].([]string)
	if !ok || len(files) != 2 || files[0] != "file1.txt" {
		t.Fatalf("list_directory returned unexpected files: %v", files)
	}

	// Test search_files
	res, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{"query": "file"})
	if err != nil {
		t.Fatalf("search_files failed: %v", err)
	}
	resMap, ok = res.(map[string]interface{})
	if !ok {
		t.Fatalf("search_files returned unexpected type")
	}
	files, ok = resMap["files"].([]string)
	if !ok || len(files) != 2 || files[0] != "file1.txt" {
		t.Fatalf("search_files returned unexpected files: %v", files)
	}
}
