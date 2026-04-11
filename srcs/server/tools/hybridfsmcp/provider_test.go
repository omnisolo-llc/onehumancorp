package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "test-org"}

	// Test WriteFile
	err := provider.WriteFile(ctx, claims, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("expected no error writing file, got %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("expected no error reading file, got %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("expected 'hello local', got '%s'", string(data))
	}

	// Test path traversal
	err = provider.WriteFile(ctx, claims, "../escaped.txt", []byte("hack"))
	if err == nil {
		t.Error("expected error on path traversal, got nil")
	}
	_, err = provider.ReadFile(ctx, claims, "../escaped.txt")
	if err == nil {
		t.Error("expected error on path traversal read, got nil")
	}

	// Test ListDir
	err = provider.WriteFile(ctx, claims, "subdir/file1.txt", []byte("1"))
	if err != nil {
		t.Fatalf("expected no error writing file in subdir, got %v", err)
	}

	entries, err := provider.ListDir(ctx, claims, "")
	if err != nil {
		t.Fatalf("expected no error listing dir, got %v", err)
	}
	if len(entries) != 2 {
		t.Errorf("expected 2 entries, got %d", len(entries))
	}

	entriesSub, err := provider.ListDir(ctx, claims, "subdir")
	if err != nil {
		t.Fatalf("expected no error listing subdir, got %v", err)
	}
	if len(entriesSub) != 1 || entriesSub[0] != "file1.txt" {
		t.Errorf("expected [file1.txt], got %v", entriesSub)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{OrganizationID: "tenant-a"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err := provider.WriteFile(ctx, claims, "data.json", []byte(`{"key":"value"}`))
	if err != nil {
		t.Fatalf("expected no error writing file, got %v", err)
	}

	// Verify it was written to the tenant subfolder
	expectedPath := filepath.Join(tempDir, "tenant-a", "data.json")
	if _, err := os.Stat(expectedPath); os.IsNotExist(err) {
		t.Errorf("expected file to exist at %s, but it did not", expectedPath)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "data.json")
	if err != nil {
		t.Fatalf("expected no error reading file, got %v", err)
	}
	if string(data) != `{"key":"value"}` {
		t.Errorf("expected '{\"key\":\"value\"}', got '%s'", string(data))
	}

	// Test unauthorized (missing claims)
	err = provider.WriteFile(context.Background(), nil, "test.txt", []byte("bad"))
	if err == nil {
		t.Error("expected error for missing claims, got nil")
	}

	// Test path traversal
	err = provider.WriteFile(ctx, claims, "../tenant-b/hack.txt", []byte("hack"))
	if err == nil {
		t.Error("expected error on path traversal, got nil")
	}
	_, err = provider.ReadFile(ctx, claims, "../tenant-b/hack.txt")
	if err == nil {
		t.Error("expected error on path traversal read, got nil")
	}

	// Test ListDir
	err = provider.WriteFile(ctx, claims, "folder/item.txt", []byte("item"))
	if err != nil {
		t.Fatalf("expected no error writing file, got %v", err)
	}

	entries, err := provider.ListDir(ctx, claims, "")
	if err != nil {
		t.Fatalf("expected no error listing dir, got %v", err)
	}

	// Should see "data.json" and "folder"
	foundData := false
	foundFolder := false
	for _, e := range entries {
		if e == "data.json" {
			foundData = true
		}
		if e == "folder" {
			foundFolder = true
		}
	}
	if !foundData || !foundFolder {
		t.Errorf("expected to find data.json and folder, got %v", entries)
	}

	// Test ListDir on non-existent tenant dir
	claimsB := &auth.Claims{OrganizationID: "tenant-b"}
	ctxB := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claimsB)
	entriesB, err := provider.ListDir(ctxB, claimsB, "")
	if err != nil {
		t.Fatalf("expected no error listing non-existent tenant dir, got %v", err)
	}
	if len(entriesB) != 0 {
		t.Errorf("expected 0 entries, got %d", len(entriesB))
	}
}
