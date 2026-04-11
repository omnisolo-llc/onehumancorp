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
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(content) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(content))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("ListDir unexpected output: %v", entries)
	}

	// Test Path Traversal
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Error("Expected error for path traversal, got none")
	}

	// Test Absolute Path
	_, err = provider.ReadFile(ctx, "/etc/passwd")
	if err == nil {
		t.Error("Expected error for absolute path, got none")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(content) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(content))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("ListDir unexpected output: %v", entries)
	}

	// Test Path Traversal
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Error("Expected error for path traversal, got none")
	}

	// Test Absolute Path
	_, err = provider.ReadFile(ctx, "/etc/passwd")
	if err == nil {
		t.Error("Expected error for absolute path, got none")
	}

	// Verify Tenant Isolation
	tenantDir := filepath.Join(tempDir, "tenant1")
	if _, err := os.Stat(filepath.Join(tenantDir, "test.txt")); os.IsNotExist(err) {
		t.Error("File was not written to tenant specific directory")
	}

	// Test Missing Claims
	_, err = provider.ReadFile(context.Background(), "test.txt")
	if err == nil {
		t.Error("Expected error when claims are missing, got none")
	}
}

func TestSiblingDirectoryTraversal(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "sibling_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	tenant1Dir := filepath.Join(tempDir, "tenant1")
	tenant10Dir := filepath.Join(tempDir, "tenant10")

	os.MkdirAll(tenant1Dir, 0755)
	os.MkdirAll(tenant10Dir, 0755)

	os.WriteFile(filepath.Join(tenant10Dir, "secrets.txt"), []byte("secret10"), 0644)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Attempt to access tenant10 from tenant1 using path traversal
	_, err = provider.ReadFile(ctx, "../tenant10/secrets.txt")
	if err == nil {
		t.Error("VULNERABILITY: Sibling directory traversal succeeded!")
	}
}
