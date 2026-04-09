package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	dir, _ := os.MkdirTemp("", "localfs")
	defer os.RemoveAll(dir)

	provider, err := NewLocalFSProvider(dir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	ctx := context.Background()

	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil || string(data) != "hello" {
		t.Fatalf("ReadFile failed: %v, %s", err, string(data))
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil || len(entries) == 0 {
		t.Fatalf("ListDir failed: %v", err)
	}

	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Fatalf("Expected error for path escape")
	}
}

func TestCloudFSProvider(t *testing.T) {
	dir, _ := os.MkdirTemp("", "cloudfs")
	defer os.RemoveAll(dir)

	provider, err := NewCloudFSProvider(dir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	// Use ContextWithValue directly as defined in middleware.go: ClaimsContextKeyForTest
	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil || string(data) != "hello cloud" {
		t.Fatalf("ReadFile failed: %v, %s", err, string(data))
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil || len(entries) == 0 {
		t.Fatalf("ListDir failed: %v", err)
	}

	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Fatalf("Expected error for path escape")
	}

	err = provider.WriteFile(context.Background(), "test2.txt", []byte("fail"))
	if err == nil {
		t.Fatalf("Expected error for missing claims")
	}
}

func TestNewProvider(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	p, _ := NewProvider(".")
	if _, ok := p.(*LocalFSProvider); !ok {
		t.Fatalf("Expected LocalFSProvider")
	}

	os.Setenv("OHC_STANDALONE", "false")
	p, _ = NewProvider(".")
	if _, ok := p.(*CloudFSProvider); !ok {
		t.Fatalf("Expected CloudFSProvider")
	}
}
