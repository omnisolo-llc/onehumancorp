package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello world" {
		t.Fatalf("Expected 'hello world', got '%s'", string(data))
	}

	// List
	dir, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(dir) != 1 || dir[0] != "test.txt" {
		t.Fatalf("Expected ['test.txt'], got %v", dir)
	}

	// Search
	res, err := provider.SearchFiles(ctx, ".", "test")
	if err != nil {
		t.Fatalf("Failed to search files: %v", err)
	}
	if len(res) != 1 || res[0] != "test.txt" {
		t.Fatalf("Expected ['test.txt'], got %v", res)
	}

	// Security: Escape attempt
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Fatalf("Expected error when attempting path escape")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	ctxWithClaims := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant123",
	})

	ctxWithoutClaims := context.Background()

	// Write without claims should fail
	err = provider.WriteFile(ctxWithoutClaims, "test.txt", []byte("data"))
	if err == nil {
		t.Fatalf("Expected error without claims")
	}

	// Write with claims
	err = provider.WriteFile(ctxWithClaims, "test.txt", []byte("cloud data"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Read with claims
	data, err := provider.ReadFile(ctxWithClaims, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "cloud data" {
		t.Fatalf("Expected 'cloud data', got '%s'", string(data))
	}

	// List with claims
	dir, err := provider.ListDir(ctxWithClaims, ".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(dir) != 1 || dir[0] != "test.txt" {
		t.Fatalf("Expected ['test.txt'], got %v", dir)
	}

	// Security: escape attempt
	err = provider.WriteFile(ctxWithClaims, "../escaped.txt", []byte("escape"))
	if err == nil {
		t.Fatalf("Expected error when attempting path escape")
	}

	// Make sure tenant separation works (implied by directory structure)
	tenantPath := filepath.Join(tmpDir, "tenant123", "test.txt")
	if _, err := os.Stat(tenantPath); os.IsNotExist(err) {
		t.Fatalf("File not created in correct tenant directory: %v", err)
	}
}

func TestHybridFSMCP(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	mcp := NewHybridFSMCP(provider)

	ctx := context.Background()

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Fatalf("Expected 4 tools, got %d", len(tools))
	}

	// Write file via MCP
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp data",
	})
	if err != nil {
		t.Fatalf("Failed to call write_file: %v", err)
	}

	// Read file via MCP
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if err != nil {
		t.Fatalf("Failed to call read_file: %v", err)
	}
	if res.(string) != "mcp data" {
		t.Fatalf("Expected 'mcp data', got '%s'", res)
	}

	// List via MCP
	resList, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("Failed to call list_directory: %v", err)
	}
	if len(resList.([]string)) != 1 || resList.([]string)[0] != "mcp_test.txt" {
		t.Fatalf("Expected ['mcp_test.txt'], got %v", resList)
	}

	// Search via MCP
	resSearch, err := mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"dir":     ".",
		"pattern": "mcp",
	})
	if err != nil {
		t.Fatalf("Failed to call search_files: %v", err)
	}
	if len(resSearch.([]string)) != 1 || resSearch.([]string)[0] != "mcp_test.txt" {
		t.Fatalf("Expected ['mcp_test.txt'], got %v", resSearch)
	}
}

func TestProviderFactory(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	p1 := ProviderFactory()
	if _, ok := p1.(*CloudFSProvider); !ok {
		t.Fatalf("Expected CloudFSProvider, got %T", p1)
	}

	os.Setenv("OHC_MULTITENANT", "false")
	p2 := ProviderFactory()
	if _, ok := p2.(*LocalFSProvider); !ok {
		t.Fatalf("Expected LocalFSProvider, got %T", p2)
	}
}

func TestCloudFSSearch(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_search")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	ctxWithClaims := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant123",
	})

	err = provider.WriteFile(ctxWithClaims, "search_test.txt", []byte("data"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	res, err := provider.SearchFiles(ctxWithClaims, ".", "search")
	if err != nil {
		t.Fatalf("Failed to search files: %v", err)
	}
	if len(res) != 1 || res[0] != "search_test.txt" {
		t.Fatalf("Expected ['search_test.txt'], got %v", res)
	}
}

func TestHybridFSMCP_Errors(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_test_errors")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	mcp := NewHybridFSMCP(provider)

	ctx := context.Background()

	// read_file missing path
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Fatalf("Expected error missing path")
	}

	// write_file missing content
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt"})
	if err == nil {
		t.Fatalf("Expected error missing content")
	}

	// list_dir missing path
	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{})
	if err == nil {
		t.Fatalf("Expected error missing path")
	}

	// search missing pattern
	_, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{"dir": "."})
	if err == nil {
		t.Fatalf("Expected error missing pattern")
	}

	// search missing dir
	_, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{"pattern": "a"})
	if err == nil {
		t.Fatalf("Expected error missing dir")
	}

	// unknown tool
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Fatalf("Expected error for unknown tool")
	}
}

func TestLocalFSErrors(t *testing.T) {
	provider := NewLocalFSProvider("/non/existent/dir")
	ctx := context.Background()

	_, err := provider.ListDir(ctx, ".")
	if err == nil {
		t.Fatalf("Expected error listing non-existent dir")
	}

	_, err = provider.ReadFile(ctx, "nonexistent.txt")
	if err == nil {
		t.Fatalf("Expected error reading non-existent file")
	}
}

func TestHybridFSMCP_MoreErrors(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_test_more_errors")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// write_file missing path
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"content": "abc"})
	if err == nil {
		t.Fatalf("Expected error missing path")
	}

	// try to read file outside
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{"path": "../abc"})
	if err == nil {
		t.Fatalf("Expected error out of bounds")
	}

	// try to write file outside
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "../abc", "content": "abc"})
	if err == nil {
		t.Fatalf("Expected error out of bounds")
	}

	// list dir outside
	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{"path": "../abc"})
	if err == nil {
		t.Fatalf("Expected error out of bounds")
	}

	// search outside
	_, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{"dir": "../abc", "pattern": "a"})
	if err == nil {
		t.Fatalf("Expected error out of bounds")
	}
}

func TestCloudFSErrors(t *testing.T) {
	provider := NewCloudFSProvider("/tmp/cloud")
	ctxWithClaims := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant123",
	})

	_, err := provider.ListDir(ctxWithClaims, ".")
	if err == nil {
		t.Fatalf("Expected error listing non-existent dir")
	}

	_, err = provider.ReadFile(ctxWithClaims, "nonexistent.txt")
	if err == nil {
		t.Fatalf("Expected error reading non-existent file")
	}

	_, err = provider.SearchFiles(context.Background(), ".", "a")
	if err == nil {
		t.Fatalf("Expected error no claims")
	}
}

func TestSiblingDirectoryEscape(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_sibling")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	// Victim directory setup
	victimDir := filepath.Join(tmpDir, "tenant12")
	os.MkdirAll(victimDir, 0755)
	os.WriteFile(filepath.Join(victimDir, "secret.txt"), []byte("secret"), 0644)

	// Attacker tenant context
	ctxWithClaims := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant1",
	})

	// Attacker tries to read sibling directory relying on prefix match logic:
	// "tenant1" is a prefix of "tenant12".
	// The path requested: "../tenant12/secret.txt"
	// From attacker base /tmp/cloudfs_sibling/tenant1/../tenant12/secret.txt -> /tmp/cloudfs_sibling/tenant12/secret.txt
	_, err = provider.ReadFile(ctxWithClaims, "../tenant12/secret.txt")
	if err == nil {
		t.Fatalf("Expected error when attempting sibling directory traversal via prefix match")
	}
}
