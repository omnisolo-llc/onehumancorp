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

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test WriteFile in subdir
	err = provider.WriteFile(ctx, "subdir/test2.txt", []byte("hello subdir"))
	if err != nil {
		t.Fatalf("WriteFile subdir failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("expected 'hello local', got '%s'", string(data))
	}

	// Test ReadFile with directory traversal
	_, err = provider.ReadFile(ctx, "../test.txt")
	if err == nil {
		t.Errorf("ReadFile with directory traversal should have failed")
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 2 { // test.txt, subdir
		t.Errorf("expected 2 entries, got %d", len(entries))
	}

	// Test SearchFiles
	results, err := provider.SearchFiles(ctx, ".", "*.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(results) != 2 {
		t.Errorf("expected 2 search results, got %d", len(results))
	}

	foundTestTxt := false
	foundTest2Txt := false
	for _, r := range results {
		if r == "test.txt" {
			foundTestTxt = true
		}
		// Windows may return backslashes or slashes depending on ToSlash
		if r == "subdir/test2.txt" {
			foundTest2Txt = true
		}
	}
	if !foundTestTxt || !foundTest2Txt {
		t.Errorf("expected test.txt and subdir/test2.txt in results, got %v", results)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test without claims
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err == nil {
		t.Errorf("WriteFile without claims should have failed")
	}

	// Create a context with claims
	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctxWithClaims, "test.txt", []byte("hello cloud tenant"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test WriteFile in subdir
	err = provider.WriteFile(ctxWithClaims, "subdir/test2.txt", []byte("hello cloud subdir"))
	if err != nil {
		t.Fatalf("WriteFile subdir failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctxWithClaims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud tenant" {
		t.Errorf("expected 'hello cloud tenant', got '%s'", string(data))
	}

	// Test ReadFile with directory traversal
	_, err = provider.ReadFile(ctxWithClaims, "../test.txt")
	if err == nil {
		t.Errorf("ReadFile with directory traversal should have failed")
	}

	// Verify file is actually in the tenant subdirectory
	_, err = os.Stat(filepath.Join(tempDir, "tenant-123", "test.txt"))
	if err != nil {
		t.Errorf("file was not written to tenant directory: %v", err)
	}

	// Test ListDir
	entries, err := provider.ListDir(ctxWithClaims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 2 {
		t.Errorf("expected 2 entries, got %d", len(entries))
	}

	// Test SearchFiles
	results, err := provider.SearchFiles(ctxWithClaims, ".", "*.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(results) != 2 {
		t.Errorf("expected 2 search results, got %d", len(results))
	}

	// Create another tenant to ensure isolation
	claims2 := &auth.Claims{
		OrganizationID: "tenant-456",
	}
	ctxWithClaims2 := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims2)

	// Should not find tenant-123's files
	results2, err := provider.SearchFiles(ctxWithClaims2, ".", "*.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(results2) != 0 {
		t.Errorf("expected 0 search results for new tenant, got %d", len(results2))
	}

	// Write for tenant-456
	err = provider.WriteFile(ctxWithClaims2, "other.txt", []byte("hello tenant 456"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	results2, err = provider.SearchFiles(ctxWithClaims2, ".", "*.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(results2) != 1 || results2[0] != "other.txt" {
		t.Errorf("expected 1 result 'other.txt', got %v", results2)
	}
}
