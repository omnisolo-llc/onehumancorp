package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs-test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Valid path
	validPath := "test.txt"
	err = provider.WriteFile(ctx, validPath, []byte("hello"))
	if err != nil {
		t.Errorf("expected no error for valid path, got: %v", err)
	}

	// Path traversal attempt
	traversalPath := "../outside.txt"
	err = provider.WriteFile(ctx, traversalPath, []byte("hello"))
	if err == nil || err.Error() != "path traversal detected" {
		t.Errorf("expected path traversal error, got: %v", err)
	}

	// Absolute path escaping attempt
	absPath := "/etc/passwd"
	err = provider.WriteFile(ctx, absPath, []byte("hello"))
	if err == nil {
		t.Errorf("expected error for absolute path traversal")
	}
}

func TestCloudFSProvider_TenantScoping(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs-test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	// Context without claims
	ctx := context.Background()
	_, err = provider.ReadFile(ctx, "test.txt")
	if err == nil || err.Error() != "unauthorized: missing claims" {
		t.Errorf("expected missing claims error, got: %v", err)
	}

	// Context with claims
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Write file
	err = provider.WriteFile(ctxWithClaims, "test.txt", []byte("tenant data"))
	if err != nil {
		t.Errorf("expected no error, got: %v", err)
	}

	// Verify file is actually in the tenant directory
	tenantDir := filepath.Join(tmpDir, "tenant1")
	actualPath := filepath.Join(tenantDir, "test.txt")
	data, err := os.ReadFile(actualPath)
	if err != nil {
		t.Errorf("failed to read actual file: %v", err)
	}
	if string(data) != "tenant data" {
		t.Errorf("expected 'tenant data', got: %s", string(data))
	}

	// Path traversal attempt
	err = provider.WriteFile(ctxWithClaims, "../outside.txt", []byte("hello"))
	if err == nil || err.Error() != "path traversal detected" {
		t.Errorf("expected path traversal error, got: %v", err)
	}
}

func TestHybridFSMCP_Tools(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "hybridmcp-test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	mcp, err := NewHybridFSMCP(true, tmpDir)
	if err != nil {
		t.Fatalf("failed to create MCP: %v", err)
	}

	ctx := context.Background()

	// Test ListTools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}

	// Test CallTool - write_file
	content := "test data"
	encodedContent := base64.StdEncoding.EncodeToString([]byte(content))

	writeArgs := map[string]interface{}{
		"path":    "file1.txt",
		"content": encodedContent,
	}
	res, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Errorf("write_file failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("expected success, got %v", resMap["status"])
	}

	// Test CallTool - read_file
	readArgs := map[string]interface{}{
		"path": "file1.txt",
	}
	res, err = mcp.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Errorf("read_file failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("expected success, got %v", resMap["status"])
	}
	if resMap["content"] != encodedContent {
		t.Errorf("expected content %s, got %s", encodedContent, resMap["content"])
	}

	// Test CallTool - list_directory
	listArgs := map[string]interface{}{
		"path": ".",
	}
	res, err = mcp.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Errorf("list_directory failed: %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Errorf("expected success, got %v", resMap["status"])
	}
	entries := resMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "file1.txt" {
		t.Errorf("unexpected directory contents: %v", entries)
	}
}
