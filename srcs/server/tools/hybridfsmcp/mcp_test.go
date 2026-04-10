package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_FS_ROOT", tmpDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := NewLocalFSProvider()

	// Valid path
	err = provider.WriteFile(context.Background(), "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("expected nil error, got %v", err)
	}

	content, err := provider.ReadFile(context.Background(), "test.txt")
	if err != nil {
		t.Errorf("expected nil error, got %v", err)
	}
	if string(content) != "hello" {
		t.Errorf("expected 'hello', got '%s'", string(content))
	}

	// Path traversal protection
	err = provider.WriteFile(context.Background(), "../outside.txt", []byte("bad"))
	if err == nil || !strings.Contains(err.Error(), "access denied") {
		t.Errorf("expected path traversal error, got %v", err)
	}
}

func TestCloudFSProvider_TenantScoping(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_FS_ROOT", tmpDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := NewCloudFSProvider()

	// No claims should fail
	ctx := context.Background()
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Errorf("expected unauthorized error, got %v", err)
	}

	// With valid claims
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Valid path for tenant
	err = provider.WriteFile(ctxWithClaims, "test.txt", []byte("tenant1 data"))
	if err != nil {
		t.Errorf("expected nil error, got %v", err)
	}

	// Check isolation
	tenant1File := filepath.Join(tmpDir, "tenant1", "test.txt")
	b, err := os.ReadFile(tenant1File)
	if err != nil {
		t.Fatalf("expected file to be created at %s, got err: %v", tenant1File, err)
	}
	if string(b) != "tenant1 data" {
		t.Errorf("unexpected content: %s", string(b))
	}

	// Path traversal protection
	err = provider.WriteFile(ctxWithClaims, "../tenant2/test.txt", []byte("hack"))
	if err == nil || !strings.Contains(err.Error(), "access denied") {
		t.Errorf("expected path traversal error, got %v", err)
	}
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "hybridmcp_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_FS_ROOT", tmpDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	// Test Local Mode
	os.Setenv("OHC_MULTITENANT", "false")
	localProvider := GetProvider()
	mcpLocal := NewHybridFSMCP(localProvider)

	if !localProvider.IsLocal() {
		t.Error("expected IsLocal to be true")
	}

	tools := mcpLocal.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}

	res, err := mcpLocal.CallTool(context.Background(), "write_file", map[string]interface{}{
		"path":    "local.txt",
		"content": "local content",
	})
	if err != nil {
		t.Errorf("CallTool error: %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["mode"] != "standalone" {
		t.Errorf("expected standalone mode, got %v", resMap["mode"])
	}

	// Test Cloud Mode
	os.Setenv("OHC_MULTITENANT", "true")
	cloudProvider := GetProvider()
	mcpCloud := NewHybridFSMCP(cloudProvider)

	if cloudProvider.IsLocal() {
		t.Error("expected IsLocal to be false")
	}

	// Missing claims
	_, err = mcpCloud.CallTool(context.Background(), "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Errorf("expected unauthorized error for missing claims, got %v", err)
	}

	// Valid claims
	claims := &auth.Claims{OrganizationID: "org-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write
	_, err = mcpCloud.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "cloud.txt",
		"content": "cloud content",
	})
	if err != nil {
		t.Errorf("CallTool write error: %v", err)
	}

	// Read
	resRead, err := mcpCloud.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "cloud.txt",
	})
	if err != nil {
		t.Errorf("CallTool read error: %v", err)
	}
	resReadMap := resRead.(map[string]interface{})
	if resReadMap["content"] != "cloud content" {
		t.Errorf("unexpected content: %v", resReadMap["content"])
	}

	// List
	resList, err := mcpCloud.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Errorf("CallTool list error: %v", err)
	}
	resListMap := resList.(map[string]interface{})
	results := resListMap["results"].([]map[string]interface{})
	if len(results) != 1 || results[0]["name"] != "cloud.txt" {
		t.Errorf("unexpected list result: %v", results)
	}
}
