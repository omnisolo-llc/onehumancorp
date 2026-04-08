package hybridfsmcp

import (
	"context"
	"os"
	"strings"
	"testing"
)

// mockProvider implements FileSystemProvider for testing MCP
type mockProvider struct {
	files map[string][]byte
}

func newMockProvider() *mockProvider {
	return &mockProvider{
		files: make(map[string][]byte),
	}
}

func (m *mockProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	if data, ok := m.files[path]; ok {
		return data, nil
	}
	return nil, os.ErrNotExist
}

func (m *mockProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	m.files[path] = data
	return nil
}

func (m *mockProvider) ListDir(ctx context.Context, path string) ([]FileInfo, error) {
	var infos []FileInfo
	for k := range m.files {
		// Mock list directory just checking if the path is a prefix or "."
		if path == "." || strings.HasPrefix(k, path) {
			infos = append(infos, FileInfo{Name: k, IsDir: false, Size: int64(len(m.files[k]))})
		}
	}
	return infos, nil
}

func (m *mockProvider) SearchFiles(ctx context.Context, path string, pattern string) ([]string, error) {
	var matches []string
	for k := range m.files {
		if (path == "." || strings.HasPrefix(k, path)) && strings.Contains(k, pattern) {
			matches = append(matches, k)
		}
	}
	return matches, nil
}

func TestMCPTools(t *testing.T) {
	mockProv := newMockProvider()
	mcp := NewHybridFSMCP(mockProv)
	ctx := context.Background()

	// 1. Write File
	writeArgs := []byte(`{"path": "test.txt", "content": "hello world"}`)
	res, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("write_file error: %v", err)
	}
	if res != "success" {
		t.Errorf("expected success, got %v", res)
	}

	// 2. Read File
	readArgs := []byte(`{"path": "test.txt"}`)
	res, err = mcp.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("read_file error: %v", err)
	}
	if res != "hello world" {
		t.Errorf("expected hello world, got %v", res)
	}

	// 3. List Directory
	listArgs := []byte(`{"path": "."}`)
	res, err = mcp.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("list_directory error: %v", err)
	}
	infos, ok := res.([]FileInfo)
	if !ok || len(infos) != 1 || infos[0].Name != "test.txt" {
		t.Errorf("unexpected list result: %v", res)
	}

	// 4. Search Files
	searchArgs := []byte(`{"path": ".", "pattern": "test"}`)
	res, err = mcp.CallTool(ctx, "search_files", searchArgs)
	if err != nil {
		t.Fatalf("search_files error: %v", err)
	}
	matches, ok := res.([]string)
	if !ok || len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("unexpected search result: %v", res)
	}

	// Invalid Tool
	_, err = mcp.CallTool(ctx, "invalid_tool", []byte(`{}`))
	if err == nil {
		t.Errorf("expected error for invalid tool")
	}

	// Invalid Args
	_, err = mcp.CallTool(ctx, "read_file", []byte(`{"path": ""}`))
	if err == nil {
		t.Errorf("expected error for empty path in read_file")
	}

	_, err = mcp.CallTool(ctx, "write_file", []byte(`{"path": ""}`))
	if err == nil {
		t.Errorf("expected error for empty path in write_file")
	}

	_, err = mcp.CallTool(ctx, "search_files", []byte(`{"path": ".", "pattern": ""}`))
	if err == nil {
		t.Errorf("expected error for empty pattern in search_files")
	}
}
