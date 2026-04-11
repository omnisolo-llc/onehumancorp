package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"reflect"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	if !provider.IsLocal() {
		t.Error("Expected local provider")
	}

	ctx := context.Background()

	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatal(err)
	}

	content, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(content) != "hello world" {
		t.Errorf("Expected 'hello world', got %s", content)
	}

	files, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatal(err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", files)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)
	if provider.IsLocal() {
		t.Error("Expected cloud provider")
	}

	ctx := context.Background()

	err = provider.WriteFile(ctx, "tenant1/test.txt", []byte("cloud hello"))
	if err != nil {
		t.Fatal(err)
	}

	content, err := provider.ReadFile(ctx, "tenant1/test.txt")
	if err != nil {
		t.Fatal(err)
	}
	if string(content) != "cloud hello" {
		t.Errorf("Expected 'cloud hello', got %s", content)
	}

	files, err := provider.ListDir(ctx, "tenant1")
	if err != nil {
		t.Fatal(err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", files)
	}
}

func TestHybridFSProxy_Local(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "proxy_local_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	proxy := NewHybridFSProxy(provider)
	ctx := context.Background()

	tools := proxy.ListTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	_, err = proxy.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"content": "hello",
	})
	if err != nil {
		t.Fatal(err)
	}

	res, err := proxy.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatal(err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatal("Expected map response")
	}
	if resMap["content"] != "hello" {
		t.Errorf("Expected 'hello', got %v", resMap["content"])
	}

	listRes, err := proxy.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatal(err)
	}

	listResMap := listRes.(map[string]interface{})
	files := listResMap["files"].([]string)
	if !reflect.DeepEqual(files, []string{"test.txt"}) {
		t.Errorf("Expected ['test.txt'], got %v", files)
	}
}

func TestHybridFSProxy_Cloud(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "proxy_cloud_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)
	proxy := NewHybridFSProxy(provider)

	// Should fail without claims
	ctx := context.Background()
	_, err = proxy.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"content": "hello",
	})
	if err == nil {
		t.Fatal("Expected error without claims")
	}

	// Should succeed with claims
	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	_, err = proxy.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"content": "hello cloud",
	})
	if err != nil {
		t.Fatal(err)
	}

	// Verify it was written to the tenant subfolder
	content, err := os.ReadFile(filepath.Join(tmpDir, "tenant1/test.txt"))
	if err != nil {
		t.Fatal(err)
	}
	if string(content) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got %s", content)
	}
}

func TestNewProviderFromEnv(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	p1 := NewProviderFromEnv("/tmp")
	if p1.IsLocal() {
		t.Error("Expected cloud provider")
	}

	os.Setenv("OHC_MULTITENANT", "false")
	p2 := NewProviderFromEnv("/tmp")
	if !p2.IsLocal() {
		t.Error("Expected local provider")
	}
}
