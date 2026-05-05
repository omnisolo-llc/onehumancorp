package hybridfsmcp

import (
	"context"
	"io/fs"
	"testing"
	"time"
)

// MockProvider is a mock implementation of FileSystemProvider for testing
type MockProvider struct {
	ReadFileFunc    func(ctx context.Context, claims *Claims, path string) ([]byte, error)
	WriteFileFunc   func(ctx context.Context, claims *Claims, path string, data []byte) error
	ListDirFunc     func(ctx context.Context, claims *Claims, path string) ([]fs.FileInfo, error)
	SearchFilesFunc func(ctx context.Context, claims *Claims, query string) ([]string, error)
}

func (m *MockProvider) ReadFile(ctx context.Context, claims *Claims, path string) ([]byte, error) {
	if m.ReadFileFunc != nil {
		return m.ReadFileFunc(ctx, claims, path)
	}
	return nil, nil
}

func (m *MockProvider) WriteFile(ctx context.Context, claims *Claims, path string, data []byte) error {
	if m.WriteFileFunc != nil {
		return m.WriteFileFunc(ctx, claims, path, data)
	}
	return nil
}

func (m *MockProvider) ListDir(ctx context.Context, claims *Claims, path string) ([]fs.FileInfo, error) {
	if m.ListDirFunc != nil {
		return m.ListDirFunc(ctx, claims, path)
	}
	return nil, nil
}

func (m *MockProvider) SearchFiles(ctx context.Context, claims *Claims, query string) ([]string, error) {
	if m.SearchFilesFunc != nil {
		return m.SearchFilesFunc(ctx, claims, query)
	}
	return nil, nil
}

type MockFileInfo struct {
	name  string
	size  int64
	isDir bool
}

func (m MockFileInfo) Name() string       { return m.name }
func (m MockFileInfo) Size() int64        { return m.size }
func (m MockFileInfo) Mode() fs.FileMode  { return 0 }
func (m MockFileInfo) ModTime() time.Time { return time.Now() }
func (m MockFileInfo) IsDir() bool        { return m.isDir }
func (m MockFileInfo) Sys() interface{}   { return nil }

func TestHybridFSMCP_CallTool(t *testing.T) {
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	t.Run("read_file", func(t *testing.T) {
		provider := &MockProvider{
			ReadFileFunc: func(ctx context.Context, claims *Claims, path string) ([]byte, error) {
				if path != "test.txt" {
					t.Errorf("Expected path 'test.txt', got '%s'", path)
				}
				return []byte("content"), nil
			},
		}
		mcp := NewHybridFSMCP(provider)

		args := map[string]interface{}{"path": "test.txt"}
		res, err := mcp.CallTool(ctx, "read_file", args, claims)
		if err != nil {
			t.Fatalf("Unexpected error: %v", err)
		}

		resMap := res.(map[string]interface{})
		if resMap["content"] != "content" {
			t.Errorf("Expected content 'content', got '%v'", resMap["content"])
		}
	})

	t.Run("write_file", func(t *testing.T) {
		provider := &MockProvider{
			WriteFileFunc: func(ctx context.Context, claims *Claims, path string, data []byte) error {
				if path != "test.txt" {
					t.Errorf("Expected path 'test.txt', got '%s'", path)
				}
				if string(data) != "new content" {
					t.Errorf("Expected data 'new content', got '%s'", string(data))
				}
				return nil
			},
		}
		mcp := NewHybridFSMCP(provider)

		args := map[string]interface{}{"path": "test.txt", "content": "new content"}
		res, err := mcp.CallTool(ctx, "write_file", args, claims)
		if err != nil {
			t.Fatalf("Unexpected error: %v", err)
		}

		resMap := res.(map[string]interface{})
		if resMap["status"] != "success" {
			t.Errorf("Expected status 'success', got '%v'", resMap["status"])
		}
	})

	t.Run("list_directory", func(t *testing.T) {
		provider := &MockProvider{
			ListDirFunc: func(ctx context.Context, claims *Claims, path string) ([]fs.FileInfo, error) {
				return []fs.FileInfo{
					MockFileInfo{name: "file1.txt", size: 100, isDir: false},
					MockFileInfo{name: "dir1", size: 0, isDir: true},
				}, nil
			},
		}
		mcp := NewHybridFSMCP(provider)

		args := map[string]interface{}{"path": "."}
		res, err := mcp.CallTool(ctx, "list_directory", args, claims)
		if err != nil {
			t.Fatalf("Unexpected error: %v", err)
		}

		resMap := res.(map[string]interface{})
		files := resMap["files"].([]map[string]interface{})
		if len(files) != 2 {
			t.Fatalf("Expected 2 files, got %d", len(files))
		}
		if files[0]["name"] != "file1.txt" || files[1]["name"] != "dir1" {
			t.Errorf("Unexpected files list")
		}
	})

	t.Run("search_files", func(t *testing.T) {
		provider := &MockProvider{
			SearchFilesFunc: func(ctx context.Context, claims *Claims, query string) ([]string, error) {
				return []string{"match1.txt", "match2.txt"}, nil
			},
		}
		mcp := NewHybridFSMCP(provider)

		args := map[string]interface{}{"query": "match"}
		res, err := mcp.CallTool(ctx, "search_files", args, claims)
		if err != nil {
			t.Fatalf("Unexpected error: %v", err)
		}

		resMap := res.(map[string]interface{})
		files := resMap["files"].([]string)
		if len(files) != 2 {
			t.Fatalf("Expected 2 files, got %d", len(files))
		}
	})

	t.Run("unknown tool", func(t *testing.T) {
		mcp := NewHybridFSMCP(nil) // Uses default provider
		_, err := mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{}, claims)
		if err == nil {
			t.Errorf("Expected error for unknown tool")
		}
	})

	t.Run("read_file missing arg", func(t *testing.T) {
		mcp := NewHybridFSMCP(&MockProvider{})
		_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{}, claims)
		if err == nil {
			t.Errorf("Expected error for missing path")
		}
	})

	t.Run("write_file missing path", func(t *testing.T) {
		mcp := NewHybridFSMCP(&MockProvider{})
		_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{"content": "data"}, claims)
		if err == nil {
			t.Errorf("Expected error for missing path")
		}
	})

	t.Run("write_file missing content", func(t *testing.T) {
		mcp := NewHybridFSMCP(&MockProvider{})
		_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt"}, claims)
		if err == nil {
			t.Errorf("Expected error for missing content")
		}
	})

	t.Run("list_directory missing arg", func(t *testing.T) {
		mcp := NewHybridFSMCP(&MockProvider{})
		_, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{}, claims)
		if err == nil {
			t.Errorf("Expected error for missing path")
		}
	})

	t.Run("search_files missing arg", func(t *testing.T) {
		mcp := NewHybridFSMCP(&MockProvider{})
		_, err := mcp.CallTool(ctx, "search_files", map[string]interface{}{}, claims)
		if err == nil {
			t.Errorf("Expected error for missing query")
		}
	})
}
