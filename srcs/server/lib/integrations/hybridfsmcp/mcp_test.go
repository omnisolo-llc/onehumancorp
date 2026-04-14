package hybridfsmcp

import (
	"context"
	"strings"

	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got '%s'", string(data))
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("unexpected directory contents: %v", entries)
	}

	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("expected path traversal error, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{OrganizationID: "org-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got '%s'", string(data))
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("unexpected directory contents: %v", entries)
	}

	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("expected path traversal error, got nil")
	}

	// Test missing claims
	ctxNoAuth := context.Background()
	_, err = provider.ReadFile(ctxNoAuth, "test.txt")
	if err == nil {
		t.Errorf("expected authorization error, got nil")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider, nil)
	ctx := context.Background()

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("expected 4 tools, got %d", len(tools))
	}

	// Write
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "content": "hello"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Read
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "hello" {
		t.Errorf("expected 'hello', got '%v'", resMap["content"])
	}

	// List
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "."})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap = res.(map[string]interface{})
	entries := resMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("unexpected directory contents: %v", entries)
	}
}

func TestHybridFSMCP_RAGQuery(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	escalator := NewDefaultEscalator()
	mcp := NewHybridFSMCP(provider, escalator)
	ctx := context.Background()

	// Short query - should use local
	res, err := mcp.CallTool(ctx, "rag_query", map[string]interface{}{"query": "short query"})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["source"] != "local" {
		t.Errorf("expected local source, got %v", resMap["source"])
	}

	// Long query - should escalate to cloud
	longQuery := strings.Repeat("a", 501)
	res, err = mcp.CallTool(ctx, "rag_query", map[string]interface{}{"query": longQuery})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["source"] != "cloud" {
		t.Errorf("expected cloud source, got %v", resMap["source"])
	}

	// Long query but cloud unreachable - should fallback to local
	escalator.CloudModeReachable = false
	res, err = mcp.CallTool(ctx, "rag_query", map[string]interface{}{"query": longQuery})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["source"] != "local" {
		t.Errorf("expected fallback to local source, got %v", resMap["source"])
	}
}
