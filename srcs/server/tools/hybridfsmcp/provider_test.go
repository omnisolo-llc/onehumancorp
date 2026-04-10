package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathValidation(t *testing.T) {
	baseDir, err := os.MkdirTemp("", "localfs_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(baseDir)

	provider := NewLocalFSProvider(baseDir)

	tests := []struct {
		name      string
		reqPath   string
		expectErr bool
	}{
		{"Valid path", "file.txt", false},
		{"Valid subdirectory path", "sub/file.txt", false},
		{"Path traversal blocked", "../file.txt", true},
		{"Path traversal with absolute path blocked", "/etc/passwd", true},
		{"Path traversal edge case blocked", baseDir + "suffix/file.txt", true}, // Prevent overlapping prefix
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := provider.validatePath(tt.reqPath)
			if (err != nil) != tt.expectErr {
				t.Errorf("validatePath(%q) error = %v, expectErr %v", tt.reqPath, err, tt.expectErr)
			}
		})
	}
}

func TestLocalFSProvider_Operations(t *testing.T) {
	baseDir, err := os.MkdirTemp("", "localfs_test_ops_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(baseDir)

	provider := NewLocalFSProvider(baseDir)
	ctx := context.Background()

	// Test WriteFile
	res, err := provider.WriteFile(ctx, "test.txt", "hello world", nil)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}
	if res["status"] != "success" {
		t.Errorf("WriteFile unexpected status: %v", res)
	}

	// Test ReadFile
	res, err = provider.ReadFile(ctx, "test.txt", nil)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if res["content"] != "hello world" {
		t.Errorf("ReadFile unexpected content: %v", res)
	}

	// Test ListDir
	res, err = provider.ListDir(ctx, ".", nil)
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	files := res["files"].([]map[string]interface{})
	if len(files) != 1 || files[0]["name"] != "test.txt" {
		t.Errorf("ListDir unexpected files: %v", res)
	}
}

func TestCloudFSProvider_PathValidation(t *testing.T) {
	cloudRoot, err := os.MkdirTemp("", "cloudfs_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(cloudRoot)

	provider := NewCloudFSProvider(cloudRoot)
	claims := &auth.Claims{OrganizationID: "tenant-1"}

	tests := []struct {
		name      string
		reqPath   string
		claims    *auth.Claims
		expectErr bool
	}{
		{"Valid path", "file.txt", claims, false},
		{"Path traversal blocked", "../file.txt", claims, true},
		{"Path traversal to other tenant blocked", "../../tenant-2/file.txt", claims, true},
		{"Missing claims", "file.txt", nil, true},
		{"Missing org ID", "file.txt", &auth.Claims{}, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := provider.validatePath(tt.reqPath, tt.claims)
			if (err != nil) != tt.expectErr {
				t.Errorf("validatePath(%q) error = %v, expectErr %v", tt.reqPath, err, tt.expectErr)
			}
		})
	}
}

func TestCloudFSProvider_Operations(t *testing.T) {
	cloudRoot, err := os.MkdirTemp("", "cloudfs_test_ops_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(cloudRoot)

	provider := NewCloudFSProvider(cloudRoot)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-1"}

	// Test WriteFile
	res, err := provider.WriteFile(ctx, "test.txt", "hello tenant", claims)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}
	if res["status"] != "success" {
		t.Errorf("WriteFile unexpected status: %v", res)
	}

	// Verify file is actually in the tenant's subdirectory
	content, err := os.ReadFile(filepath.Join(cloudRoot, "tenant-1", "test.txt"))
	if err != nil || string(content) != "hello tenant" {
		t.Errorf("File not written to correct tenant path: %v", err)
	}

	// Test ReadFile
	res, err = provider.ReadFile(ctx, "test.txt", claims)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if res["content"] != "hello tenant" {
		t.Errorf("ReadFile unexpected content: %v", res)
	}

	// Test ListDir
	res, err = provider.ListDir(ctx, ".", claims)
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	files := res["files"].([]map[string]interface{})
	if len(files) != 1 || files[0]["name"] != "test.txt" {
		t.Errorf("ListDir unexpected files: %v", res)
	}
}
