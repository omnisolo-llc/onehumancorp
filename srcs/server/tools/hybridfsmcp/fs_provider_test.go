package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "local_fs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()
	testPath := "test.txt"
	testData := []byte("hello local")

	err = provider.WriteFile(ctx, testPath, testData)
	if err != nil {
		t.Errorf("expected no error writing file, got %v", err)
	}

	data, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Errorf("expected no error reading file, got %v", err)
	}
	if string(data) != string(testData) {
		t.Errorf("expected data %s, got %s", testData, data)
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("expected no error listing dir, got %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("unexpected dir entries: %v", entries)
	}

	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("expected path traversal error")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloud_fs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()
	testPath := "test.txt"
	testData := []byte("hello cloud")

	err = provider.WriteFile(ctx, testPath, testData)
	if err == nil {
		t.Errorf("expected unauthorized error")
	}

	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	err = provider.WriteFile(ctx, testPath, testData)
	if err != nil {
		t.Errorf("expected no error writing file, got %v", err)
	}

	data, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Errorf("expected no error reading file, got %v", err)
	}
	if string(data) != string(testData) {
		t.Errorf("expected data %s, got %s", testData, data)
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("expected no error listing dir, got %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("unexpected dir entries: %v", entries)
	}

	_, err = provider.ReadFile(ctx, "../tenant2/test.txt")
	if err == nil {
		t.Errorf("expected path traversal error")
	}
}
