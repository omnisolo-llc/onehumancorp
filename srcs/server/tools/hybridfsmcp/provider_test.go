package hybridfsmcp

import (
	"path/filepath"
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got %s", string(data))
	}

	// Escaping path
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error when path escapes workspace")
	}

	// List
	err = provider.WriteFile(ctx, "dir/a.txt", []byte("a"))
	if err != nil {
		t.Fatal(err)
	}
	entries, err := provider.ListDir(ctx, "dir")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "a.txt" {
		t.Errorf("expected ['a.txt'], got %v", entries)
	}

	// Search
	matches, err := provider.SearchFiles(ctx, ".", "*.txt")
	if err != nil {
		t.Errorf("SearchFiles failed: %v", err)
	}
	if len(matches) != 2 {
		t.Errorf("expected 2 matches, got %v", matches)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	// Without claims should fail
	ctx := context.Background()
	_, err = provider.ReadFile(ctx, "test.txt")
	if err == nil {
		t.Errorf("expected error without tenant context")
	}

	// With claims
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Write
	err = provider.WriteFile(ctxWithClaims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Read
	data, err := provider.ReadFile(ctxWithClaims, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %s", string(data))
	}

	// List
	entries, err := provider.ListDir(ctxWithClaims, ".")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	// Search
	matches, err := provider.SearchFiles(ctxWithClaims, ".", "*.txt")
	if err != nil {
		t.Errorf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", matches)
	}
}

func TestFactory(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")
	p1 := NewFileSystemProvider("/tmp")
	if _, ok := p1.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider")
	}

	t.Setenv("OHC_MULTITENANT", "false")
	p2 := NewFileSystemProvider("/tmp")
	if _, ok := p2.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider")
	}
}

func TestDirectoryTraversalLeak(t *testing.T) {
	// Setup workspace mimicking `/tmp/tenant1` and another directory `/tmp/tenant10`
	tmpDir, err := os.MkdirTemp("", "mcp_traversal")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	tenant1Dir := filepath.Join(tmpDir, "tenant1")
	tenant10Dir := filepath.Join(tmpDir, "tenant10")

	os.MkdirAll(tenant1Dir, 0755)
	os.MkdirAll(tenant10Dir, 0755)
	os.WriteFile(filepath.Join(tenant10Dir, "secret.txt"), []byte("secret"), 0644)

	provider := NewLocalFSProvider(tenant1Dir)

	// Malicious path that evaluates to /tmp/tenant10/secret.txt
	// For LocalFSProvider, this gets passed to filepath.Join(p.WorkspaceDir, path)
	// E.g., workspace = /tmp/tenant1
	// path = ../tenant10/secret.txt
	// cleanPath = /tmp/tenant10/secret.txt

	ctx := context.Background()
	_, err = provider.ReadFile(ctx, "../tenant10/secret.txt")
	if err == nil {
		t.Errorf("expected error reading outside workspace with similar prefix, but succeeded")
	}
}
