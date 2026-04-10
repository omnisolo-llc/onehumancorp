package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := &LocalFSProvider{BaseDir: tmpDir}
	ctx := context.Background()

	// Test valid write/read
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got %q", string(data))
	}

	// Test path traversal prevention
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Error("expected error for path traversal, got nil")
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("unexpected ListDir result: %v", entries)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := &CloudFSProvider{BaseDir: tmpDir}
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test invalid context (no claims)
	ctxInvalid := context.Background()
	err = provider.WriteFile(ctxInvalid, "test.txt", []byte("fail"))
	if err == nil {
		t.Error("expected error without claims, got nil")
	}

	// Test valid write/read with claims
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %q", string(data))
	}

	// Ensure the file is actually under the tenant dir
	tenantDir := filepath.Join(tmpDir, "tenant1")
	_, err = os.Stat(filepath.Join(tenantDir, "test.txt"))
	if err != nil {
		t.Errorf("expected file in tenant dir, got err: %v", err)
	}

	// Test path traversal prevention
	err = provider.WriteFile(ctx, "../tenant2/escape.txt", []byte("escape"))
	if err == nil {
		t.Error("expected error for path traversal, got nil")
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("unexpected ListDir result: %v", entries)
	}
}

func TestNewProvider(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	p := NewProvider()
	if _, ok := p.(*CloudFSProvider); !ok {
		t.Error("expected CloudFSProvider")
	}

	os.Setenv("OHC_MULTITENANT", "false")
	p2 := NewProvider()
	if _, ok := p2.(*LocalFSProvider); !ok {
		t.Error("expected LocalFSProvider")
	}
}

func TestLocalFSProvider_Errors(t *testing.T) {
	tmpDir, _ := os.MkdirTemp("", "localfs")
	defer os.RemoveAll(tmpDir)

	provider := &LocalFSProvider{BaseDir: tmpDir}
	ctx := context.Background()

	// ListDir non existent
	_, err := provider.ListDir(ctx, "does_not_exist")
	if err == nil {
		t.Error("expected err on ListDir")
	}

	// ReadFile non existent
	_, err = provider.ReadFile(ctx, "does_not_exist.txt")
	if err == nil {
		t.Error("expected err on ReadFile")
	}
}

func TestCloudFSProvider_Errors(t *testing.T) {
	tmpDir, _ := os.MkdirTemp("", "cloudfs")
	defer os.RemoveAll(tmpDir)

	provider := &CloudFSProvider{BaseDir: tmpDir}
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// ListDir non existent
	_, err := provider.ListDir(ctx, "does_not_exist")
	if err == nil {
		t.Error("expected err on ListDir")
	}

	// ReadFile non existent
	_, err = provider.ReadFile(ctx, "does_not_exist.txt")
	if err == nil {
		t.Error("expected err on ReadFile")
	}
}
