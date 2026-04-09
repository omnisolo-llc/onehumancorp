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
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", "hello world")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if content != "hello world" {
		t.Errorf("expected 'hello world', got '%s'", content)
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", infos)
	}

	// Test Path Escaping (Directory Traversal)
	err = provider.WriteFile(ctx, "../escape.txt", "hacked")
	if err == nil {
		t.Errorf("expected error when path escapes base dir, got nil")
	}

	// Test Absolute Path
	err = provider.WriteFile(ctx, "/tmp/escape.txt", "hacked")
	if err == nil {
		t.Errorf("expected error when using absolute path, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	// Test without claims (should fail)
	ctx := context.Background()
	_, err = provider.ReadFile(ctx, "test.txt")
	if err == nil {
		t.Errorf("expected error without claims, got nil")
	}

	// Create test context with claims
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctxWithClaims, "test.txt", "hello cloud")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}

	// Verify it wrote to the tenant directory
	expectedPath := filepath.Join(tempDir, "tenant-1", "test.txt")
	if _, err := os.Stat(expectedPath); os.IsNotExist(err) {
		t.Errorf("file not created in tenant dir: %s", expectedPath)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctxWithClaims, "test.txt")
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if content != "hello cloud" {
		t.Errorf("expected 'hello cloud', got '%s'", content)
	}

	// Test Path Escaping (Directory Traversal)
	err = provider.WriteFile(ctxWithClaims, "../tenant-2/escape.txt", "hacked")
	if err == nil {
		t.Errorf("expected error when path escapes tenant dir, got nil")
	}
}
