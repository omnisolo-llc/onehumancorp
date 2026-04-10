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

	provider := &LocalFSProvider{BaseDir: tempDir}
	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(content))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test absolute path rejection
	err = provider.WriteFile(ctx, "/etc/passwd", []byte("hack"))
	if err == nil {
		t.Error("Expected error for absolute path")
	}

	// Test path traversal rejection
	err = provider.WriteFile(ctx, "../escape.txt", []byte("hack"))
	if err == nil {
		t.Error("Expected error for path traversal")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := &CloudFSProvider{BaseDir: tempDir}

	// Create context with claims
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify tenant isolation
	if _, err := os.Stat(filepath.Join(tempDir, "tenant-1", "test.txt")); os.IsNotExist(err) {
		t.Error("File was not created in tenant directory")
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(content))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test absolute path rejection
	err = provider.WriteFile(ctx, "/etc/passwd", []byte("hack"))
	if err == nil {
		t.Error("Expected error for absolute path")
	}

	// Test path traversal rejection
	err = provider.WriteFile(ctx, "../escape.txt", []byte("hack"))
	if err == nil {
		t.Error("Expected error for path traversal")
	}

	// Test no claims
	emptyCtx := context.Background()
	_, err = provider.ReadFile(emptyCtx, "test.txt")
	if err == nil {
		t.Error("Expected error for missing claims")
	}
}

func TestNewFileSystemProvider(t *testing.T) {
	_, err := NewFileSystemProvider("OHC_STANDALONE", "/tmp")
	if err != nil {
		t.Errorf("Failed to create standalone provider: %v", err)
	}

	_, err = NewFileSystemProvider("OHC_MULTITENANT", "/tmp")
	if err != nil {
		t.Errorf("Failed to create multitenant provider: %v", err)
	}

	_, err = NewFileSystemProvider("UNKNOWN", "/tmp")
	if err == nil {
		t.Error("Expected error for unknown mode")
	}
}
