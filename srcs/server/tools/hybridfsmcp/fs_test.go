package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewFileSystemProvider("OHC_STANDALONE", tmpDir)

	ctx := context.Background()

	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got %q", string(data))
	}

	_, err = provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Escaping workspace
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Error("expected error for escaping workspace")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewFileSystemProvider("OHC_MULTITENANT", tmpDir)

	ctx := context.Background()

	// Unauth context
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err == nil {
		t.Error("expected error for unauthorized context")
	}

	// Auth context
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got %q", string(data))
	}

	_, err = provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	// Escaping tenant dir
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Error("expected error for escaping tenant dir")
	}
}
