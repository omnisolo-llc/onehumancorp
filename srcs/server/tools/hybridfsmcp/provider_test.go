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
	os.Setenv("OHC_WORKSPACE_DIR", tempDir)
	defer os.Unsetenv("OHC_WORKSPACE_DIR")

	provider := NewLocalFSProvider()
	ctx := context.Background()

	// Write
	err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read
	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil || string(data) != "hello" {
		t.Fatalf("ReadFile failed or wrong data: %v, %s", err, string(data))
	}

	// List
	entries, err := provider.ListDir(ctx, nil, ".")
	if err != nil || len(entries) == 0 {
		t.Fatalf("ListDir failed or empty: %v", err)
	}

	// Search
	matches, err := provider.SearchFiles(ctx, nil, ".", "test")
	if err != nil || len(matches) == 0 {
		t.Fatalf("SearchFiles failed or empty: %v", err)
	}

	// Traversal
	_, err = provider.ReadFile(ctx, nil, "../../../etc/passwd")
	if err == nil {
		t.Fatalf("Expected directory traversal to fail")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	os.Setenv("OHC_TENANT_PV_DIR", tempDir)
	defer os.Unsetenv("OHC_TENANT_PV_DIR")

	provider := NewCloudFSProvider()
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}

	// Missing claims test
	err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello"))
	if err == nil {
		t.Fatalf("Expected missing claims to fail")
	}

	// Write
	err = provider.WriteFile(ctx, claims, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Path formatting
	expectedPath := filepath.Join(tempDir, "tenant-1", "test.txt")
	if _, err := os.Stat(expectedPath); os.IsNotExist(err) {
		t.Fatalf("File not found at tenant scope path: %s", expectedPath)
	}

	// Read
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil || string(data) != "hello" {
		t.Fatalf("ReadFile failed or wrong data: %v", err)
	}

	// List
	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil || len(entries) == 0 {
		t.Fatalf("ListDir failed or empty: %v", err)
	}

	// Search
	matches, err := provider.SearchFiles(ctx, claims, ".", "test")
	if err != nil || len(matches) == 0 {
		t.Fatalf("SearchFiles failed or empty: %v", err)
	}

	// Traversal
	_, err = provider.ReadFile(ctx, claims, "../../../etc/passwd")
	if err == nil {
		t.Fatalf("Expected directory traversal to fail")
	}
}
