package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSServer(t *testing.T) {
	tmpDir := t.TempDir()
	server, err := NewHybridFSServer(tmpDir)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	ctx := context.Background()

	// Test write_file
	writeArgs := map[string]interface{}{
		"path":    "test.txt",
		"content": "hello world",
	}
	res, err := server.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}
	if res.Result == "" {
		t.Errorf("expected result, got empty")
	}

	// Test read_file
	readArgs := map[string]interface{}{
		"path": "test.txt",
	}
	res, err = server.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if res.Result != "hello world" {
		t.Errorf("expected 'hello world', got '%s'", res.Result)
	}

	// Test list_directory
	listArgs := map[string]interface{}{
		"path": ".",
	}
	res, err = server.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("failed to list directory: %v", err)
	}
	if res.Result != `["test.txt"]` {
		t.Errorf("expected [\"test.txt\"], got %s", res.Result)
	}
}

func TestCloudFSServer(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	tmpDir := t.TempDir()
	server, err := NewHybridFSServer(tmpDir)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Ensure tenant dir is isolated
	writeArgs := map[string]interface{}{
		"path":    "data.txt",
		"content": "secret",
	}
	_, err = server.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	// Verify file is under tenant dir
	b, err := os.ReadFile(filepath.Join(tmpDir, "tenant-1", "data.txt"))
	if err != nil {
		t.Fatalf("file not found in tenant dir: %v", err)
	}
	if string(b) != "secret" {
		t.Errorf("expected 'secret', got '%s'", string(b))
	}
}

func TestLocalFSServer_PathTraversal(t *testing.T) {
	tmpDir := t.TempDir()

	// Create another directory to act as a target for path traversal
	otherDir := filepath.Join(filepath.Dir(tmpDir), filepath.Base(tmpDir)+"10")
	os.MkdirAll(otherDir, 0755)
	defer os.RemoveAll(otherDir)

	secretFile := filepath.Join(otherDir, "secret.txt")
	os.WriteFile(secretFile, []byte("super secret"), 0644)
	defer os.Remove(secretFile)

	server, err := NewHybridFSServer(tmpDir)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	ctx := context.Background()

	// Attempt path traversal using relative path
	readArgs := map[string]interface{}{
		"path": "../" + filepath.Base(otherDir) + "/secret.txt",
	}
	_, err = server.CallTool(ctx, "read_file", readArgs)
	if err == nil {
		t.Fatalf("expected error from path traversal, got nil")
	}
	if err.Error() != "access denied: path escapes workspace" {
		t.Errorf("expected 'access denied: path escapes workspace', got %v", err)
	}
}

func TestCloudFSServer_CrossTenantAccess(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	tmpDir := t.TempDir()
	server, err := NewHybridFSServer(tmpDir)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	// Create data for tenant-10
	tenant10Dir := filepath.Join(tmpDir, "tenant-10")
	os.MkdirAll(tenant10Dir, 0755)
	os.WriteFile(filepath.Join(tenant10Dir, "data.txt"), []byte("tenant 10 data"), 0644)

	// Auth as tenant-1
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Attempt to access tenant-10 from tenant-1
	readArgs := map[string]interface{}{
		"path": "../tenant-10/data.txt",
	}
	_, err = server.CallTool(ctx, "read_file", readArgs)
	if err == nil {
		t.Fatalf("expected error from cross-tenant access, got nil")
	}
	if err.Error() != "access denied: cross-tenant access attempt" {
		t.Errorf("expected 'access denied: cross-tenant access attempt', got %v", err)
	}
}
