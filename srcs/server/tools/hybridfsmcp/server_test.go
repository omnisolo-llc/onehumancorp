package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
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
	ctx := context.Background()

	// Write file
	data := []byte("hello local")
	err = provider.WriteFile(ctx, "test.txt", data, 0644)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read file
	readData, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readData) != "hello local" {
		t.Fatalf("Expected 'hello local', got '%s'", string(readData))
	}

	// List dir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Fatalf("Expected 1 file named 'test.txt', got %v", infos)
	}

	// Access denied test
	err = provider.WriteFile(ctx, "../escape.txt", data, 0644)
	if err != ErrAccessDenied {
		t.Fatalf("Expected ErrAccessDenied, got %v", err)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	// Create a context with claims
	claims := &auth.Claims{OrganizationID: "tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write file
	data := []byte("hello cloud")
	err = provider.WriteFile(ctx, "data.txt", data, 0644)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify tenant isolation (it should be in volumeRoot/tenant-123/data.txt)
	tenantPath := filepath.Join(tmpDir, "tenant-123", "data.txt")
	if _, err := os.Stat(tenantPath); os.IsNotExist(err) {
		t.Fatalf("File was not created in tenant-scoped directory")
	}

	// Read file
	readData, err := provider.ReadFile(ctx, "data.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readData) != "hello cloud" {
		t.Fatalf("Expected 'hello cloud', got '%s'", string(readData))
	}

	// List dir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "data.txt" {
		t.Fatalf("Expected 1 file named 'data.txt', got %v", infos)
	}

	// Test with no claims
	err = provider.WriteFile(context.Background(), "noauth.txt", data, 0644)
	if err == nil {
		t.Fatalf("Expected error when writing without claims")
	}

	// Access denied test
	err = provider.WriteFile(ctx, "../escape.txt", data, 0644)
	if err != ErrAccessDenied {
		t.Fatalf("Expected ErrAccessDenied, got %v", err)
	}
}

func TestFileSystemMCPServer(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcpserver_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	server := NewFileSystemMCPServer(provider)
	ctx := context.Background()

	// Write File via MCP
	writePayload := `{"path":"mcp.txt","data":"aGVsbG8="}` // "hello" base64
	res, err := server.ExecuteTool(ctx, "write_file", []byte(writePayload))
	if err != nil {
		t.Fatalf("ExecuteTool write_file error: %v", err)
	}
	if res.Status != "success" {
		t.Fatalf("Expected success, got %s: %s", res.Status, string(res.ResultData))
	}

	// Read File via MCP
	readPayload := `{"path":"mcp.txt"}`
	res, err = server.ExecuteTool(ctx, "read_file", []byte(readPayload))
	if err != nil {
		t.Fatalf("ExecuteTool read_file error: %v", err)
	}
	if res.Status != "success" {
		t.Fatalf("Expected success, got %s", res.Status)
	}
	// Note: in MCP server we just return raw bytes currently, if you sent base64 you read base64 back
	if string(res.ResultData) != "hello" {
		t.Fatalf("Expected 'hello', got %s", string(res.ResultData))
	}

	// List Dir via MCP
	listPayload := `{"path":"."}`
	res, err = server.ExecuteTool(ctx, "list_directory", []byte(listPayload))
	if err != nil {
		t.Fatalf("ExecuteTool list_directory error: %v", err)
	}
	if res.Status != "success" {
		t.Fatalf("Expected success, got %s", res.Status)
	}

	var entries []struct{ Name string `json:"name"` }
	json.Unmarshal(res.ResultData, &entries)
	if len(entries) != 1 || entries[0].Name != "mcp.txt" {
		t.Fatalf("Unexpected list_directory output: %s", string(res.ResultData))
	}

	// Unimplemented search
	res, err = server.ExecuteTool(ctx, "search_files", []byte(`{}`))
	if err != nil {
		t.Fatalf("ExecuteTool search_files error: %v", err)
	}
	if res.Status != "error" {
		t.Fatalf("Expected error, got %s", res.Status)
	}
}

func TestNewProvider(t *testing.T) {
	// Test Cloud
	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("OHC_CLOUD_VOLUME_ROOT", "/tmp/cloud")
	p, err := NewProvider(context.Background())
	if err != nil {
		t.Fatal(err)
	}
	if _, ok := p.(*CloudFSProvider); !ok {
		t.Fatalf("Expected CloudFSProvider, got %T", p)
	}

	// Test Local
	os.Setenv("OHC_MULTITENANT", "")
	os.Setenv("OHC_LOCAL_WORKSPACE", "/tmp/local")
	p, err = NewProvider(context.Background())
	if err != nil {
		t.Fatal(err)
	}
	if _, ok := p.(*LocalFSProvider); !ok {
		t.Fatalf("Expected LocalFSProvider, got %T", p)
	}
}

func TestLocalFSProvider_PrefixBypass(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test_prefix")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	// Setup directories
	baseDir := filepath.Join(tmpDir, "tenant-1")
	otherDir := filepath.Join(tmpDir, "tenant-10")
	if err := os.MkdirAll(baseDir, 0755); err != nil {
		t.Fatal(err)
	}
	if err := os.MkdirAll(otherDir, 0755); err != nil {
		t.Fatal(err)
	}

	// Write file to other dir
	data := []byte("secret")
	if err := os.WriteFile(filepath.Join(otherDir, "secret.txt"), data, 0644); err != nil {
		t.Fatal(err)
	}

	provider := NewLocalFSProvider(baseDir)
	ctx := context.Background()

	// Try to access other dir
	// baseDir is .../tenant-1
	// if we ask for ../tenant-10/secret.txt
	// cleanTarget is .../tenant-10/secret.txt
	_, err = provider.ReadFile(ctx, "../tenant-10/secret.txt")
	if err != ErrAccessDenied {
		t.Fatalf("Expected ErrAccessDenied, got %v", err)
	}
}

func TestCloudFSProvider_PrefixBypass(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test_prefix")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	// Create a context with claims for tenant-1
	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Setup other dir
	otherDir := filepath.Join(tmpDir, "tenant-10")
	if err := os.MkdirAll(otherDir, 0755); err != nil {
		t.Fatal(err)
	}

	// Write file to other dir
	data := []byte("secret")
	if err := os.WriteFile(filepath.Join(otherDir, "secret.txt"), data, 0644); err != nil {
		t.Fatal(err)
	}

	// Try to access other dir
	_, err = provider.ReadFile(ctx, "../tenant-10/secret.txt")
	if err != ErrAccessDenied {
		t.Fatalf("Expected ErrAccessDenied, got %v", err)
	}
}
