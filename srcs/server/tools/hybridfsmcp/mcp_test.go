package hybridfsmcp

import (
	"context"
	"encoding/json"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type mockProvider struct {
	data map[string]string
}

func (m *mockProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	return []byte(m.data[path]), nil
}

func (m *mockProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	m.data[path] = string(content)
	return nil
}

func (m *mockProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	return []string{"file1.txt"}, nil
}

func TestHybridFSMCP(t *testing.T) {
	mock := &mockProvider{data: make(map[string]string)}
	mcp := NewHybridFSMCP(mock)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})

	t.Run("Write File Tool", func(t *testing.T) {
		input := writeFileInput{Path: "test.txt", Content: "hello mcp"}
		data, _ := json.Marshal(input)
		_, err := mcp.WriteFileTool(ctx, data)
		if err != nil {
			t.Fatalf("WriteFileTool failed: %v", err)
		}
		if mock.data["test.txt"] != "hello mcp" {
			t.Errorf("Expected 'hello mcp', got '%s'", mock.data["test.txt"])
		}
	})

	t.Run("Read File Tool", func(t *testing.T) {
		input := readFileInput{Path: "test.txt"}
		data, _ := json.Marshal(input)
		res, err := mcp.ReadFileTool(ctx, data)
		if err != nil {
			t.Fatalf("ReadFileTool failed: %v", err)
		}
		resMap := res.(map[string]string)
		if resMap["content"] != "hello mcp" {
			t.Errorf("Expected 'hello mcp', got '%s'", resMap["content"])
		}
	})

	t.Run("List Dir Tool", func(t *testing.T) {
		input := listDirInput{Path: "."}
		data, _ := json.Marshal(input)
		res, err := mcp.ListDirTool(ctx, data)
		if err != nil {
			t.Fatalf("ListDirTool failed: %v", err)
		}
		resMap := res.(map[string]interface{})
		entries := resMap["entries"].([]string)
		if len(entries) != 1 || entries[0] != "file1.txt" {
			t.Errorf("Unexpected entries: %v", entries)
		}
	})
}
