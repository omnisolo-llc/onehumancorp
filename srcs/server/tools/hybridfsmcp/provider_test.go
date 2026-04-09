package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Write
	err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read
	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil || string(data) != "hello local" {
		t.Fatalf("ReadFile failed or mismatch: %v, %s", err, string(data))
	}

	// List
	entries, err := provider.ListDir(ctx, nil, ".")
	if err != nil || len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Fatalf("ListDir failed or mismatch")
	}

	// Traversal
	err = provider.WriteFile(ctx, nil, "../outside.txt", []byte("hack"))
	if err == nil {
		t.Fatalf("Expected path traversal error")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewCloudFSProvider(tmpDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant1"}

	// Unauthorized
	err := provider.WriteFile(ctx, nil, "test.txt", []byte("fail"))
	if err == nil {
		t.Fatalf("Expected unauthorized error")
	}

	// Write
	err = provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify Tenant Isolation
	content, err := os.ReadFile(filepath.Join(tmpDir, "tenant1", "test.txt"))
	if err != nil || string(content) != "hello cloud" {
		t.Fatalf("Tenant isolation failed: %v", err)
	}

	// Read
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil || string(data) != "hello cloud" {
		t.Fatalf("ReadFile failed or mismatch")
	}

	// List
	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil || len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Fatalf("ListDir failed or mismatch")
	}

	// Traversal
	err = provider.WriteFile(ctx, claims, "../outside.txt", []byte("hack"))
	if err == nil {
		t.Fatalf("Expected path traversal error")
	}
}

func TestNewProvider(t *testing.T) {
	tmpDir := t.TempDir()
	p1 := NewProvider(true, tmpDir)
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Fatalf("Expected LocalFSProvider")
	}

	p2 := NewProvider(false, tmpDir)
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Fatalf("Expected CloudFSProvider")
	}
}
