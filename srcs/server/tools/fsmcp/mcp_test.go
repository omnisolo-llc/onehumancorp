package fsmcp

import (
	"context"


	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test write and read
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(data) != "hello" {
		t.Fatalf("expected 'hello', got '%s'", string(data))
	}

	// Test list dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Fatalf("expected ['test.txt'], got %v", entries)
	}

	// Test directory traversal prevention
	_, err = provider.ReadFile(ctx, "../../../etc/passwd")
	if err == nil {
		t.Fatalf("expected error on traversal, got nil")
	}
}

func TestFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewFSMCP(provider)
	ctx := context.Background()

	// List tools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	// Write file
	_, err := mcp.CallTool(ctx, "fs_write", map[string]interface{}{
		"path":    "mcp.txt",
		"content": "mcp content",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Read file
	res, err := mcp.CallTool(ctx, "fs_read", map[string]interface{}{
		"path": "mcp.txt",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["content"] != "mcp content" {
		t.Fatalf("expected 'mcp content', got '%v'", resMap["content"])
	}

	// List dir
	listRes, err := mcp.CallTool(ctx, "fs_list", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	listResMap := listRes.(map[string]interface{})
	entries := listResMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "mcp.txt" {
		t.Fatalf("expected ['mcp.txt'], got %v", entries)
	}
}

func TestNewProviderFactory_Local(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
    t.Setenv("OHC_FS_BASE_DIR", t.TempDir())
	provider, err := NewProviderFactory()
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Fatalf("expected LocalFSProvider")
	}
}

func TestNewProviderFactory_Cloud_MissingEnv(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "false")
	_, err := NewProviderFactory()
    if err == nil {
        t.Fatalf("expected error due to missing S3 env vars")
    }
}

func TestLocalFSProvider_Errors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test read non-existent file
	_, err := provider.ReadFile(ctx, "nonexistent.txt")
	if err == nil {
		t.Fatalf("expected error reading nonexistent file")
	}

	// Test write directory traversal
	err = provider.WriteFile(ctx, "../../../etc/passwd", []byte("hacked"))
	if err == nil {
		t.Fatalf("expected error writing with traversal")
	}

	// Test list dir traversal
	_, err = provider.ListDir(ctx, "../../../etc")
	if err == nil {
		t.Fatalf("expected error listing with traversal")
	}

	// Test list non-existent dir
	_, err = provider.ListDir(ctx, "nonexistent")
	if err == nil {
		t.Fatalf("expected error listing nonexistent dir")
	}
}

func TestFSMCP_Errors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewFSMCP(provider)
	ctx := context.Background()

	// Test unknown tool
	_, err := mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for unknown tool")
	}

	// Test fs_read missing path
	_, err = mcp.CallTool(ctx, "fs_read", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for missing path in fs_read")
	}

	// Test fs_read provider error
	_, err = mcp.CallTool(ctx, "fs_read", map[string]interface{}{
		"path": "nonexistent.txt",
	})
	if err == nil {
		t.Fatalf("expected error for provider error in fs_read")
	}

	// Test fs_write missing path
	_, err = mcp.CallTool(ctx, "fs_write", map[string]interface{}{
		"content": "test",
	})
	if err == nil {
		t.Fatalf("expected error for missing path in fs_write")
	}

	// Test fs_write missing content
	_, err = mcp.CallTool(ctx, "fs_write", map[string]interface{}{
		"path": "test.txt",
	})
	if err == nil {
		t.Fatalf("expected error for missing content in fs_write")
	}

	// Test fs_write traversal error
	_, err = mcp.CallTool(ctx, "fs_write", map[string]interface{}{
		"path":    "../test.txt",
		"content": "test",
	})
	if err == nil {
		t.Fatalf("expected error for provider error in fs_write")
	}

	// Test fs_list missing path
	_, err = mcp.CallTool(ctx, "fs_list", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for missing path in fs_list")
	}

	// Test fs_list provider error
	_, err = mcp.CallTool(ctx, "fs_list", map[string]interface{}{
		"path": "nonexistent_dir",
	})
	if err == nil {
		t.Fatalf("expected error for provider error in fs_list")
	}
}

func TestCloudFSProvider_GetTenantPrefix(t *testing.T) {
    provider := &CloudFSProvider{}

    // Test with claims
	claims := &auth.Claims{
		OrganizationID: "tenant1",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

    prefix, err := provider.getTenantPrefix(ctx)
    if err != nil {
        t.Fatalf("expected no error, got %v", err)
    }
    if prefix != "tenant/tenant1/fs/" {
        t.Fatalf("expected tenant/tenant1/fs/, got %s", prefix)
    }

    // Test without claims
    _, err = provider.getTenantPrefix(context.Background())
    if err == nil {
        t.Fatalf("expected error without claims")
    }
}
