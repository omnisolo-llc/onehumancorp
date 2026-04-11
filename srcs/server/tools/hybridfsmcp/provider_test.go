package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test WriteFile
	testPath := "test_dir/test_file.txt"
	testContent := []byte("hello local")
	err = provider.WriteFile(ctx, testPath, testContent)
	if err != nil {
		t.Errorf("expected no error writing file, got: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Errorf("expected no error reading file, got: %v", err)
	}
	if string(content) != string(testContent) {
		t.Errorf("expected %q, got %q", testContent, content)
	}

	// Test Path Bounds
	err = provider.WriteFile(ctx, "../outside.txt", testContent)
	if err == nil {
		t.Error("expected error for path outside bounds, got nil")
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, "test_dir")
	if err != nil {
		t.Errorf("expected no error listing dir, got: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test_file.txt" {
		t.Errorf("unexpected directory entries: %v", entries)
	}

	// Test SearchFiles
	matches, err := provider.SearchFiles(ctx, ".", "test")
	if err != nil {
		t.Errorf("expected no error searching files, got: %v", err)
	}
	if len(matches) != 1 || filepath.ToSlash(matches[0]) != "test_dir/test_file.txt" {
		t.Errorf("unexpected search matches: %v", matches)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{OrganizationID: "tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test without claims
	emptyCtx := context.Background()
	err = provider.WriteFile(emptyCtx, "test.txt", []byte("fail"))
	if err == nil {
		t.Error("expected error for missing claims, got nil")
	}

	// Test WriteFile
	testPath := "test_dir/test_file.txt"
	testContent := []byte("hello cloud")
	err = provider.WriteFile(ctx, testPath, testContent)
	if err != nil {
		t.Errorf("expected no error writing file, got: %v", err)
	}

	// Verify it wrote to the tenant directory
	tenantPath := filepath.Join(tempDir, "tenant-123", filepath.FromSlash(testPath))
	_, err = os.Stat(tenantPath)
	if os.IsNotExist(err) {
		t.Errorf("expected file at %s, but it was not created", tenantPath)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Errorf("expected no error reading file, got: %v", err)
	}
	if string(content) != string(testContent) {
		t.Errorf("expected %q, got %q", testContent, content)
	}

	// Test Path Bounds
	err = provider.WriteFile(ctx, "../outside.txt", testContent)
	if err == nil {
		t.Error("expected error for path outside tenant bounds, got nil")
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, "test_dir")
	if err != nil {
		t.Errorf("expected no error listing dir, got: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test_file.txt" {
		t.Errorf("unexpected directory entries: %v", entries)
	}

	// Test SearchFiles
	matches, err := provider.SearchFiles(ctx, ".", "test")
	if err != nil {
		t.Errorf("expected no error searching files, got: %v", err)
	}
	if len(matches) != 1 || filepath.ToSlash(matches[0]) != "test_dir/test_file.txt" {
		t.Errorf("unexpected search matches: %v", matches)
	}
}

func TestNewProvider(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	p1 := NewProvider(context.Background(), "/tmp")
	if _, ok := p1.(*CloudFSProvider); !ok {
		t.Error("expected CloudFSProvider")
	}

	os.Setenv("OHC_MULTITENANT", "false")
	p2 := NewProvider(context.Background(), "/tmp")
	if _, ok := p2.(*LocalFSProvider); !ok {
		t.Error("expected LocalFSProvider")
	}

	os.Unsetenv("OHC_MULTITENANT")
}

func TestHybridFSServer(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "server_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	server := NewHybridFSServer(provider)
	ctx := context.Background()

	// Write
	res := server.WriteFileTool(ctx, "test.txt", "content")
	if res.Status != "success" {
		t.Errorf("expected success, got %s: %s", res.Status, res.ResultData)
	}

	// Read
	res = server.ReadFileTool(ctx, "test.txt")
	if res.Status != "success" {
		t.Errorf("expected success, got %s: %s", res.Status, res.ResultData)
	}

	// List
	res = server.ListDirectoryTool(ctx, ".")
	if res.Status != "success" {
		t.Errorf("expected success, got %s: %s", res.Status, res.ResultData)
	}

	// Search
	res = server.SearchFilesTool(ctx, ".", "test")
	if res.Status != "success" {
		t.Errorf("expected success, got %s: %s", res.Status, res.ResultData)
	}
}
