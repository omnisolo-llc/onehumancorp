package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

type mockFSProvider struct {
	isLocal bool
	written map[string]string
}

func (m *mockFSProvider) ReadFile(ctx context.Context, claims *auth.Claims, path string) ([]byte, error) {
	content, ok := m.written[path]
	if !ok {
		return nil, context.DeadlineExceeded // simulate an error if not found
	}
	return []byte(content), nil
}

func (m *mockFSProvider) WriteFile(ctx context.Context, claims *auth.Claims, path string, content []byte) error {
	m.written[path] = string(content)
	return nil
}

func (m *mockFSProvider) ListDir(ctx context.Context, claims *auth.Claims, path string) ([]string, error) {
	return []string{"file1.txt", "file2.txt"}, nil
}

func (m *mockFSProvider) IsLocal() bool {
	return m.isLocal
}

func TestHybridFSMCP_LocalMode(t *testing.T) {
	provider := &mockFSProvider{isLocal: true, written: make(map[string]string)}
	mcpServer := NewHybridFSMCP(provider)

	ctx := context.Background()

	// ListTools
	tools := mcpServer.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}

	// Write file (no claims needed)
	res, err := mcpServer.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" || resMap["mode"] != "standalone" {
		t.Errorf("unexpected response: %v", resMap)
	}

	// Read file
	res, err = mcpServer.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["content"] != "hello" || resMap["mode"] != "standalone" {
		t.Errorf("unexpected response: %v", resMap)
	}

	// List directory
	res, err = mcpServer.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": "dir1",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	entries := resMap["entries"].([]string)
	if len(entries) != 2 || resMap["mode"] != "standalone" {
		t.Errorf("unexpected response: %v", resMap)
	}
}

func TestHybridFSMCP_CloudMode(t *testing.T) {
	provider := &mockFSProvider{isLocal: false, written: make(map[string]string)}
	mcpServer := NewHybridFSMCP(provider)

	ctx := context.Background()

	// Should fail without claims
	_, err := mcpServer.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello",
	})
	if err == nil || err.Error() != "unauthorized: missing claims" {
		t.Errorf("expected unauthorized error, got %v", err)
	}

	// With claims
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	res, err := mcpServer.CallTool(ctxWithClaims, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello cloud",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" || resMap["mode"] != "cloud" {
		t.Errorf("unexpected response: %v", resMap)
	}

	res, err = mcpServer.CallTool(ctxWithClaims, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["content"] != "hello cloud" || resMap["mode"] != "cloud" {
		t.Errorf("unexpected response: %v", resMap)
	}
}
