package hybridfsmcp

import (
	"context"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	dir := t.TempDir()
	provider := NewLocalFSProvider(dir)
	ctx := context.Background()

	err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil || string(data) != "hello" {
		t.Fatalf("ReadFile failed or wrong data: %v, %s", err, string(data))
	}

	infos, err := provider.ListDir(ctx, nil, ".")
	if err != nil || len(infos) != 1 {
		t.Fatalf("ListDir failed or wrong length: %v, len %d", err, len(infos))
	}

	results, err := provider.SearchFiles(ctx, nil, "test")
	if err != nil || len(results) != 1 {
		t.Fatalf("SearchFiles failed or wrong length: %v, len %d", err, len(results))
	}

	_, err = provider.ReadFile(ctx, nil, "../outside.txt")
	if err != ErrAccessDenied {
		t.Fatalf("Expected access denied for path traversal, got %v", err)
	}
}

func TestCloudFSProvider(t *testing.T) {
	dir := t.TempDir()
	provider := NewCloudFSProvider(dir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}

	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil || string(data) != "hello" {
		t.Fatalf("ReadFile failed or wrong data: %v, %s", err, string(data))
	}

	_, err = provider.ReadFile(ctx, nil, "test.txt")
	if err != ErrAccessDenied {
		t.Fatalf("Expected access denied for nil claims, got %v", err)
	}

	claims2 := &auth.Claims{OrganizationID: "tenant-10"}
	_, err = provider.ReadFile(ctx, claims2, "../tenant-1/test.txt")
	if err != ErrAccessDenied {
		t.Fatalf("Expected access denied for cross tenant access, got %v", err)
	}
}
