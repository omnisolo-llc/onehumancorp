package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_WriteAndRead(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test Write
	err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test Read
	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello world" {
		t.Errorf("Expected 'hello world', got '%s'", string(data))
	}
}

func TestLocalFSProvider_ListDir(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	_ = provider.WriteFile(ctx, nil, "file1.txt", []byte("1"))
	_ = provider.WriteFile(ctx, nil, "dir/file2.txt", []byte("2"))

	infos, err := provider.ListDir(ctx, nil, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}

	if len(infos) != 2 {
		t.Errorf("Expected 2 items, got %d", len(infos))
	}

	// Read error directory
	_, err = provider.ListDir(ctx, nil, "nonexistent")
	if err == nil {
		t.Errorf("Expected error reading nonexistent directory")
	}
}

func TestLocalFSProvider_OutOfBounds(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	_, err := provider.ReadFile(ctx, nil, "../outside.txt")
	if err == nil {
		t.Errorf("Expected error when accessing out of bounds path")
	}
	if err != nil && !strings.Contains(err.Error(), "out of bounds") {
		t.Errorf("Expected 'out of bounds' error, got: %v", err)
	}

	err = provider.WriteFile(ctx, nil, "../outside.txt", []byte("hax"))
	if err == nil {
		t.Errorf("Expected error when accessing out of bounds path")
	}

	_, err = provider.ListDir(ctx, nil, "../")
	if err == nil {
		t.Errorf("Expected error when accessing out of bounds path")
	}
}

func TestCloudFSProvider_WriteAndRead(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-A"}

	// Test Write
	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it wrote to tenant dir
	b, err := os.ReadFile(filepath.Join(tempDir, "tenant-A", "test.txt"))
	if err != nil || string(b) != "hello cloud" {
		t.Errorf("Failed to write to correct tenant directory")
	}

	// Test Read
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}
}

func TestCloudFSProvider_ListDir(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-A"}

	_ = provider.WriteFile(ctx, claims, "file1.txt", []byte("1"))

	infos, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}

	if len(infos) != 1 {
		t.Errorf("Expected 1 items, got %d", len(infos))
	}

	// Read error directory
	_, err = provider.ListDir(ctx, claims, "nonexistent")
	if err == nil {
		t.Errorf("Expected error reading nonexistent directory")
	}
}

func TestCloudFSProvider_MissingClaims(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()

	_, err := provider.ReadFile(ctx, nil, "test.txt")
	if err == nil {
		t.Errorf("Expected error when claims are nil")
	}

	err = provider.WriteFile(ctx, nil, "test.txt", []byte("hax"))
	if err == nil {
		t.Errorf("Expected error when claims are nil")
	}

	_, err = provider.ListDir(ctx, nil, ".")
	if err == nil {
		t.Errorf("Expected error when claims are nil")
	}

	claims := &auth.Claims{OrganizationID: ""}
	_, err = provider.ReadFile(ctx, claims, "test.txt")
	if err == nil {
		t.Errorf("Expected error when OrganizationID is empty")
	}
}

func TestCloudFSProvider_CrossTenantBounds(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()
	claimsA := &auth.Claims{OrganizationID: "tenant-A"}

	_ = provider.WriteFile(ctx, claimsA, "test.txt", []byte("secret"))

	claimsB := &auth.Claims{OrganizationID: "tenant-B"}
	_, err := provider.ReadFile(ctx, claimsB, "../tenant-A/test.txt")
	if err == nil {
		t.Errorf("Expected error when accessing cross tenant path")
	}
	if err != nil && !strings.Contains(err.Error(), "cross-tenant access forbidden") {
		t.Errorf("Expected 'cross-tenant access forbidden' error, got: %v", err)
	}
}
