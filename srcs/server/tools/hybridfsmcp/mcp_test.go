package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHybridFSMCP_LocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	localFS, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatal(err)
	}

	mcp := NewHybridFSMCP(localFS)
	ctx := context.Background()

	// Test WriteFile
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"key": "test.txt", "content": "hello world"})
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{"key": "test.txt"})
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "hello world" {
		t.Errorf("Expected content 'hello world', got '%v'", resMap["content"])
	}

	// Test boundary check
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{"key": "../../../etc/passwd"})
	if err == nil {
		t.Errorf("Expected boundary error, got nil")
	}
}

func TestHybridFSMCP_CloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	cloudFS, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatal(err)
	}

	mcp := NewHybridFSMCP(cloudFS)
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "test-org"})

	// Test WriteFile
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"key": "test.txt", "content": "cloud data"})
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Ensure it is in the tenant directory
	orgDir := filepath.Join(tmpDir, "test-org")
	content, err := os.ReadFile(filepath.Join(orgDir, "test.txt"))
	if err != nil {
		t.Fatalf("Expected file to exist in tenant directory, got err: %v", err)
	}
	if string(content) != "cloud data" {
		t.Errorf("Expected 'cloud data', got '%s'", string(content))
	}

	// Test ListDir
	res, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{"prefix": ""})
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	results := resMap["results"].([]string)
	if len(results) != 1 || results[0] != "test.txt" {
		t.Errorf("Expected 'test.txt' in results, got %v", results)
	}

	// Test unauthorized access
	ctxNoAuth := context.Background()
	_, err = mcp.CallTool(ctxNoAuth, "read_file", map[string]interface{}{"key": "test.txt"})
	if err == nil {
		t.Errorf("Expected unauthorized error, got nil")
	}
}
