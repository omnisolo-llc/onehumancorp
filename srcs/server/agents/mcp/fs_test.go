package mcp

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create local fs provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "hello world" {
		t.Errorf("expected 'hello world', got %s", content)
	}

	// Test path escaping
	_, err = provider.ReadFile(ctx, "../../../../etc/passwd")
	if err == nil {
		t.Errorf("expected error when path escapes workspace")
	}

	// Test ListDir
	err = provider.WriteFile(ctx, "dir/file1.txt", []byte("file1"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}
	files, err := provider.ListDir(ctx, "dir")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 {
		t.Errorf("expected 1 file, got %d", len(files))
	}
	if files[0].Name != "file1.txt" {
		t.Errorf("expected 'file1.txt', got %s", files[0].Name)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create cloud fs provider: %v", err)
	}

	ctxWithClaims := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-1",
	})
	ctxWithoutClaims := context.Background()

	// Test unauthorized
	err = provider.WriteFile(ctxWithoutClaims, "test.txt", []byte("hello"))
	if err == nil {
		t.Errorf("expected error without claims")
	}

	// Test WriteFile
	err = provider.WriteFile(ctxWithClaims, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it wrote to the tenant directory
	content, err := os.ReadFile(filepath.Join(tempDir, "tenant-1", "test.txt"))
	if err != nil {
		t.Fatalf("failed to read written file directly: %v", err)
	}
	if string(content) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %s", content)
	}

	// Test ReadFile
	readContent, err := provider.ReadFile(ctxWithClaims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readContent) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %s", readContent)
	}

	// Test ListDir
	files, err := provider.ListDir(ctxWithClaims, "")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0].Name != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", files)
	}

	// Test path escaping
	_, err = provider.ReadFile(ctxWithClaims, "../../../../etc/passwd")
	if err == nil {
		t.Errorf("expected error when path escapes tenant workspace")
	}
}

func TestFSFactory(t *testing.T) {
	// Test Cloud
	os.Setenv("OHC_STANDALONE", "false")
	provider, err := NewFileSystemProvider(".")
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider, got %T", provider)
	}

	// Test Local
	os.Setenv("OHC_STANDALONE", "true")
	provider, err = NewFileSystemProvider(".")
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider, got %T", provider)
	}
}

func TestFSMCPTools(t *testing.T) {
	tempDir, _ := os.MkdirTemp("", "fs-tools")
	defer os.RemoveAll(tempDir)

	provider, _ := NewLocalFSProvider(tempDir)
	tools := NewFSMCPTools(provider)
	ctx := context.Background()

	// Write
	writeArgs := []byte(`{"path":"test.txt","content":"tool content"}`)
	res := tools.WriteFile(ctx, writeArgs)
	if res.Status != "success" {
		t.Errorf("expected success, got %s: %s", res.Status, res.ResultData)
	}

	// Read
	readArgs := []byte(`{"path":"test.txt"}`)
	res = tools.ReadFile(ctx, readArgs)
	if res.Status != "success" {
		t.Errorf("expected success, got %s: %s", res.Status, res.ResultData)
	}
	var readResult map[string]interface{}
	json.Unmarshal(res.ResultData, &readResult)
	if readResult["content"] != "tool content" {
		t.Errorf("expected 'tool content', got %v", readResult["content"])
	}

	// List
	listArgs := []byte(`{"path":""}`)
	res = tools.ListDirectory(ctx, listArgs)
	if res.Status != "success" {
		t.Errorf("expected success, got %s: %s", res.Status, res.ResultData)
	}
}

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_traversal")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create local fs provider: %v", err)
	}

	ctx := context.Background()

	// Test HasPrefix bypass: e.g., if base is /tmp/tenant-1, try /tmp/tenant-10
	siblingDir := tempDir + "0"
	err = os.MkdirAll(siblingDir, 0755)
	if err == nil {
		defer os.RemoveAll(siblingDir)
		err = os.WriteFile(filepath.Join(siblingDir, "secret.txt"), []byte("secret"), 0644)
		if err == nil {
			_, err = provider.ReadFile(ctx, "../" + filepath.Base(siblingDir) + "/secret.txt")
			if err == nil {
				t.Errorf("expected error when path escapes workspace using sibling directory")
			}
		}
	}
}

func TestCloudFSProvider_PathTraversal(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_traversal")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create cloud fs provider: %v", err)
	}

	ctxWithClaims := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-1",
	})

	// Create base tenant dir
	os.MkdirAll(filepath.Join(tempDir, "tenant-1"), 0755)

	// Test HasPrefix bypass
	siblingDir := filepath.Join(tempDir, "tenant-10")
	err = os.MkdirAll(siblingDir, 0755)
	if err == nil {
		defer os.RemoveAll(siblingDir)
		err = os.WriteFile(filepath.Join(siblingDir, "secret.txt"), []byte("secret"), 0644)
		if err == nil {
			_, err = provider.ReadFile(ctxWithClaims, "../tenant-10/secret.txt")
			if err == nil {
				t.Errorf("expected error when path escapes tenant workspace using sibling directory")
			}
		}
	}
}

func TestRouter_ExecuteTool(t *testing.T) {
	tempDir, _ := os.MkdirTemp("", "router")
	defer os.RemoveAll(tempDir)

	provider, _ := NewLocalFSProvider(tempDir)
	router := NewRouter(provider)
	ctx := context.Background()

	// Test unknown tool
	res := router.ExecuteTool(ctx, "unknown_tool", []byte(`{}`))
	if res.Status != "error" {
		t.Errorf("expected error for unknown tool")
	}

	// Test valid tool
	writeArgs := []byte(`{"path":"test.txt","content":"hello router"}`)
	res = router.ExecuteTool(ctx, "write_file", writeArgs)
	if res.Status != "success" {
		t.Errorf("expected success for write_file")
	}
}
