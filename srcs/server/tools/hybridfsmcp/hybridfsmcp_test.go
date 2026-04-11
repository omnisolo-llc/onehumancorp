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

	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("Expected 'hello', got '%s'", string(data))
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Fatalf("Expected 1 entry 'test.txt', got %v", entries)
	}

	// Escape attempt
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Fatal("Expected error on path escape")
	}

	// Create sibling directory to test prefix bypass
	siblingDir := filepath.Join(filepath.Dir(tmpDir), filepath.Base(tmpDir)+"-secrets")
	os.MkdirAll(siblingDir, 0755)
	defer os.RemoveAll(siblingDir)
	os.WriteFile(filepath.Join(siblingDir, "secret.txt"), []byte("secret"), 0644)

	// Attempt to access sibling
	_, err = provider.ReadFile(ctx, "../"+filepath.Base(siblingDir)+"/secret.txt")
	if err == nil {
		t.Fatal("Expected error on prefix bypass escape")
	}
}

func TestCloudFSProvider(t *testing.T) {
	baseDir := t.TempDir()
	tenantID := "tenant-123"
	provider := NewCloudFSProvider(baseDir)

	claims := &auth.Claims{OrganizationID: tenantID}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err := provider.WriteFile(ctx, "test2.txt", []byte("world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test2.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "world" {
		t.Fatalf("Expected 'world', got '%s'", string(data))
	}

	// Check actual path
	actualPath := filepath.Join(baseDir, tenantID, "test2.txt")
	if _, err := os.Stat(actualPath); os.IsNotExist(err) {
		t.Fatalf("File was not written to tenant directory")
	}

	// Escape attempt
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Fatal("Expected error on path escape")
	}

	// Unauthenticated test
	unauthCtx := context.Background()
	_, err = provider.ReadFile(unauthCtx, "test2.txt")
	if err == nil {
		t.Fatal("Expected error on unauthenticated access")
	}
}

func TestServerFactory(t *testing.T) {
	// Test standalone
	os.Setenv("OHC_MULTITENANT", "false")
	os.Setenv("OHC_WORKSPACE", t.TempDir())
	server, err := NewServer()
	if err != nil {
		t.Fatalf("NewServer failed: %v", err)
	}
	if _, ok := server.Provider.(*LocalFSProvider); !ok {
		t.Fatal("Expected LocalFSProvider")
	}

	// Test cloud
	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("OHC_CLOUD_BASE_DIR", t.TempDir())
	server, err = NewServer()
	if err != nil {
		t.Fatalf("NewServer failed: %v", err)
	}
	if _, ok := server.Provider.(*CloudFSProvider); !ok {
		t.Fatal("Expected CloudFSProvider")
	}
}
