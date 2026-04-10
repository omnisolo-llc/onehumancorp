package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Test WriteFile
	testContent := []byte("hello local")
	err = provider.WriteFile(ctx, nil, "test.txt", testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, nil, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// Test ListDir
	info, err := provider.ListDir(ctx, nil, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(info) != 1 || info[0].Name != "test.txt" {
		t.Errorf("ListDir returned unexpected result: %+v", info)
	}

	// Test SearchFiles
	results, err := provider.SearchFiles(ctx, nil, ".", "test.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(results) != 1 || results[0] != "test.txt" {
		t.Errorf("SearchFiles returned unexpected result: %+v", results)
	}

	// Test Traversal
	_, err = provider.ReadFile(ctx, nil, "../outside.txt")
	if err == nil {
		t.Errorf("Expected path traversal error, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "org123"}

	// Test missing claims
	_, err = provider.ReadFile(ctx, nil, "test.txt")
	if err == nil {
		t.Errorf("Expected error for missing claims, got nil")
	}

	// Test WriteFile
	testContent := []byte("hello cloud")
	err = provider.WriteFile(ctx, claims, "test.txt", testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Verify tenant isolation
	claims2 := &auth.Claims{OrganizationID: "org456"}
	_, err = provider.ReadFile(ctx, claims2, "test.txt")
	if err == nil {
		t.Errorf("Expected error when reading other tenant's file")
	}

	// Test ListDir
	info, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(info) != 1 || info[0].Name != "test.txt" {
		t.Errorf("ListDir returned unexpected result: %+v", info)
	}

	// Test SearchFiles
	results, err := provider.SearchFiles(ctx, claims, ".", "test.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(results) != 1 || results[0] != "test.txt" {
		t.Errorf("SearchFiles returned unexpected result: %+v", results)
	}
}

func TestFactory(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	provider := NewFileSystemProvider("/local", "/cloud")
	_, ok := provider.(*CloudFSProvider)
	if !ok {
		t.Errorf("Expected CloudFSProvider when OHC_MULTITENANT=true")
	}

	os.Unsetenv("OHC_MULTITENANT")
	provider = NewFileSystemProvider("/local", "/cloud")
	_, ok = provider.(*LocalFSProvider)
	if !ok {
		t.Errorf("Expected LocalFSProvider when OHC_MULTITENANT is not set")
	}
}

func TestHybridFSMCPServer(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	server := NewHybridFSMCPServer(provider)
	ctx := context.Background()

	// Test WriteFile tool
	writeInput := []byte(`{"path":"test.txt","data":"hello mcp"}`)
	_, err = server.CallTool(ctx, nil, "write_file", writeInput)
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// Test ReadFile tool
	readInput := []byte(`{"path":"test.txt"}`)
	res, err := server.CallTool(ctx, nil, "read_file", readInput)
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if res.(string) != "hello mcp" {
		t.Errorf("Expected 'hello mcp', got '%v'", res)
	}

	// Test ListDir tool
	listInput := []byte(`{"path":"."}`)
	listRes, err := server.CallTool(ctx, nil, "list_directory", listInput)
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	if len(listRes.([]FileInfo)) != 1 {
		t.Errorf("Expected 1 file in list_directory, got %d", len(listRes.([]FileInfo)))
	}

	// Test SearchFiles tool
	searchInput := []byte(`{"path":".", "pattern":"test.txt"}`)
	searchRes, err := server.CallTool(ctx, nil, "search_files", searchInput)
	if err != nil {
		t.Fatalf("CallTool search_files failed: %v", err)
	}
	if len(searchRes.([]string)) != 1 {
		t.Errorf("Expected 1 file in search_files, got %d", len(searchRes.([]string)))
	}

	// Test Unknown tool
	_, err = server.CallTool(ctx, nil, "unknown", readInput)
	if err == nil {
		t.Errorf("Expected error for unknown tool")
	}
}
