package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestLocalProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "local_provider_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create local provider: %v", err)
	}

	if !provider.IsLocal() {
		t.Errorf("expected IsLocal to be true")
	}

	ctx := context.Background()

	// Test write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Errorf("failed to write file: %v", err)
	}

	// Test read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("failed to read file: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("expected 'hello local', got '%s'", string(data))
	}

	// Test list
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected 1 entry 'test.txt', got %v", entries)
	}

	// Test search
	err = provider.WriteFile(ctx, "subdir/findme.txt", []byte("find me"))
	if err != nil {
		t.Errorf("failed to write file: %v", err)
	}
	searchRes, err := provider.SearchFiles(ctx, "findme", ".")
	if err != nil {
		t.Errorf("failed to search files: %v", err)
	}
	if len(searchRes) != 1 || searchRes[0] != filepath.Join("subdir", "findme.txt") {
		t.Errorf("expected search result ['subdir/findme.txt'], got %v", searchRes)
	}

	// Test path escape
	err = provider.WriteFile(ctx, "../escape.txt", []byte("escape"))
	if err == nil {
		t.Errorf("expected error when path escapes base dir")
	}
}

func TestCloudProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloud_provider_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create cloud provider: %v", err)
	}

	if provider.IsLocal() {
		t.Errorf("expected IsLocal to be false")
	}

	ctx := context.WithValue(context.Background(), tenantIDKey{}, "tenant1")

	// Test write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Errorf("failed to write file: %v", err)
	}

	// Verify it wrote to the correct tenant folder
	data, err := os.ReadFile(filepath.Join(tmpDir, "tenant1", "test.txt"))
	if err != nil {
		t.Errorf("failed to read actual file: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got '%s'", string(data))
	}

	// Test read
	readData, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("failed to read file: %v", err)
	}
	if string(readData) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got '%s'", string(readData))
	}

	// Test list
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected 1 entry 'test.txt', got %v", entries)
	}

	// Test search
	err = provider.WriteFile(ctx, "searchdir/cloudfindme.txt", []byte("find me cloud"))
	if err != nil {
		t.Errorf("failed to write file: %v", err)
	}
	searchRes, err := provider.SearchFiles(ctx, "cloudfindme", ".")
	if err != nil {
		t.Errorf("failed to search files: %v", err)
	}
	if len(searchRes) != 1 || searchRes[0] != filepath.Join("searchdir", "cloudfindme.txt") {
		t.Errorf("expected search result ['searchdir/cloudfindme.txt'], got %v", searchRes)
	}

	// Test cross tenant access attempt (should fail or write to correct tenant based on context)
	ctx2 := context.WithValue(context.Background(), tenantIDKey{}, "tenant2")
	_, err = provider.ReadFile(ctx2, "test.txt")
	if err == nil {
		t.Errorf("expected error reading file of another tenant")
	}
}

func TestServer(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "server_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create local provider: %v", err)
	}

	server := NewServer(provider)

	tools := server.ListTools()
	if len(tools) != 4 {
		t.Errorf("expected 4 tools, got %d", len(tools))
	}

	claims := &Claims{OrganizationID: "org1"}
	ctx := context.Background()

	// Test write tool
	res, err := server.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	}, claims)
	if err != nil {
		t.Errorf("CallTool write_file failed: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok || resMap["status"] != "success" {
		t.Errorf("expected success status, got %v", res)
	}

	// Test read tool
	res, err = server.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "hello.txt",
	}, claims)
	if err != nil {
		t.Errorf("CallTool read_file failed: %v", err)
	}

	resMap, ok = res.(map[string]interface{})
	if !ok || resMap["content"] != "world" {
		t.Errorf("expected content 'world', got %v", res)
	}

	// Test list tool
	res, err = server.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	}, claims)
	if err != nil {
		t.Errorf("CallTool list_directory failed: %v", err)
	}

	resList, ok := res.([]string)
	if !ok || len(resList) != 1 || resList[0] != "hello.txt" {
		t.Errorf("expected ['hello.txt'], got %v", res)
	}

	// Test search tool
	res, err = server.CallTool(ctx, "search_files", map[string]interface{}{
		"path":  ".",
		"query": "hello",
	}, claims)
	if err != nil {
		t.Errorf("CallTool search_files failed: %v", err)
	}
	resList, ok = res.([]string)
	if !ok || len(resList) != 1 || resList[0] != "hello.txt" {
		t.Errorf("expected search result ['hello.txt'], got %v", res)
	}

	// Test unknown tool
	_, err = server.CallTool(ctx, "unknown_tool", map[string]interface{}{}, claims)
	if err == nil {
		t.Errorf("expected error for unknown tool")
	}

	// Test missing claims
	_, err = server.CallTool(ctx, "read_file", map[string]interface{}{"path": "hello.txt"}, nil)
	if err == nil {
		t.Errorf("expected error for missing claims")
	}
}

func TestCloudProviderContext(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloud_server_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewCloudFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create cloud provider: %v", err)
	}

	server := NewServer(provider)
	claims := &Claims{OrganizationID: "org1"}
	ctx := context.Background()

	// Write file via tool
	_, err = server.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "hello.txt",
		"content": "cloud world",
	}, claims)
	if err != nil {
		t.Errorf("CallTool write_file failed: %v", err)
	}

	// Check it was created in the correct tenant dir
	data, err := os.ReadFile(filepath.Join(tmpDir, "org1", "hello.txt"))
	if err != nil {
		t.Errorf("failed to read actual file: %v", err)
	}
	if string(data) != "cloud world" {
		t.Errorf("expected 'cloud world', got '%s'", string(data))
	}
}

func TestFactory(t *testing.T) {
	// Test default Local provider
	os.Setenv("OHC_STANDALONE", "true")
	os.Setenv("OHC_MULTITENANT", "false")
	provider, err := NewProvider()
	if err != nil {
		t.Fatalf("failed to create default local provider: %v", err)
	}
	if !provider.IsLocal() {
		t.Errorf("expected IsLocal to be true")
	}

	// Test default Cloud provider
	os.Setenv("OHC_STANDALONE", "false")
	os.Setenv("OHC_MULTITENANT", "true")
	tmpDirCloud, _ := os.MkdirTemp("", "factory_cloud_test")
	defer os.RemoveAll(tmpDirCloud)
	os.Setenv("OHC_CLOUD_FS_MOUNT", tmpDirCloud)
	provider, err = NewProvider()
	if err != nil {
		t.Fatalf("failed to create default cloud provider: %v", err)
	}
	if provider.IsLocal() {
		t.Errorf("expected IsLocal to be false")
	}
}

func TestLocalProviderErrors(t *testing.T) {
	tmpDir, _ := os.MkdirTemp("", "local_provider_err_test")
	defer os.RemoveAll(tmpDir)

	provider, _ := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Read non-existent
	_, err := provider.ReadFile(ctx, "nonexistent.txt")
	if err == nil {
		t.Errorf("expected error for nonexistent file")
	}

	// List non-existent
	_, err = provider.ListDir(ctx, "nonexistent_dir")
	if err == nil {
		t.Errorf("expected error for nonexistent dir")
	}

	// Search non-existent
	_, err = provider.SearchFiles(ctx, "find", "nonexistent_dir")
	if err == nil {
		t.Errorf("expected error for nonexistent dir")
	}

	// Read escaped path
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Errorf("expected error for escaped path")
	}

	// List escaped path
	_, err = provider.ListDir(ctx, "../escape_dir")
	if err == nil {
		t.Errorf("expected error for escaped path")
	}

	// Escaping sibling folder check
	// Suppose base is /tmp/dir1. Try accessing /tmp/dir12 (starts with same prefix)
	siblingProvider, _ := NewLocalFSProvider(filepath.Join(tmpDir, "tenant1"))
	os.MkdirAll(filepath.Join(tmpDir, "tenant12"), 0755)

	_, err = siblingProvider.ReadFile(ctx, "../tenant12/file.txt")
	if err == nil {
		t.Errorf("expected error for accessing sibling folder")
	}
}

func TestCloudProviderErrors(t *testing.T) {
	tmpDir, _ := os.MkdirTemp("", "cloud_provider_err_test")
	defer os.RemoveAll(tmpDir)

	provider, _ := NewCloudFSProvider(tmpDir)
	ctx := context.WithValue(context.Background(), tenantIDKey{}, "tenant1")

	// Read non-existent
	_, err := provider.ReadFile(ctx, "nonexistent.txt")
	if err == nil {
		t.Errorf("expected error for nonexistent file")
	}

	// List non-existent
	_, err = provider.ListDir(ctx, "nonexistent_dir")
	if err == nil {
		t.Errorf("expected error for nonexistent dir")
	}

	// Read escaped path
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Errorf("expected error for escaped path")
	}

	// List escaped path
	_, err = provider.ListDir(ctx, "../escape_dir")
	if err == nil {
		t.Errorf("expected error for escaped path")
	}

	// Empty tenant ID in context
	emptyCtx := context.Background()
	_, err = provider.ReadFile(emptyCtx, "test.txt")
	if err == nil {
		t.Errorf("expected error for empty tenant ID")
	}

	err = provider.WriteFile(emptyCtx, "test.txt", []byte("data"))
	if err == nil {
		t.Errorf("expected error for empty tenant ID")
	}

	_, err = provider.ListDir(emptyCtx, ".")
	if err == nil {
		t.Errorf("expected error for empty tenant ID")
	}

	_, err = provider.SearchFiles(emptyCtx, "find", ".")
	if err == nil {
		t.Errorf("expected error for empty tenant ID")
	}
}

func TestServerErrors(t *testing.T) {
	tmpDir, _ := os.MkdirTemp("", "server_err_test")
	defer os.RemoveAll(tmpDir)

	provider, _ := NewLocalFSProvider(tmpDir)
	server := NewServer(provider)
	claims := &Claims{OrganizationID: "org1"}
	ctx := context.Background()

	// Missing provider
	nilServer := NewServer(nil)
	_, err := nilServer.CallTool(ctx, "read_file", map[string]interface{}{"path": "test.txt"}, claims)
	if err == nil {
		t.Errorf("expected error for nil provider")
	}

	// Read file - missing path
	_, err = server.CallTool(ctx, "read_file", map[string]interface{}{}, claims)
	if err == nil {
		t.Errorf("expected error for missing path")
	}

	// Write file - missing path
	_, err = server.CallTool(ctx, "write_file", map[string]interface{}{"content": "test"}, claims)
	if err == nil {
		t.Errorf("expected error for missing path")
	}

	// Write file - missing content
	_, err = server.CallTool(ctx, "write_file", map[string]interface{}{"path": "test.txt"}, claims)
	if err == nil {
		t.Errorf("expected error for missing content")
	}

	// List directory - missing path
	_, err = server.CallTool(ctx, "list_directory", map[string]interface{}{}, claims)
	if err == nil {
		t.Errorf("expected error for missing path")
	}

	// Search files - missing path
	_, err = server.CallTool(ctx, "search_files", map[string]interface{}{"query": "hello"}, claims)
	if err == nil {
		t.Errorf("expected error for missing path")
	}

	// Search files - missing query
	_, err = server.CallTool(ctx, "search_files", map[string]interface{}{"path": "."}, claims)
	if err == nil {
		t.Errorf("expected error for missing query")
	}

	// Read file - provider error
	_, err = server.CallTool(ctx, "read_file", map[string]interface{}{"path": "nonexistent.txt"}, claims)
	if err == nil {
		t.Errorf("expected provider error to propagate")
	}

	// List directory - provider error
	_, err = server.CallTool(ctx, "list_directory", map[string]interface{}{"path": "nonexistent_dir"}, claims)
	if err == nil {
		t.Errorf("expected provider error to propagate")
	}

	// Search files - provider error
	_, err = server.CallTool(ctx, "search_files", map[string]interface{}{"path": "nonexistent_dir", "query": "a"}, claims)
	if err == nil {
		t.Errorf("expected provider error to propagate")
	}
}
