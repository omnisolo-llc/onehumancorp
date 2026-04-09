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
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)

	ctx := context.Background()

	// Test WriteFile
	err := provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("Unexpected ListDir result")
	}

	// Test bounds checking
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("Expected bounds check error for outside path")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewCloudFSProvider(tmpDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Ensure tenant base dir exists
	tenantDir := filepath.Join(tmpDir, "tenant-123")
	os.MkdirAll(tenantDir, 0755)

	// Test WriteFile
	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("Unexpected ListDir result")
	}

	// Test bounds checking
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("Expected bounds check error for outside path")
	}

	// Test missing claims
	ctxNoClaims := context.Background()
	_, err = provider.ReadFile(ctxNoClaims, "test.txt")
	if err == nil {
		t.Errorf("Expected error for missing claims")
	}

    // Test invalid tenant ID
    invalidClaims := &auth.Claims{
		OrganizationID: "../tenant",
	}
	ctxInvalidClaims := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, invalidClaims)
    _, err = provider.ReadFile(ctxInvalidClaims, "test.txt")
	if err == nil {
		t.Errorf("Expected error for invalid tenant ID")
	}
}

func TestNewProviderFactory(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	provider := NewProviderFactory("/tmp")
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider in standalone mode")
	}

	os.Setenv("OHC_STANDALONE", "false")
	provider = NewProviderFactory("/tmp")
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider in cloud mode")
	}
}

func TestServer(t *testing.T) {
	tmpDir := t.TempDir()
	provider := NewLocalFSProvider(tmpDir)
	server := NewServer(provider)

	ctx := context.Background()

	// Test write_file tool
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "test.txt", Content: "test content"})
	res := server.ExecuteTool(ctx, "write_file", writeArgs)
	if res.Status != "success" {
		t.Fatalf("write_file tool failed: %s", string(res.ResultData))
	}

	// Test read_file tool
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "test.txt"})
	res = server.ExecuteTool(ctx, "read_file", readArgs)
	if res.Status != "success" {
		t.Fatalf("read_file tool failed: %s", string(res.ResultData))
	}
	var readRes map[string]string
	json.Unmarshal(res.ResultData, &readRes)
	if readRes["content"] != "test content" {
		t.Errorf("Expected 'test content', got '%s'", readRes["content"])
	}

	// Test list_directory tool
	listArgs, _ := json.Marshal(ListDirArgs{Path: "."})
	res = server.ExecuteTool(ctx, "list_directory", listArgs)
	if res.Status != "success" {
		t.Fatalf("list_directory tool failed: %s", string(res.ResultData))
	}
	var listRes []ListDirResult
	json.Unmarshal(res.ResultData, &listRes)
	if len(listRes) != 1 || listRes[0].Name != "test.txt" {
		t.Errorf("Unexpected list_directory result")
	}

	// Test unknown tool
	res = server.ExecuteTool(ctx, "unknown_tool", nil)
	if res.Status != "error" {
		t.Errorf("Expected error for unknown tool")
	}
}
