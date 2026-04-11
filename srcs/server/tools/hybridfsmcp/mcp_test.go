package hybridfsmcp

import (
	"context"
	"errors"
	"reflect"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

// MockProvider is a mock for FileSystemProvider.
type MockProvider struct {
	readFunc  func(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	writeFunc func(ctx context.Context, claims *auth.Claims, path string, data []byte) error
	listFunc  func(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

func (m *MockProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	if m.readFunc != nil {
		return m.readFunc(ctx, claims, path)
	}
	return nil, nil
}

func (m *MockProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
	if m.writeFunc != nil {
		return m.writeFunc(ctx, claims, path, data)
	}
	return nil
}

func (m *MockProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	if m.listFunc != nil {
		return m.listFunc(ctx, claims, path)
	}
	return nil, nil
}

func TestHybridFSMCP_ListTools(t *testing.T) {
	mcp := NewHybridFSMCP(&MockProvider{}, true)
	tools := mcp.ListTools()

	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}

	toolNames := map[string]bool{}
	for _, tool := range tools {
		toolNames[tool.Name] = true
	}

	expected := []string{"read_file", "write_file", "list_directory"}
	for _, name := range expected {
		if !toolNames[name] {
			t.Errorf("expected tool %s to be listed", name)
		}
	}
}

func TestHybridFSMCP_CallTool_ReadFile(t *testing.T) {
	mockProvider := &MockProvider{
		readFunc: func(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
			if path == "test.txt" {
				return []byte("test content"), nil
			}
			return nil, errors.New("file not found")
		},
	}

	mcp := NewHybridFSMCP(mockProvider, true)
	ctx := context.Background()

	// Test successful read
	result, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap, ok := result.(map[string]interface{})
	if !ok || resMap["status"] != "success" || resMap["content"] != "test content" {
		t.Errorf("unexpected result: %v", result)
	}

	// Test missing path
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Error("expected error for missing path, got nil")
	}
}

func TestHybridFSMCP_CallTool_WriteFile(t *testing.T) {
	called := false
	mockProvider := &MockProvider{
		writeFunc: func(ctx context.Context, claims *auth.Claims, path string, data []byte) error {
			if path == "test.txt" && string(data) == "new data" {
				called = true
				return nil
			}
			return errors.New("unexpected write")
		},
	}

	mcp := NewHybridFSMCP(mockProvider, true)
	ctx := context.Background()

	// Test successful write
	result, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "new data",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if !called {
		t.Error("expected write provider function to be called")
	}
	resMap, ok := result.(map[string]interface{})
	if !ok || resMap["status"] != "success" {
		t.Errorf("unexpected result: %v", result)
	}

	// Test missing arguments
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt"})
	if err == nil {
		t.Error("expected error for missing content, got nil")
	}
}

func TestHybridFSMCP_CallTool_ListDirectory(t *testing.T) {
	mockProvider := &MockProvider{
		listFunc: func(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
			if path == "testdir" {
				return []string{"file1.txt", "file2.txt"}, nil
			}
			return nil, errors.New("dir not found")
		},
	}

	mcp := NewHybridFSMCP(mockProvider, true)
	ctx := context.Background()

	// Test successful list
	result, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "testdir"})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap, ok := result.(map[string]interface{})
	if !ok || resMap["status"] != "success" {
		t.Fatalf("unexpected result: %v", result)
	}
	entries, ok := resMap["entries"].([]string)
	if !ok || !reflect.DeepEqual(entries, []string{"file1.txt", "file2.txt"}) {
		t.Errorf("unexpected entries: %v", entries)
	}
}

func TestHybridFSMCP_CallTool_CloudModeAuth(t *testing.T) {
	mockProvider := &MockProvider{
		readFunc: func(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
			return []byte("ok"), nil
		},
	}

	mcp := NewHybridFSMCP(mockProvider, false) // Cloud mode

	// Should fail without claims
	ctx := context.Background()
	_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err == nil {
		t.Error("expected error for missing claims in cloud mode, got nil")
	}

	// Should succeed with claims
	claims := &auth.Claims{OrganizationID: "tenant-x"}
	ctxWithAuth := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)
	result, err := mcp.CallTool(ctxWithAuth, "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil {
		t.Fatalf("expected no error with claims, got %v", err)
	}
	resMap, ok := result.(map[string]interface{})
	if !ok || resMap["mode"] != "cloud" {
		t.Errorf("expected cloud mode result, got %v", result)
	}
}
