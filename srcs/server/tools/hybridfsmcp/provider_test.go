package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	dir, err := os.MkdirTemp("", "localfstmp")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)

	provider, err := NewLocalFSProvider(dir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	ctx := context.Background()

	// WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Unexpected ListDir result: %v", entries)
	}

	// SearchFiles
	matches, err := provider.SearchFiles(ctx, "*.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("Unexpected SearchFiles result: %v", matches)
	}

	// Boundary check
	err = provider.WriteFile(ctx, "../outside.txt", []byte("bad"))
	if err != ErrAccessDenied {
		t.Errorf("Expected ErrAccessDenied, got %v", err)
	}

	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err != ErrAccessDenied {
		t.Errorf("Expected ErrAccessDenied, got %v", err)
	}

	_, err = provider.ListDir(ctx, "../")
	if err != ErrAccessDenied {
		t.Errorf("Expected ErrAccessDenied, got %v", err)
	}
}

func TestCloudFSProvider(t *testing.T) {
	dir, err := os.MkdirTemp("", "cloudfstmp")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(dir)

	provider, err := NewCloudFSProvider(dir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	ctx := context.Background()
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-1",
	})

	// Missing claims error
	err = provider.WriteFile(ctx, "test.txt", []byte("bad"))
	if err != ErrUnauthorized {
		t.Errorf("Expected ErrUnauthorized, got %v", err)
	}
	_, err = provider.SearchFiles(ctx, "*")
	if err != ErrUnauthorized {
		t.Errorf("Expected ErrUnauthorized, got %v", err)
	}

	// WriteFile
	err = provider.WriteFile(ctxWithClaims, "folder/test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Ensure file is isolated physically at org-1/folder/test.txt
	b, err := os.ReadFile(filepath.Join(dir, "org-1/folder/test.txt"))
	if err != nil {
		t.Fatalf("Failed physical verify: %v", err)
	}
	if string(b) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(b))
	}

	// ReadFile
	data, err := provider.ReadFile(ctxWithClaims, "folder/test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// ListDir
	entries, err := provider.ListDir(ctxWithClaims, "folder")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Unexpected ListDir result: %v", entries)
	}

	// SearchFiles
	matches, err := provider.SearchFiles(ctxWithClaims, "*.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || matches[0] != filepath.Join("folder", "test.txt") {
		t.Errorf("Unexpected SearchFiles result: %v", matches)
	}

	// SearchFiles on missing org dir
	ctxOrg2 := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "org-2",
	})
	matches2, err := provider.SearchFiles(ctxOrg2, "*")
	if err != nil {
		t.Fatalf("SearchFiles empty failed: %v", err)
	}
	if len(matches2) != 0 {
		t.Errorf("Expected empty matches, got %v", matches2)
	}
}
