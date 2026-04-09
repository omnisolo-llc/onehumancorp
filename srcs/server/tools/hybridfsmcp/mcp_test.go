package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create local provider: %v", err)
	}

	if !provider.IsLocal() {
		t.Errorf("Expected IsLocal to be true")
	}

	ctx := context.Background()

	// Test write
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("hello world"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test read
	content, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(content) != "hello world" {
		t.Errorf("Expected 'hello world', got '%s'", string(content))
	}

	// Test bounds checking
	err = provider.WriteFile(ctx, nil, "../outside.txt", []byte("bad"))
	if err == nil {
		t.Errorf("Expected bounds check to fail")
	}

	// Test list
	infos, err := provider.ListDir(ctx, nil, ".")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name != "test.txt" {
		t.Errorf("Expected 1 file named test.txt, got %+v", infos)
	}

	// Test search
	matches, err := provider.SearchFiles(ctx, nil, "test")
	if err != nil {
		t.Errorf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("Expected 1 match named test.txt, got %+v", matches)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)

	if provider.IsLocal() {
		t.Errorf("Expected IsLocal to be false")
	}

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant1"}
	claims2 := &auth.Claims{OrganizationID: "tenant2"}

	// Missing claims
	err = provider.WriteFile(ctx, nil, "test.txt", []byte("hello"))
	if err == nil {
		t.Errorf("Expected WriteFile to fail without claims")
	}

	// Test write
	err = provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test tenant isolation
	_, err = provider.ReadFile(ctx, claims2, "test.txt")
	if err == nil {
		t.Errorf("Expected ReadFile to fail for different tenant")
	}

	// Test read
	content, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(content) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(content))
	}

	// Test bounds checking
	err = provider.WriteFile(ctx, claims, "../outside.txt", []byte("bad"))
	if err == nil {
		t.Errorf("Expected bounds check to fail")
	}

	// Test list
	infos, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name != "test.txt" {
		t.Errorf("Expected 1 file named test.txt, got %+v", infos)
	}

	// Test search
	matches, err := provider.SearchFiles(ctx, claims, "test")
	if err != nil {
		t.Errorf("SearchFiles failed: %v", err)
	}
	// Depending on filepath.Rel, it might return just the filename
	if len(matches) != 1 || filepath.Base(matches[0]) != "test.txt" {
		t.Errorf("Expected 1 match named test.txt, got %+v", matches)
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	localProvider, _ := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(localProvider)

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("Expected 4 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// write_file
	b64content := base64.StdEncoding.EncodeToString([]byte("mcp content"))
	writeRes, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": b64content,
	})
	if err != nil {
		t.Errorf("CallTool write_file failed: %v", err)
	}
	if writeRes.(map[string]interface{})["status"] != "success" {
		t.Errorf("Expected success, got %v", writeRes)
	}

	// read_file
	readRes, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if err != nil {
		t.Errorf("CallTool read_file failed: %v", err)
	}
	contentStr := readRes.(map[string]interface{})["content"].(string)
	if contentStr != b64content {
		t.Errorf("Expected %s, got %s", b64content, contentStr)
	}

	// list_directory
	listRes, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Errorf("CallTool list_directory failed: %v", err)
	}
	results := listRes.(map[string]interface{})["results"].([]map[string]interface{})
	if len(results) != 1 || results[0]["name"] != "mcp_test.txt" {
		t.Errorf("Expected 1 result named mcp_test.txt, got %+v", results)
	}

	// search_files
	searchRes, err := mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"query": "mcp_test",
	})
	if err != nil {
		t.Errorf("CallTool search_files failed: %v", err)
	}
	matches := searchRes.(map[string]interface{})["results"].([]string)
	if len(matches) != 1 || matches[0] != "mcp_test.txt" {
		t.Errorf("Expected 1 match named mcp_test.txt, got %+v", matches)
	}

	// test missing args
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Errorf("Expected error for missing args")
	}

	// test unknown tool
	_, err = mcp.CallTool(ctx, "unknown", map[string]interface{}{})
	if err == nil {
		t.Errorf("Expected error for unknown tool")
	}
}

// TestCloudMissingClaims
func TestHybridFSMCPCloudMissingClaims(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_test_cloud")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	cloudProvider := NewCloudFSProvider(tempDir)
	mcp := NewHybridFSMCP(cloudProvider)

	ctx := context.Background()

	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err == nil {
		t.Errorf("Expected error for missing claims in cloud mode")
	}
}
