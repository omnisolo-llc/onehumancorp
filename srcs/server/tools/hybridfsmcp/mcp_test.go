package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHybridFSMCP_Factory(t *testing.T) {
	tempDir := t.TempDir()

	// Test Standalone (Local)
	t.Setenv("OHC_STANDALONE", "true")
	mcpLocal := NewHybridFSMCP(tempDir)
	if _, ok := mcpLocal.provider.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider in standalone mode")
	}

	// Test Cloud (Tenant Isolated)
	t.Setenv("OHC_STANDALONE", "false")
	mcpCloud := NewHybridFSMCP(tempDir)
	if _, ok := mcpCloud.provider.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider in cloud mode")
	}
}

func TestHybridFSMCP_Tools(t *testing.T) {
	tempDir := t.TempDir()
	t.Setenv("OHC_STANDALONE", "true")
	mcp := NewHybridFSMCP(tempDir)

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// 1. Write file
	writeArgs := map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp testing",
	}
	res, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); !ok || resMap["status"] != "success" {
		t.Errorf("expected success status, got %v", res)
	}

	// 2. Read file
	readArgs := map[string]interface{}{
		"path": "mcp_test.txt",
	}
	res, err = mcp.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); !ok || resMap["content"] != "mcp testing" {
		t.Errorf("expected content 'mcp testing', got %v", res)
	}

	// 3. List directory
	listArgs := map[string]interface{}{
		"path": ".",
	}
	res, err = mcp.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	resMap, ok := res.(map[string]interface{})
	if !ok || resMap["status"] != "success" {
		t.Fatalf("expected success status, got %v", res)
	}
	entries, ok := resMap["entries"].([]map[string]interface{})
	if !ok || len(entries) != 1 || entries[0]["name"] != "mcp_test.txt" {
		t.Errorf("expected entries list with mcp_test.txt, got %v", res)
	}
}

func TestHybridFSMCP_CloudMode_RequiresAuth(t *testing.T) {
	tempDir := t.TempDir()
	t.Setenv("OHC_STANDALONE", "false")
	mcp := NewHybridFSMCP(tempDir)

	ctx := context.Background()

	// Write without claims should fail in CloudFSProvider
	writeArgs := map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp testing",
	}
	_, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err == nil {
		t.Fatalf("expected error without claims in cloud mode")
	}

	// Write with claims should succeed
	claims := &auth.Claims{OrganizationID: "test_org"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	_, err = mcp.CallTool(ctxWithClaims, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("expected success with claims, got error: %v", err)
	}

	// Verify isolated path was used
	if _, statErr := os.Stat(filepath.Join(tempDir, "test_org", "mcp_test.txt")); os.IsNotExist(statErr) {
		t.Errorf("file not written to isolated tenant path")
	}
}
