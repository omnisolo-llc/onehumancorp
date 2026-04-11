package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestCloudFSProvider_Coverage(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	// Missing Claims
	ctx := context.Background()
	_, err := provider.ReadFile(ctx, "test.txt")
	if err == nil {
		t.Fatal("expected error")
	}

	claims := &auth.Claims{OrganizationID: "org1"}
	ctx = context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Path Traversal
	_, err = provider.ReadFile(ctx, "../../../test.txt")
	if err == nil {
		t.Fatal("expected error")
	}

	// IsLocal
	if provider.IsLocal() {
		t.Fatal("expected IsLocal to be false")
	}

	// ListDir non-existent
	_, err = provider.ListDir(ctx, "not_exist")
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestLocalFSProvider_Coverage(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)

	ctx := context.Background()

	// IsLocal
	if !provider.IsLocal() {
		t.Fatal("expected IsLocal to be true")
	}

	// Path traversal on WriteFile
	err := provider.WriteFile(ctx, "../../../test.txt", []byte("a"))
	if err == nil {
		t.Fatal("expected error")
	}

	// Path traversal on ListDir
	_, err = provider.ListDir(ctx, "../../../test.txt")
	if err == nil {
		t.Fatal("expected error")
	}

	// ListDir non-existent
	_, err = provider.ListDir(ctx, "not_exist")
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestMCP_Coverage(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewFSInspectorMCP(provider)

	ctx := context.Background()

	// ListTools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatal("expected 3 tools")
	}

	// Missing paths
	_, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Fatal("expected error")
	}

	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{})
	if err == nil {
		t.Fatal("expected error")
	}

	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt"})
	if err == nil {
		t.Fatal("expected error")
	}

	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{})
	if err == nil {
		t.Fatal("expected error")
	}

	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Fatal("expected error")
	}

	// Invalid Base64
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "content_b64": "!!!"})
	if err == nil {
		t.Fatal("expected error")
	}
}

func TestFactory(t *testing.T) {
	os.Setenv("OHC_FS_ROOT", "")
	os.Setenv("OHC_MULTITENANT", "false")
	p1 := NewProvider()
	if !p1.IsLocal() {
		t.Fatal("expected local")
	}

	os.Setenv("OHC_FS_ROOT", "/tmp")
	os.Setenv("OHC_MULTITENANT", "true")
	p2 := NewProvider()
	if p2.IsLocal() {
		t.Fatal("expected cloud")
	}
}
