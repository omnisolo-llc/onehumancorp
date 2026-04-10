package hybridfsmcp

import (
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := &LocalFSProvider{WorkspaceRoot: tmpDir}

	// Test WriteFile
	testPath := "test.txt"
	testContent := []byte("hello local")
	err = provider.WriteFile(testPath, testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	readContent, err := provider.ReadFile(testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readContent) != string(testContent) {
		t.Errorf("expected %s, got %s", string(testContent), string(readContent))
	}

	// Test ListDir
	entries, err := provider.ListDir(".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != testPath {
		t.Errorf("expected [%s], got %v", testPath, entries)
	}

	// Test Path Traversal
	_, err = provider.ReadFile("../outside.txt")
	if err == nil {
		t.Error("expected error for path traversal, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	claims := &auth.Claims{OrganizationID: "tenant-123"}
	provider := &CloudFSProvider{MountRoot: tmpDir, Claims: claims}

	// Test WriteFile
	testPath := "test.txt"
	testContent := []byte("hello cloud")
	err = provider.WriteFile(testPath, testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify Tenant Scoping (file should be in tmpDir/tenant-123/test.txt)
	tenantPath := filepath.Join(tmpDir, "tenant-123", testPath)
	if _, err := os.Stat(tenantPath); os.IsNotExist(err) {
		t.Errorf("file not written to correct tenant directory: %s", tenantPath)
	}

	// Test ReadFile
	readContent, err := provider.ReadFile(testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readContent) != string(testContent) {
		t.Errorf("expected %s, got %s", string(testContent), string(readContent))
	}

	// Test Path Traversal
	_, err = provider.ReadFile("../outside.txt")
	if err == nil || !strings.Contains(err.Error(), "traversal") {
		t.Errorf("expected traversal error, got %v", err)
	}

	// Test Missing Tenant
	providerNoAuth := &CloudFSProvider{MountRoot: tmpDir, Claims: nil}
	_, err = providerNoAuth.ReadFile(testPath)
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Errorf("expected unauthorized error, got %v", err)
	}
}

func TestProviderFactory(t *testing.T) {
	claims := &auth.Claims{OrganizationID: "tenant-456"}

	// Test Cloud Mode
	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("OHC_CLOUD_FS_MOUNT", "/tmp/cloud")
	cloudProv := ProviderFactory(claims, ".")
	if _, ok := cloudProv.(*CloudFSProvider); !ok {
		t.Error("expected CloudFSProvider")
	}
	os.Unsetenv("OHC_MULTITENANT")
	os.Unsetenv("OHC_CLOUD_FS_MOUNT")

	// Test Local Mode
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_LOCAL_WORKSPACE", "/tmp/local")
	localProv := ProviderFactory(claims, ".")
	if _, ok := localProv.(*LocalFSProvider); !ok {
		t.Error("expected LocalFSProvider")
	}
	os.Unsetenv("OHC_STANDALONE")
	os.Unsetenv("OHC_LOCAL_WORKSPACE")
}

func TestHybridFSMCP(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := &LocalFSProvider{WorkspaceRoot: tmpDir}
	mcp := NewHybridFSMCP(provider)

	// ListTools
	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("expected 4 tools, got %d", len(tools))
	}

	// WriteFile Tool
	writeArgs := map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "hello mcp",
	}
	_, err = mcp.CallTool(nil, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("write_file tool failed: %v", err)
	}

	// ReadFile Tool
	readArgs := map[string]interface{}{
		"path": "mcp_test.txt",
	}
	res, err := mcp.CallTool(nil, "read_file", readArgs)
	if err != nil {
		t.Fatalf("read_file tool failed: %v", err)
	}
	readRes, ok := res.(map[string]interface{})
	if !ok || readRes["content"] != "hello mcp" {
		t.Errorf("read_file expected 'hello mcp', got %v", res)
	}

	// ListDir Tool
	listArgs := map[string]interface{}{
		"path": ".",
	}
	res, err = mcp.CallTool(nil, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("list_directory tool failed: %v", err)
	}
	listRes, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("unexpected list_directory result format")
	}
	entries, ok := listRes["entries"].([]string)
	if !ok || len(entries) != 1 || entries[0] != "mcp_test.txt" {
		t.Errorf("list_directory expected [mcp_test.txt], got %v", listRes["entries"])
	}

	// SearchFiles Tool
	_, err = mcp.CallTool(nil, "search_files", map[string]interface{}{"query": "test"})
	if err == nil {
		t.Errorf("expected search_files to fail (not implemented)")
	}

	// Unknown Tool
	_, err = mcp.CallTool(nil, "unknown_tool", nil)
	if err == nil {
		t.Errorf("expected unknown_tool to fail")
	}
}
