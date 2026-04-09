package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	workspaceDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(workspaceDir)

	provider := NewLocalFSProvider(workspaceDir)
	ctx := context.Background()

	// Test WriteFile
	testPath := "test_dir/test_file.txt"
	testData := []byte("hello local fs")
	err = provider.WriteFile(ctx, testPath, testData)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local fs" {
		t.Fatalf("Expected 'hello local fs', got '%s'", string(data))
	}

	// Test ListDir
	info, err := provider.ListDir(ctx, "test_dir")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(info) != 1 {
		t.Fatalf("Expected 1 file, got %d", len(info))
	}
	if info[0].Name() != "test_file.txt" {
		t.Fatalf("Expected test_file.txt, got %s", info[0].Name())
	}

	// Test path escape
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Fatalf("Expected path escape to fail")
	}

	// Test partial path match vulnerability
	testPrefixDir := workspaceDir + "_extra"
	err = os.MkdirAll(testPrefixDir, 0755)
	if err != nil {
		t.Fatalf("Failed to create prefix dir: %v", err)
	}
	defer os.RemoveAll(testPrefixDir)
	err = os.WriteFile(filepath.Join(testPrefixDir, "test.txt"), []byte("secret"), 0644)
	if err != nil {
		t.Fatalf("Failed to write test file: %v", err)
	}

	_, err = provider.ReadFile(ctx, "../localfs_test_extra/test.txt")
	if err == nil {
		t.Fatalf("Expected partial path match escape to fail")
	}
}

func TestCloudFSProvider(t *testing.T) {
	baseDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(baseDir)

	provider := NewCloudFSProvider(baseDir)
	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	testPath := "test_dir/test_file.txt"
	testData := []byte("hello cloud fs")
	err = provider.WriteFile(ctx, testPath, testData)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud fs" {
		t.Fatalf("Expected 'hello cloud fs', got '%s'", string(data))
	}

	// Test ListDir
	info, err := provider.ListDir(ctx, "test_dir")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(info) != 1 {
		t.Fatalf("Expected 1 file, got %d", len(info))
	}
	if info[0].Name() != "test_file.txt" {
		t.Fatalf("Expected test_file.txt, got %s", info[0].Name())
	}

	// Test Unauthorized (No Claims)
	unauthCtx := context.Background()
	err = provider.WriteFile(unauthCtx, testPath, testData)
	if err == nil {
		t.Fatalf("Expected WriteFile to fail without claims")
	}

	// Test path escape
	_, err = provider.ReadFile(ctx, "../tenant2/test_file.txt")
	if err == nil {
		t.Fatalf("Expected path escape to fail")
	}
}

func TestNewProvider(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_MULTITENANT", "false")
	p1 := NewProvider()
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Fatalf("Expected LocalFSProvider in standalone mode")
	}

	os.Setenv("OHC_STANDALONE", "false")
	os.Setenv("OHC_MULTITENANT", "true")
	p2 := NewProvider()
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Fatalf("Expected CloudFSProvider in cloud mode")
	}
}
