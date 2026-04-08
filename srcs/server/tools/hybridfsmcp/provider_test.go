package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "local_fs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "any_tenant", "test.txt", []byte("hello local"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "any_tenant", "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("ReadFile returned unexpected data: %s", data)
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, "any_tenant", ".")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("ListDir returned unexpected files: %v", files)
	}

	// Test SearchFiles
	matches, err := provider.SearchFiles(ctx, "any_tenant", ".", "*.txt")
	if err != nil {
		t.Errorf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("SearchFiles returned unexpected matches: %v", matches)
	}

	// Test Traversal
	_, err = provider.ReadFile(ctx, "any_tenant", "../outside.txt")
	if err == nil {
		t.Errorf("expected error for directory traversal, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloud_fs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)
	ctx := context.Background()
	tenantID := "tenant-1"

	// Create tenant dir
	os.MkdirAll(filepath.Join(tmpDir, tenantID), 0755)

	// Test Invalid Tenant
	err = provider.WriteFile(ctx, "invalid tenant!", "test.txt", []byte("hello"))
	if err == nil {
		t.Errorf("expected error for invalid tenant ID, got nil")
	}

	// Test WriteFile
	err = provider.WriteFile(ctx, tenantID, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, tenantID, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("ReadFile returned unexpected data: %s", data)
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, tenantID, ".")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("ListDir returned unexpected files: %v", files)
	}

	// Test SearchFiles
	matches, err := provider.SearchFiles(ctx, tenantID, ".", "*.txt")
	if err != nil {
		t.Errorf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Errorf("SearchFiles returned unexpected matches: %v", matches)
	}

	// Test Traversal
	_, err = provider.ReadFile(ctx, tenantID, "../outside.txt")
	if err == nil {
		t.Errorf("expected error for directory traversal, got nil")
	}
}

func TestFactory(t *testing.T) {
	local := NewFileSystemProvider(true, "/tmp")
	if _, ok := local.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider")
	}

	cloud := NewFileSystemProvider(false, "/tmp")
	if _, ok := cloud.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider")
	}
}

func TestLocalFSProvider_Errors(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "local_fs_err_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Test ReadFile not found
	_, err = provider.ReadFile(ctx, "any", "missing.txt")
	if err == nil {
		t.Errorf("expected error reading missing file")
	}

	// Test ListDir not found
	_, err = provider.ListDir(ctx, "any", "missing_dir")
	if err == nil {
		t.Errorf("expected error listing missing dir")
	}

	// Test SearchFiles not found path
	_, err = provider.SearchFiles(ctx, "any", "missing_dir", "*.txt")
	if err == nil {
		t.Errorf("expected error searching missing dir")
	}

	// Test traversal writes
	err = provider.WriteFile(ctx, "any", "../outside.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error writing outside base dir")
	}

	// Test traversal lists
	_, err = provider.ListDir(ctx, "any", "../")
	if err == nil {
		t.Errorf("expected error listing outside base dir")
	}

	// Test traversal searches
	_, err = provider.SearchFiles(ctx, "any", "../", "*.txt")
	if err == nil {
		t.Errorf("expected error searching outside base dir")
	}
}

func TestCloudFSProvider_Errors(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloud_fs_err_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)
	ctx := context.Background()
	tenantID := "tenant-2"

	os.MkdirAll(filepath.Join(tmpDir, tenantID), 0755)

	// Test ReadFile not found
	_, err = provider.ReadFile(ctx, tenantID, "missing.txt")
	if err == nil {
		t.Errorf("expected error reading missing file")
	}

	// Test ListDir not found
	_, err = provider.ListDir(ctx, tenantID, "missing_dir")
	if err == nil {
		t.Errorf("expected error listing missing dir")
	}

	// Test SearchFiles not found path
	_, err = provider.SearchFiles(ctx, tenantID, "missing_dir", "*.txt")
	if err == nil {
		t.Errorf("expected error searching missing dir")
	}

	// Test valid tenant regex but invalid paths
	err = provider.WriteFile(ctx, tenantID, "../outside.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error writing outside base dir")
	}

	_, err = provider.ListDir(ctx, tenantID, "../")
	if err == nil {
		t.Errorf("expected error listing outside base dir")
	}

	_, err = provider.SearchFiles(ctx, tenantID, "../", "*.txt")
	if err == nil {
		t.Errorf("expected error searching outside base dir")
	}

	// Test invalid tenants across operations
	_, err = provider.ReadFile(ctx, "bad tenant!", "test.txt")
	if err == nil {
		t.Errorf("expected error for invalid tenant")
	}

	_, err = provider.ListDir(ctx, "bad tenant!", ".")
	if err == nil {
		t.Errorf("expected error for invalid tenant")
	}

	_, err = provider.SearchFiles(ctx, "bad tenant!", ".", "*.txt")
	if err == nil {
		t.Errorf("expected error for invalid tenant")
	}
}
