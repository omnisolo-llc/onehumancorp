package hybridfsmcp

import (
	"context"
	"reflect"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type mockFileSystemProvider struct {
	readFileFunc  func(ctx context.Context, claims *auth.Claims, path string) ([]byte, error)
	writeFileFunc func(ctx context.Context, claims *auth.Claims, path string, content []byte) error
	listDirFunc   func(ctx context.Context, claims *auth.Claims, path string) ([]string, error)
}

func (m *mockFileSystemProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	if m.readFileFunc != nil {
		return m.readFileFunc(ctx, claims, path)
	}
	return nil, nil
}

func (m *mockFileSystemProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	if m.writeFileFunc != nil {
		return m.writeFileFunc(ctx, claims, path, content)
	}
	return nil
}

func (m *mockFileSystemProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	if m.listDirFunc != nil {
		return m.listDirFunc(ctx, claims, path)
	}
	return nil, nil
}

func TestHybridFSMCP_ListTools(t *testing.T) {
	mcp := NewHybridFSMCP(&mockFileSystemProvider{})
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	names := map[string]bool{}
	for _, tool := range tools {
		names[tool.Name] = true
	}

	for _, name := range []string{"read_file", "write_file", "list_directory"} {
		if !names[name] {
			t.Errorf("missing tool: %s", name)
		}
	}
}

func TestHybridFSMCP_CallTool_NoClaims(t *testing.T) {
	mcp := NewHybridFSMCP(&mockFileSystemProvider{})
	_, err := mcp.CallTool(context.Background(), "read_file", map[string]interface{}{"path": "test.txt"})
	if err == nil {
		t.Fatal("Expected error for missing claims")
	}
}

func TestHybridFSMCP_CallTool_ReadFile(t *testing.T) {
	provider := &mockFileSystemProvider{
		readFileFunc: func(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
			if path != "test.txt" {
				t.Errorf("expected test.txt, got %s", path)
			}
			return []byte("hello world"), nil
		},
	}
	mcp := NewHybridFSMCP(provider)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil {
		t.Fatal(err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatal("Expected map response")
	}

	if resMap["content"] != "hello world" {
		t.Errorf("Expected hello world, got %v", resMap["content"])
	}
}

func TestHybridFSMCP_CallTool_WriteFile(t *testing.T) {
	provider := &mockFileSystemProvider{
		writeFileFunc: func(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
			if path != "test.txt" {
				t.Errorf("expected test.txt, got %s", path)
			}
			if string(content) != "new content" {
				t.Errorf("expected new content, got %s", string(content))
			}
			return nil
		},
	}
	mcp := NewHybridFSMCP(provider)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "content": "new content"})
	if err != nil {
		t.Fatal(err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatal("Expected map response")
	}

	if resMap["status"] != "success" {
		t.Errorf("Expected success, got %v", resMap["status"])
	}
}

func TestHybridFSMCP_CallTool_ListDir(t *testing.T) {
	provider := &mockFileSystemProvider{
		listDirFunc: func(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
			if path != "testdir" {
				t.Errorf("expected testdir, got %s", path)
			}
			return []string{"file1.txt", "file2.txt"}, nil
		},
	}
	mcp := NewHybridFSMCP(provider)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	res, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "testdir"})
	if err != nil {
		t.Fatal(err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatal("Expected map response")
	}

	files, ok := resMap["files"].([]string)
	if !ok {
		t.Fatal("Expected slice of strings")
	}

	if !reflect.DeepEqual(files, []string{"file1.txt", "file2.txt"}) {
		t.Errorf("Expected file1.txt, file2.txt, got %v", files)
	}
}

func TestHybridFSMCP_CallTool_UnknownTool(t *testing.T) {
	mcp := NewHybridFSMCP(&mockFileSystemProvider{})
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	_, err := mcp.CallTool(ctx, "unknown", nil)
	if err == nil {
		t.Fatal("Expected error for unknown tool")
	}
}

func TestHybridFSMCP_CallTool_MissingArgs(t *testing.T) {
	mcp := NewHybridFSMCP(&mockFileSystemProvider{})
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Fatal("Expected error for missing path")
	}

	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test"})
	if err == nil {
		t.Fatal("Expected error for missing content")
	}

	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{})
	if err == nil {
		t.Fatal("Expected error for missing path")
	}
}
