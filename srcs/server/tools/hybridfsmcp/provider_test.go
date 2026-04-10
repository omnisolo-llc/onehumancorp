package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_Basic(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfsprovider_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := NewLocalFSProvider()
	ctx := context.Background()

	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("Expected exactly one file named test.txt")
	}
}

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfsprovider_traversal")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := NewLocalFSProvider()
	ctx := context.Background()

	_, err = provider.ReadFile(ctx, "../../../../../etc/passwd")
	if err == nil {
		t.Errorf("Expected error when attempting path traversal")
	} else if !strings.Contains(err.Error(), "path traversal attempt blocked") {
		t.Errorf("Expected traversal blocked error, got: %v", err)
	}
}

func TestCloudFSProvider_Basic(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfsprovider_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := NewCloudFSProvider()

	// Create a context with claims
	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err = provider.WriteFile(ctx, "data.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "data.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Verify the file was written to the tenant subfolder
	tenantPath := filepath.Join(tempDir, "tenant-1", "data.txt")
	if _, err := os.Stat(tenantPath); os.IsNotExist(err) {
		t.Errorf("Expected file to be created at %s, but it was not", tenantPath)
	}
}

func TestCloudFSProvider_MissingClaims(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfsprovider_missing_claims")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := NewCloudFSProvider()

	// No claims in context
	ctx := context.Background()

	_, err = provider.ReadFile(ctx, "data.txt")
	if err == nil {
		t.Errorf("Expected error when claims are missing")
	} else if err.Error() != "missing tenant context" {
		t.Errorf("Expected missing tenant context error, got: %v", err)
	}
}

func TestCloudFSProvider_PathTraversal(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfsprovider_traversal")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := NewCloudFSProvider()

	claims := &auth.Claims{
		OrganizationID: "tenant-1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Attempt to escape the tenant directory
	_, err = provider.ReadFile(ctx, "../tenant-2/data.txt")
	if err == nil {
		t.Errorf("Expected error when attempting path traversal")
	} else if !strings.Contains(err.Error(), "path traversal attempt blocked") {
		t.Errorf("Expected traversal blocked error, got: %v", err)
	}
}
