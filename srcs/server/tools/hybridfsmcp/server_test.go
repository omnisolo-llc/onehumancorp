package hybridfsmcp

import (
	"context"
	"testing"
)

type MockProvider struct {
	readData []byte
	readErr  error
	writeErr error
	listData []string
	listErr  error
	searchData []string
	searchErr  error
}

func (m *MockProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	return m.readData, m.readErr
}

func (m *MockProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	return m.writeErr
}

func (m *MockProvider) ListDir(ctx context.Context, path string) ([]string, error) {
	return m.listData, m.listErr
}

func (m *MockProvider) SearchFiles(ctx context.Context, root string, pattern string) ([]string, error) {
	return m.searchData, m.searchErr
}

func (m *MockProvider) IsLocal() bool {
	return true
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	mock := &MockProvider{
		readData: []byte("content"),
		listData: []string{"file1.txt"},
		searchData: []string{"file1.txt"},
	}
	mcp := NewHybridFSMCP(mock)
	ctx := context.Background()

	// ListTools
	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("Expected 4 tools, got %d", len(tools))
	}

	// read_file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil {
		t.Errorf("read_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "content" {
		t.Errorf("read_file content mismatch")
	}

	// write_file
	res, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "content": "data"})
	if err != nil {
		t.Errorf("write_file failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("write_file status mismatch")
	}

	// list_directory
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "."})
	if err != nil {
		t.Errorf("list_directory failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	entries := resMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "file1.txt" {
		t.Errorf("list_directory entries mismatch")
	}

	// search_files
	res, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{"root": ".", "pattern": ".*"})
	if err != nil {
		t.Errorf("search_files failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	matches := resMap["matches"].([]string)
	if len(matches) != 1 || matches[0] != "file1.txt" {
		t.Errorf("search_files matches mismatch")
	}

	// unknown tool
	_, err = mcp.CallTool(ctx, "unknown", map[string]interface{}{})
	if err == nil {
		t.Errorf("Expected error for unknown tool")
	}
}
