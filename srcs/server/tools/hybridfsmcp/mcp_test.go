package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHybridFSMCP_Standalone(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "hybridfsmcp_test_standalone")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewProvider("OHC_STANDALONE", tempDir)
	mcp := NewHybridFSMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("Expected 4 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// Test write_file
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "data": "hello world"})
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}

	// Test read_file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"})
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["data"] != "hello world" {
		t.Errorf("expected 'hello world', got %v", resMap["data"])
	}

	// Test list_directory
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "."})
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	files := resMap["files"].([]string)
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", files)
	}

	// Test search_files
	res, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{"path": ".", "query": "test"})
	if err != nil {
		t.Fatalf("search_files failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	files = resMap["files"].([]string)
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", files)
	}

	// Test security: path escape (read)
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "../escape.txt"})
	if err == nil {
		t.Errorf("expected error for path escape read, got nil")
	}

	// Test security: path escape (write)
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "../escape.txt", "data": "bad"})
	if err == nil {
		t.Errorf("expected error for path escape write, got nil")
	}

	// Test security: path escape (list)
	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "../"})
	if err == nil {
		t.Errorf("expected error for path escape list, got nil")
	}

	// Test security: path escape (search)
	_, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{"path": "../", "query": "a"})
	if err == nil {
		t.Errorf("expected error for path escape search, got nil")
	}
}

func TestHybridFSMCP_Cloud(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "hybridfsmcp_test_cloud")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewProvider("OHC_MULTITENANT", tempDir)
	mcp := NewHybridFSMCP(provider)

	// Context with claims
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Create tenant dir
	os.MkdirAll(filepath.Join(tempDir, "tenant1"), 0755)
	os.MkdirAll(filepath.Join(tempDir, "tenant2"), 0755)
	os.WriteFile(filepath.Join(tempDir, "tenant2", "secret.txt"), []byte("secret"), 0644)

	// Test write_file
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test_cloud.txt", "data": "cloud data"})
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}

	// Test read_file
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "test_cloud.txt"})
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["data"] != "cloud data" {
		t.Errorf("expected 'cloud data', got %v", resMap["data"])
	}

	// Test search_files
	res, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{"path": ".", "query": "cloud"})
	if err != nil {
		t.Fatalf("search_files failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	files := resMap["files"].([]string)
	if len(files) != 1 || files[0] != "test_cloud.txt" {
		t.Errorf("expected ['test_cloud.txt'], got %v", files)
	}

	// Test missing claims
	ctxNoClaims := context.Background()
	_, err = mcp.CallTool(ctxNoClaims, "read_file", map[string]interface{}{"path": "test_cloud.txt"})
	if err == nil {
		t.Errorf("expected error for missing claims, got nil")
	}

	// Test path traversal security (attempt to access tenant2 from tenant1)
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "../tenant2/secret.txt"})
	if err == nil {
		t.Errorf("expected error for cross-tenant path escape read, got nil")
	}
}
