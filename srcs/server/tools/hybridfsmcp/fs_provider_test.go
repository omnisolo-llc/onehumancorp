package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test WriteFile
	err := provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got %s", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("Unexpected dir entries: %v", entries)
	}

	// Test path traversal
	err = provider.WriteFile(ctx, "../outside.txt", []byte("escape"))
	if err == nil || !strings.Contains(err.Error(), "escapes workspace root") {
		t.Errorf("Expected path traversal error, got %v", err)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Verify it wrote to the tenant directory
	data, err := os.ReadFile(filepath.Join(tempDir, "tenant-123", "test.txt"))
	if err != nil {
		t.Fatalf("Failed to read underlying file: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got %s", string(data))
	}

	// Test ReadFile
	data, err = provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file via provider: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got %s", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("Unexpected dir entries: %v", entries)
	}

	// Test path traversal
	err = provider.WriteFile(ctx, "../outside.txt", []byte("escape"))
	if err == nil || !strings.Contains(err.Error(), "escapes tenant root") {
		t.Errorf("Expected path traversal error, got %v", err)
	}

	// Test missing auth claims
	badCtx := context.Background()
	_, err = provider.ReadFile(badCtx, "test.txt")
	if err == nil {
		t.Errorf("Expected error for missing claims, got nil")
	}
}

func TestNewProviderFactory(t *testing.T) {
	tempDir := t.TempDir()

	t.Setenv("OHC_STANDALONE", "true")
	p1 := NewProviderFactory(tempDir)
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider in standalone mode")
	}

	t.Setenv("OHC_STANDALONE", "false")
	p2 := NewProviderFactory(tempDir)
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider in cloud mode")
	}
}
