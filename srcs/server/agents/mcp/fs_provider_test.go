package mcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "local_fs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Valid write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("expected successful write, got %v", err)
	}

	// Valid read
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil || string(content) != "hello" {
		t.Errorf("expected to read 'hello', got %s (err: %v)", string(content), err)
	}

	// Traversal attempt 1: using ..
	_, err = provider.ReadFile(ctx, "../../../../../etc/passwd")
	if err == nil || err.Error() != "path escapes base directory" {
		t.Errorf("expected path escape error, got %v", err)
	}

	// Traversal attempt 2: absolute path bypass
	_, err = provider.ReadFile(ctx, "/etc/passwd")
	if err == nil || err.Error() != "path escapes base directory" {
		t.Errorf("expected path escape error, got %v", err)
	}

	// Exact base match attempt
	_, err = provider.ListDir(ctx, "")
	if err != nil {
		t.Errorf("expected successful list dir, got %v", err)
	}
}

func TestCloudFSProvider_Multitenancy(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloud_fs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	// Create context with claims
	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write file for tenant
	err = provider.WriteFile(ctx, "tenant_data.txt", []byte("secret"))
	if err != nil {
		t.Errorf("expected successful write, got %v", err)
	}

	// Verify file was written to the correct tenant directory
	expectedPath := filepath.Join(tempDir, "tenant-org-123", "tenant_data.txt")
	content, err := os.ReadFile(expectedPath)
	if err != nil || string(content) != "secret" {
		t.Errorf("expected to find file at %s with 'secret', got %s (err: %v)", expectedPath, string(content), err)
	}

	// Create context with different claims
	claims2 := &auth.Claims{OrganizationID: "org-456"}
	ctx2 := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims2)

	// Try to read the file from the first tenant
	_, err = provider.ReadFile(ctx2, "tenant_data.txt")
	if err == nil {
		t.Errorf("expected error reading other tenant's file")
	}

	// Create context with no claims
	ctx3 := context.Background()
	_, err = provider.ReadFile(ctx3, "tenant_data.txt")
	if err == nil || err.Error() != "unauthorized: missing organization ID in context" {
		t.Errorf("expected unauthorized error, got %v", err)
	}
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	provider, err := NewHybridFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	server := NewHybridFSMCP(provider)
	ctx := context.Background()

	// Write file via tool
	writeArgs := []byte(`{"path":"mcp.txt","content":"aGVsbG8="}`) // "hello" in base64
	_, err = server.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Errorf("expected successful write_file, got %v", err)
	}

	// Read file via tool
	readArgs := []byte(`{"path":"mcp.txt"}`)
	res, err := server.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Errorf("expected successful read_file, got %v", err)
	}
	if string(res.ResultData) != `{"content":"aGVsbG8="}` {
		t.Errorf("expected result data to be base64 hello, got %s", string(res.ResultData))
	}

	// Search files via tool
	searchArgs := []byte(`{"path":"","pattern":"mcp.txt"}`)
	res, err = server.CallTool(ctx, "search_files", searchArgs)
	if err != nil {
		t.Errorf("expected successful search_files, got %v", err)
	}
	if string(res.ResultData) != `{"matches":["mcp.txt"]}` {
		t.Errorf("expected result data to find mcp.txt, got %s", string(res.ResultData))
	}

	// List directory via tool
	listArgs := []byte(`{"path":""}`)
	_, err = server.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Errorf("expected successful list_directory, got %v", err)
	}
}
