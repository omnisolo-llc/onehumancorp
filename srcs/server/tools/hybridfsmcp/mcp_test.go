package hybridfsmcp

import (
	"context"
	"testing"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create LocalFSProvider: %v", err)
	}

	if !provider.IsLocal() {
		t.Error("Expected IsLocal() to return true")
	}

	ctx := context.Background()

	// Write file
	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// List directory
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Escape path test
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Error("Expected error for path escape, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	provider := NewCloudFSProvider()

	if provider.IsLocal() {
		t.Error("Expected IsLocal() to return false")
	}

	ctx := context.WithValue(context.Background(), TenantIDKey, "tenant123")

	// Write file
	err := provider.WriteFile(ctx, "data/test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read file
	data, err := provider.ReadFile(ctx, "data/test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// List directory
	entries, err := provider.ListDir(ctx, "data")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Read file not exist
	_, err = provider.ReadFile(ctx, "notexist.txt")
	if err == nil {
		t.Error("Expected error for non-existent file")
	}

	// Tenant Isolation
	ctx2 := context.WithValue(context.Background(), TenantIDKey, "tenant456")
	_, err = provider.ReadFile(ctx2, "data/test.txt")
	if err == nil {
		t.Error("Expected error when reading other tenant's file")
	}

	// Escape path test
	err = provider.WriteFile(ctx, "../escape.txt", []byte("hack"))
	if err == nil {
		t.Error("Expected error for path escape, got nil")
	}

	// Missing Tenant
	_, err = provider.ReadFile(context.Background(), "data/test.txt")
	if err == nil {
		t.Error("Expected error for missing tenant")
	}
}

// mockContext returns a context with Claims set
func mockContext(tenantID string) context.Context {
	// Create context with claims and the tenant key
	// In the real app auth middleware sets the claims. Since auth.ClaimsFromContext
	// relies on a private context key inside auth package, we can just test the
	// integration by checking the CloudFS logic since we already mocked it or we can set it.
	ctx := context.WithValue(context.Background(), TenantIDKey, tenantID)
	// Workaround: We can test without claims if provider is local
	return ctx
}

func TestHybridFSMCP_Local(t *testing.T) {
	tmpDir := t.TempDir()
	provider, _ := NewLocalFSProvider(tmpDir)
	mcp := NewHybridFSMCP(provider)

	ctx := context.Background() // Local doesn't need claims

	// write_file
	args := map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	}
	_, err := mcp.CallTool(ctx, "write_file", args)
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// read_file
	args = map[string]interface{}{
		"path": "hello.txt",
	}
	res, err := mcp.CallTool(ctx, "read_file", args)
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if res.(string) != "world" {
		t.Errorf("Expected 'world', got '%v'", res)
	}

	// list_directory
	args = map[string]interface{}{
		"path": ".",
	}
	res, err = mcp.CallTool(ctx, "list_directory", args)
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	entries := res.([]string)
	if len(entries) != 1 || entries[0] != "hello.txt" {
		t.Errorf("Expected ['hello.txt'], got %v", entries)
	}

	// search_files
	args = map[string]interface{}{
		"path": ".",
		"term": "hello",
	}
	res, err = mcp.CallTool(ctx, "search_files", args)
	if err != nil {
		t.Fatalf("CallTool search_files failed: %v", err)
	}
	matches := res.([]string)
	if len(matches) != 1 || matches[0] != "hello.txt" {
		t.Errorf("Expected ['hello.txt'], got %v", matches)
	}

	// list tools
	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("Expected 4 tools, got %d", len(tools))
	}

	// Error paths
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{})
	if err == nil {
		t.Error("Expected error for missing args")
	}

	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Error("Expected error for missing args")
	}

	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{})
	if err == nil {
		t.Error("Expected error for missing args")
	}

	_, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{"path":"."})
	if err == nil {
		t.Error("Expected error for missing args")
	}

	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Error("Expected error for unknown tool")
	}
}

func TestHybridFSMCP_Cloud_Unauthorized(t *testing.T) {
	provider := NewCloudFSProvider()
	mcp := NewHybridFSMCP(provider)

	ctx := context.Background() // Missing claims

	_, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "."})
	if err == nil || err.Error() != "unauthorized: missing claims" {
		t.Errorf("Expected unauthorized error, got %v", err)
	}
}
