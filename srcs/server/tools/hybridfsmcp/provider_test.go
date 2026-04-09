package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	if !provider.IsLocal() {
		t.Errorf("Expected IsLocal to return true")
	}

	ctx := context.Background()

	// Test writing and reading a file
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("Failed to write file: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("Failed to read file: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(data))
	}

	// Test boundary constraints
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Errorf("Expected error writing outside workspace")
	}

    // Test sibling boundary traversal constraint
    err = provider.WriteFile(ctx, "../" + tempDir + "-sibling/escape.txt", []byte("bad"))
    if err == nil {
        t.Errorf("Expected error writing to sibling workspace")
    }

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test SearchFiles
	matches, err := provider.SearchFiles(ctx, ".", "test.*")
	if err != nil {
		t.Errorf("Failed to search files: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", matches)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	if provider.IsLocal() {
		t.Errorf("Expected IsLocal to return false")
	}

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Setup tenant dir manually since provider won't implicitly create the tenant root directory itself
	// unless a file is written. The WriteFile will do it.

	// Test writing and reading
	err = provider.WriteFile(ctx, "data.txt", []byte("cloud"))
	if err != nil {
		t.Errorf("Failed to write file: %v", err)
	}

	data, err := provider.ReadFile(ctx, "data.txt")
	if err != nil {
		t.Errorf("Failed to read file: %v", err)
	}
	if string(data) != "cloud" {
		t.Errorf("Expected 'cloud', got '%s'", string(data))
	}

	// Test boundary constraints
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Errorf("Expected error writing outside tenant workspace")
	}

    // Test sibling boundary constraints
	err = provider.WriteFile(ctx, "../tenant-10/escape.txt", []byte("bad"))
	if err == nil {
		t.Errorf("Expected error writing to sibling tenant workspace")
	}

	// Test isolation
	ctx2 := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "tenant-2"})
	_, err = provider.ReadFile(ctx2, "data.txt")
	if err == nil {
		t.Errorf("Expected error reading from another tenant's directory")
	}

	// Test missing claims
	ctx3 := context.Background()
	_, err = provider.ReadFile(ctx3, "data.txt")
	if err == nil {
		t.Errorf("Expected error missing claims")
	}

	// Test invalid claim format
	ctx4 := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "../invalid"})
	_, err = provider.ReadFile(ctx4, "data.txt")
	if err == nil {
		t.Errorf("Expected error invalid claim format")
	}

	// ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "data.txt" {
		t.Errorf("Expected ['data.txt'], got %v", entries)
	}

	// SearchFiles
	matches, err := provider.SearchFiles(ctx, ".", ".*\\.txt")
	if err != nil {
		t.Errorf("Failed to search files: %v", err)
	}
	if len(matches) != 1 || matches[0] != "data.txt" {
		t.Errorf("Expected ['data.txt'], got %v", matches)
	}
}

func TestNewFileSystemProvider(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	p1, _ := NewFileSystemProvider(".")
	if !p1.IsLocal() {
		t.Errorf("Expected LocalFSProvider")
	}

	os.Setenv("OHC_STANDALONE", "false")
	p2, _ := NewFileSystemProvider(".")
	if p2.IsLocal() {
		t.Errorf("Expected CloudFSProvider")
	}
}
