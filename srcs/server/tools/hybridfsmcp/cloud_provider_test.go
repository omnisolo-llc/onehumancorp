package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "org-123"}
	otherClaims := &auth.Claims{OrganizationID: "org-456"}

	t.Run("Write and Read File Tenant Scoped", func(t *testing.T) {
		err := provider.WriteFile(ctx, claims, "data.txt", []byte("tenant data"))
		if err != nil {
			t.Fatalf("WriteFile failed: %v", err)
		}

		content, err := provider.ReadFile(ctx, claims, "data.txt")
		if err != nil {
			t.Fatalf("ReadFile failed: %v", err)
		}
		if string(content) != "tenant data" {
			t.Errorf("Expected 'tenant data', got '%s'", string(content))
		}

		// Other tenant should not read it
		_, err = provider.ReadFile(ctx, otherClaims, "data.txt")
		if err == nil {
			t.Error("Expected error when other tenant reads file, got nil")
		}
	})

	t.Run("Path Traversal Blocked", func(t *testing.T) {
		err := provider.WriteFile(ctx, claims, "../org-456/data.txt", []byte("hack"))
		if err == nil {
			t.Error("Expected error for cross-tenant path traversal, got nil")
		}
	})

	t.Run("List Dir", func(t *testing.T) {
		provider.WriteFile(ctx, claims, "docs/doc1.txt", []byte("1"))
		entries, err := provider.ListDir(ctx, claims, "docs")
		if err != nil {
			t.Fatalf("ListDir failed: %v", err)
		}
		if len(entries) != 1 {
			t.Errorf("Expected 1 entry, got %d", len(entries))
		}
	})
}
