package hybridfsmcp

import (
	"context"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_SecurePath(t *testing.T) {
	tempDir := t.TempDir()
	p := NewLocalFSProvider(tempDir)

	valid, err := p.securePath("test.txt")
	if err != nil {
		t.Errorf("Expected success, got %v", err)
	}
	expectedValid := filepath.Join(tempDir, "test.txt")
	if valid != expectedValid {
		t.Errorf("Expected %s, got %s", expectedValid, valid)
	}

	_, err = p.securePath("../outside.txt")
	if err == nil {
		t.Errorf("Expected error for path traversal, got nil")
	}
}

func TestCloudFSProvider_SecurePath(t *testing.T) {
	tempDir := t.TempDir()
	p := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{OrganizationID: "tenant1"}

	valid, err := p.securePath(claims, "test.txt")
	if err != nil {
		t.Errorf("Expected success, got %v", err)
	}
	expectedValid := filepath.Join(tempDir, "tenant1", "test.txt")
	if valid != expectedValid {
		t.Errorf("Expected %s, got %s", expectedValid, valid)
	}

	_, err = p.securePath(claims, "../tenant2/outside.txt")
	if err == nil {
		t.Errorf("Expected error for path traversal out of tenant scope, got nil")
	}

	_, err = p.securePath(nil, "test.txt")
	if err == nil {
		t.Errorf("Expected error for missing claims, got nil")
	}
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	tempDir := t.TempDir()
	p := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(p)

	ctx := context.Background()

	// Test write
	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"content": "hello world",
	})
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test read
	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "hello world" {
		t.Errorf("Expected hello world, got %v", resMap["content"])
	}

	// Test list
	resList, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	resListMap := resList.(map[string]interface{})
	entries := resListMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Unexpected entries: %v", entries)
	}
}

func TestCloudFSMCP_Unauthorized(t *testing.T) {
	tempDir := t.TempDir()
	p := NewCloudFSProvider(tempDir)
	mcp := NewHybridFSMCP(p)

	ctx := context.Background()

	_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err == nil || err.Error() != "unauthorized: missing claims" {
		t.Errorf("Expected unauthorized error, got %v", err)
	}

	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Attempt read on non-existent
	_, err = mcp.CallTool(ctxWithClaims, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err == nil {
		t.Errorf("Expected not found error, got nil")
	}
}
