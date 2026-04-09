package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got %s", string(data))
	}

	// List dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("Expected 1 entry 'test.txt', got %v", entries)
	}

	// Traversal attempt
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("Expected error for path traversal")
	}

	// Absolute path
	_, err = provider.ReadFile(ctx, "/etc/passwd")
	if err == nil {
		t.Errorf("Expected error for absolute path")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	// Context with claims
	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got %s", string(data))
	}

	// Verify tenant isolation on disk
	onDisk, err := os.ReadFile(filepath.Join(tempDir, "tenant-123", "test.txt"))
	if err != nil {
		t.Fatalf("Failed to read file from disk directly: %v", err)
	}
	if string(onDisk) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got %s", string(onDisk))
	}

	// List dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("Expected 1 entry 'test.txt', got %v", entries)
	}

	// No claims
	emptyCtx := context.Background()
	_, err = provider.ReadFile(emptyCtx, "test.txt")
	if err == nil {
		t.Errorf("Expected error for missing claims")
	}

	// Traversal attempt
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("Expected error for path traversal")
	}
}

func TestFactory(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	provider := NewProvider("/tmp")
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider, got %T", provider)
	}

	os.Setenv("OHC_STANDALONE", "false")
	providerCloud := NewProvider("/tmp")
	if _, ok := providerCloud.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider, got %T", providerCloud)
	}
}

func TestSearchFiles_Local(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	provider.WriteFile(ctx, "test_search1.txt", []byte("mcp test1"))
	provider.WriteFile(ctx, "other_file.txt", []byte("mcp test2"))
	provider.WriteFile(ctx, "sub/test_search2.txt", []byte("mcp test3"))

	matches, err := provider.SearchFiles(ctx, ".", "search")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(matches) != 2 {
		t.Errorf("Expected 2 matches, got %v", matches)
	}
}

func TestSearchFiles_Cloud(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	provider.WriteFile(ctx, "test_search1.txt", []byte("mcp test1"))
	provider.WriteFile(ctx, "other_file.txt", []byte("mcp test2"))
	provider.WriteFile(ctx, "sub/test_search2.txt", []byte("mcp test3"))

	matches, err := provider.SearchFiles(ctx, ".", "search")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(matches) != 2 {
		t.Errorf("Expected 2 matches, got %v", matches)
	}
}
