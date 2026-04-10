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
	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create LocalFSProvider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// Test path traversal protection
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Error("Expected error for path traversal, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create CloudFSProvider: %v", err)
	}

	// Create a context with claims
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := auth.ContextWithClaims(context.Background(), claims)

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Test unauthorized access (no claims)
	ctxNoClaims := context.Background()
	_, err = provider.ReadFile(ctxNoClaims, "test.txt")
	if err == nil {
		t.Error("Expected error for unauthorized access, got nil")
	}

	// Verify Tenant Isolation
	tenantDir := filepath.Join(tmpDir, "tenants", "tenant-1")
	if _, err := os.Stat(filepath.Join(tenantDir, "test.txt")); os.IsNotExist(err) {
		t.Error("Expected file to be created in tenant directory")
	}
}
