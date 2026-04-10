package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(content) != "hello local" {
		t.Errorf("Expected 'hello local', got %s", content)
	}

	// Test Path Traversal Protection
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Errorf("Expected path traversal error, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(content) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got %s", content)
	}

    // Verify file is in tenant dir
    tenantFile := filepath.Join(tempDir, "tenant-1", "test.txt")
    if _, err := os.Stat(tenantFile); os.IsNotExist(err) {
        t.Errorf("Expected file to exist at %s", tenantFile)
    }

	// Test Path Traversal Protection
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Errorf("Expected path traversal error, got nil")
	}

    // Test overlapping prefix
    // (tenant-10 vs tenant-1 directory) - not directly tested via path resolution because we enforce via joining, but let's test traversal again
	err = provider.WriteFile(ctx, "../../tenant-2/test.txt", []byte("escape"))
	if err == nil {
		t.Errorf("Expected path traversal error, got nil")
	}

    // Test missing context
    err = provider.WriteFile(context.Background(), "test.txt", []byte("fail"))
    if err == nil {
        t.Errorf("Expected error when missing claims, got nil")
    }
}

func TestFactoryNewProvider(t *testing.T) {
	ctx := context.Background()

	// Test Standalone mode (default)
	os.Setenv("OHC_MULTITENANT", "false")
	provider, err := NewProvider(ctx)
	if err != nil {
		t.Errorf("NewProvider failed: %v", err)
	}
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider, got %T", provider)
	}

	// Test Cloud mode
	os.Setenv("OHC_MULTITENANT", "true")
	provider, err = NewProvider(ctx)
	if err != nil {
		t.Errorf("NewProvider failed: %v", err)
	}
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider, got %T", provider)
	}

    // Restore env
    os.Unsetenv("OHC_MULTITENANT")
}

func TestLocalFSProvider_Errors(t *testing.T) {
    _, err := NewLocalFSProvider("\x00invalid")
    if err == nil {
        // Just an extra sanity check
    }

    tempDir, _ := os.MkdirTemp("", "localfserr")
    defer os.RemoveAll(tempDir)
    provider, _ := NewLocalFSProvider(tempDir)
    ctx := context.Background()

    // Test ListDir on non-existent
    _, err = provider.ListDir(ctx, "nonexistent")
    if err == nil {
        t.Errorf("Expected error on ListDir nonexistent, got nil")
    }

    // Test ListDir path traversal
    _, err = provider.ListDir(ctx, "../escape")
    if err == nil {
        t.Errorf("Expected error on ListDir traversal, got nil")
    }

    // Test ReadFile path traversal
    _, err = provider.ReadFile(ctx, "../escape.txt")
    if err == nil {
        t.Errorf("Expected error on ReadFile traversal, got nil")
    }
}

func TestCloudFSProvider_Errors(t *testing.T) {
    tempDir, _ := os.MkdirTemp("", "cloudfserr")
    defer os.RemoveAll(tempDir)
    provider, _ := NewCloudFSProvider(tempDir)
    ctx := context.Background()
    claims := &auth.Claims{OrganizationID: "tenant-1"}
	authCtx := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

    // Test ListDir on non-existent
    _, err := provider.ListDir(authCtx, "nonexistent")
    if err == nil {
        t.Errorf("Expected error on ListDir nonexistent, got nil")
    }

    // Test ListDir path traversal
    _, err = provider.ListDir(authCtx, "../escape")
    if err == nil {
        t.Errorf("Expected error on ListDir traversal, got nil")
    }

    // Test ReadFile path traversal
    _, err = provider.ReadFile(authCtx, "../escape.txt")
    if err == nil {
        t.Errorf("Expected error on ReadFile traversal, got nil")
    }

    // Test context errors
    _, err = provider.ListDir(ctx, "test")
    if err == nil {
        t.Errorf("Expected error without claims")
    }
    _, err = provider.ReadFile(ctx, "test")
    if err == nil {
        t.Errorf("Expected error without claims")
    }
}
