package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_ReadWriteListSearch(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Write
	err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Read
	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("expected 'hello local', got %s", string(data))
	}

	// List
	entries, err := provider.ListDir(ctx, nil, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected [test.txt], got %v", entries)
	}

	// Search
	err = provider.WriteFile(ctx, nil, "subdir/match_me.txt", []byte("match"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	matches, err := provider.SearchFiles(ctx, nil, ".", "match")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(matches) != 1 || !strings.Contains(matches[0], "match_me.txt") {
		t.Errorf("expected [subdir/match_me.txt], got %v", matches)
	}
}

func TestLocalFSProvider_TraversalProtection(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Attempt path traversal
	err := provider.WriteFile(ctx, nil, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Fatal("expected error due to path traversal, got nil")
	}

	_, err = provider.ReadFile(ctx, nil, "../escape.txt")
	if err == nil {
		t.Fatal("expected error due to path traversal, got nil")
	}

	_, err = provider.ListDir(ctx, nil, "../")
	if err == nil {
		t.Fatal("expected error due to path traversal, got nil")
	}

	_, err = provider.SearchFiles(ctx, nil, "../", "txt")
	if err == nil {
		t.Fatal("expected error due to path traversal, got nil")
	}
}

func TestCloudFSProvider_TenantScoping(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()

	// Should fail with nil claims
	err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello cloud"))
	if err == nil || !strings.Contains(err.Error(), "missing claims") {
		t.Fatalf("expected missing claims error, got %v", err)
	}

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}

	err = provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Verify the file was written to the tenant scoped directory
	expectedPath := filepath.Join(tempDir, "tenant", "tenant-123", "test.txt")
	data, err := os.ReadFile(expectedPath)
	if err != nil {
		t.Fatalf("failed to read expected tenant scoped file: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %s", string(data))
	}

	// Read
	readData, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(readData) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %s", string(readData))
	}
}

func TestCloudFSProvider_TraversalProtection(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}

	// Attempt path traversal out of the tenant dir
	err := provider.WriteFile(ctx, claims, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Fatal("expected error due to path traversal, got nil")
	}
}
