package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestCloudProviderInvalidPath(t *testing.T) {
	provider := NewCloudFSProvider()

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant1",
	})

	// test invalid path containing ..
	_, err := provider.ReadFile(ctx, "../secret.txt")
	if err == nil {
		t.Fatal("expected error for invalid path")
	}

	// test file not found
	_, err = provider.ReadFile(ctx, "nonexistent.txt")
	if err == nil {
		t.Fatal("expected error for file not found")
	}
}
