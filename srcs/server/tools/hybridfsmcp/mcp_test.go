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

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatal(err)
	}

	if !provider.IsLocal() {
		t.Errorf("expected LocalFSProvider to be local")
	}

	ctx := context.Background()

	// Test writing
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatal(err)
	}

	// Test reading
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got %s", string(data))
	}

	// Test path traversal prevention
	_, err = provider.ReadFile(ctx, "../test.txt")
	if err == nil {
		t.Error("expected error on path traversal")
	}

	// Check exact resolution logic behavior
	// (e.g. if I try to read an absolute path that happens to be outside)
	_, err = provider.ReadFile(ctx, "/etc/passwd")
	if err == nil {
		t.Error("expected error on absolute path escaping base")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatal(err)
	}

	if provider.IsLocal() {
		t.Errorf("expected CloudFSProvider not to be local")
	}

	ctx := context.Background()

	// Simulating MCP wrapper which prepends tenant ID
	tenantPath := filepath.Join("tenant1", "test.txt")

	err = provider.WriteFile(ctx, tenantPath, []byte("cloud"))
	if err != nil {
		t.Fatal(err)
	}

	data, err := provider.ReadFile(ctx, tenantPath)
	if err != nil {
		t.Fatal(err)
	}
	if string(data) != "cloud" {
		t.Errorf("expected 'cloud', got %s", string(data))
	}
}

func TestHybridFSMCP_Standalone(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_standalone_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp, err := NewHybridFSMCP(tmpDir)
	if err != nil {
		t.Fatal(err)
	}

	ctx := context.Background()

	// Write without claims (allowed in standalone)
	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "file.txt",
		"content": "standalone data",
	})
	if err != nil {
		t.Fatal(err)
	}

	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" || resMap["mode"] != "standalone" {
		t.Errorf("unexpected write response: %v", resMap)
	}

	// Read
	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "file.txt",
	})
	if err != nil {
		t.Fatal(err)
	}

	resMap = res.(map[string]interface{})
	if resMap["content"] != "standalone data" {
		t.Errorf("unexpected read content: %v", resMap)
	}

	// List
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": "",
	})
	if err != nil {
		t.Fatal(err)
	}
	resMap = res.(map[string]interface{})
	results := resMap["results"].([]map[string]interface{})
	if len(results) != 1 || results[0]["name"] != "file.txt" {
		t.Errorf("unexpected list results: %v", results)
	}
}

func TestHybridFSMCP_Cloud(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_cloud_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_STANDALONE", "false")
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp, err := NewHybridFSMCP(tmpDir)
	if err != nil {
		t.Fatal(err)
	}

	ctx := context.Background()

	// Attempt without claims (should fail)
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "file.txt",
		"content": "cloud data",
	})
	if err == nil {
		t.Fatal("expected unauthorized error")
	}

	// Attempt with claims
	claims := &auth.Claims{OrganizationID: "org123"}
	ctx = auth.ContextWithClaims(ctx, claims)

	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "file.txt",
		"content": "cloud data",
	})
	if err != nil {
		t.Fatal(err)
	}

	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" || resMap["mode"] != "cloud" {
		t.Errorf("unexpected write response: %v", resMap)
	}

	// Read
	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "file.txt",
	})
	if err != nil {
		t.Fatal(err)
	}

	resMap = res.(map[string]interface{})
	if resMap["content"] != "cloud data" {
		t.Errorf("unexpected read content: %v", resMap)
	}

	// Check that file was actually written to org123/file.txt on disk
	actualData, err := os.ReadFile(filepath.Join(tmpDir, "org123", "file.txt"))
	if err != nil {
		t.Fatal(err)
	}
	if string(actualData) != "cloud data" {
		t.Errorf("unexpected disk content: %s", string(actualData))
	}
}
