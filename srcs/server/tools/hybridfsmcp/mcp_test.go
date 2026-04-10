package hybridfsmcp

import (
	"context"
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

	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test WriteFile
	testPath := "test.txt"
	testContent := []byte("hello local")
	err = provider.WriteFile(ctx, testPath, testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	readContent, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readContent) != string(testContent) {
		t.Errorf("ReadFile mismatch. Got %s, want %s", string(readContent), string(testContent))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != testPath {
		t.Errorf("ListDir mismatch. Got %v, want [%s]", entries, testPath)
	}

	// Test path traversal
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Error("expected path traversal error, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)

	// Create context with claims
	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	testPath := "test.txt"
	testContent := []byte("hello cloud")
	err = provider.WriteFile(ctx, testPath, testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify tenant isolation (check underlying file system)
	expectedPath := filepath.Join(tempDir, "tenant1", testPath)
	if _, err := os.Stat(expectedPath); os.IsNotExist(err) {
		t.Errorf("tenant isolation failed, file not found at expected path: %s", expectedPath)
	}

	// Test ReadFile
	readContent, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readContent) != string(testContent) {
		t.Errorf("ReadFile mismatch. Got %s, want %s", string(readContent), string(testContent))
	}

	// Test unauthenticated access
	unauthCtx := context.Background()
	_, err = provider.ReadFile(unauthCtx, testPath)
	if err == nil {
		t.Error("expected unauthorized error for context without claims")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	// Test Write
	writeResult, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp content",
	})
	if err != nil {
		t.Fatalf("write_file tool failed: %v", err)
	}
	if status := writeResult.(map[string]interface{})["status"]; status != "success" {
		t.Errorf("expected success status, got %v", status)
	}

	// Test Read
	readResult, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if err != nil {
		t.Fatalf("read_file tool failed: %v", err)
	}
	if content := readResult.(map[string]interface{})["content"]; content != "mcp content" {
		t.Errorf("expected 'mcp content', got %v", content)
	}
}
