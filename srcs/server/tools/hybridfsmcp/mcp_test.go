package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Write file
	err := provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(data) != "hello local" {
		t.Fatalf("expected 'hello local', got '%s'", string(data))
	}

	// List dir
	items, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(items) != 1 || items[0] != "test.txt" {
		t.Fatalf("expected ['test.txt'], got %v", items)
	}

	// Directory traversal
	_, err = provider.ReadFile(ctx, "../test.txt")
	if err == nil {
		t.Fatalf("expected directory traversal error")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Context without claims
	ctxNoClaims := context.Background()
	err := provider.WriteFile(ctxNoClaims, "test.txt", []byte("hello"))
	if err == nil {
		t.Fatalf("expected error for missing claims")
	}

	// Write file
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// Verify tenant isolation
	if _, err := os.Stat(filepath.Join(tempDir, "tenant-1", "test.txt")); os.IsNotExist(err) {
		t.Fatalf("file not created in tenant directory")
	}

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(data) != "hello cloud" {
		t.Fatalf("expected 'hello cloud', got '%s'", string(data))
	}

	// List dir
	items, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if len(items) != 1 || items[0] != "test.txt" {
		t.Fatalf("expected ['test.txt'], got %v", items)
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)
	ctx := context.Background()

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Fatalf("expected 3 tools, got %d", len(tools))
	}

	// Write
	writeArgs := map[string]interface{}{"path": "mcp.txt", "content": "mcp data"}
	res, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" {
		t.Fatalf("expected success")
	}

	// Read
	readArgs := map[string]interface{}{"path": "mcp.txt"}
	res, err = mcp.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap = res.(map[string]interface{})
	if resMap["content"] != "mcp data" {
		t.Fatalf("expected 'mcp data', got '%s'", resMap["content"])
	}

	// List
	listArgs := map[string]interface{}{"path": ""}
	res, err = mcp.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	resMap = res.(map[string]interface{})
	items := resMap["items"].([]string)
	if len(items) != 1 || items[0] != "mcp.txt" {
		t.Fatalf("expected ['mcp.txt'], got %v", items)
	}
}

func TestProviderFactory(t *testing.T) {
	t.Setenv("OHC_MULTITENANT", "true")


	p := ProviderFactory("/tmp/ws", "/tmp/cloud")
	if _, ok := p.(*CloudFSProvider); !ok {
		t.Fatalf("expected CloudFSProvider in multitenant mode")
	}

	t.Setenv("OHC_MULTITENANT", "false")
	p = ProviderFactory("/tmp/ws", "/tmp/cloud")
	if _, ok := p.(*LocalFSProvider); !ok {
		t.Fatalf("expected LocalFSProvider in standalone mode")
	}
}
