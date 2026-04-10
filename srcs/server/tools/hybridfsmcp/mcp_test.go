package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	if !provider.IsLocal() {
		t.Errorf("expected IsLocal to be true")
	}

	ctx := context.Background()

	// Test WriteFile
	testData := []byte("test content")
	err = provider.WriteFile(ctx, "test.txt", testData)
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	// Test ReadFile
	readData, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("failed to read file: %v", err)
	}
	if string(readData) != string(testData) {
		t.Errorf("expected %s, got %s", string(testData), string(readData))
	}

	// Test Path Traversal
	err = provider.WriteFile(ctx, "../escape.txt", testData)
	if err == nil || !strings.Contains(err.Error(), "escapes workspace boundary") {
		t.Errorf("expected access denied error, got: %v", err)
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("failed to list dir: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("expected 1 file 'test.txt', got %d files", len(infos))
	}

	// Test SearchFiles
	files, err := provider.SearchFiles(ctx, ".", "*.txt")
	if err != nil {
		t.Fatalf("failed to search files: %v", err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("expected 1 file 'test.txt', got %v", files)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)
	if provider.IsLocal() {
		t.Errorf("expected IsLocal to be false")
	}

	claims := &auth.Claims{OrganizationID: "tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test unauthorized access
	noClaimsCtx := context.Background()
	_, err = provider.ReadFile(noClaimsCtx, "test.txt")
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Errorf("expected unauthorized error for missing claims")
	}

	// Test WriteFile
	testData := []byte("cloud content")
	err = provider.WriteFile(ctx, "test.txt", testData)
	if err != nil {
		t.Fatalf("failed to write file: %v", err)
	}

	// Check that it was actually written to tenant dir
	tenantDir := filepath.Join(tempDir, "tenant-123")
	tenantFile := filepath.Join(tenantDir, "test.txt")
	if _, err := os.Stat(tenantFile); os.IsNotExist(err) {
		t.Errorf("file was not written to tenant directory")
	}

	// Test Path Traversal
	err = provider.WriteFile(ctx, "../escape.txt", testData)
	if err == nil || !strings.Contains(err.Error(), "escapes tenant boundary") {
		t.Errorf("expected access denied error, got: %v", err)
	}

	// Ensure ListDir works
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("failed to list dir: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("expected 1 file 'test.txt', got %d files", len(infos))
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "hybridfsmcp_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	mcp := NewHybridFSMCP(provider)

	tools := mcp.ListTools()
	if len(tools) != 4 {
		t.Errorf("expected 4 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// Test call_tool write_file
	b64Data := base64.StdEncoding.EncodeToString([]byte("hello mcp"))
	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "hello.txt",
		"data": b64Data,
	})
	if err != nil {
		t.Fatalf("failed to call write_file tool: %v", err)
	}

	resMap := res.(map[string]interface{})
	if resMap["status"] != "success" || resMap["mode"] != "standalone" {
		t.Errorf("unexpected write result: %v", resMap)
	}

	// Test call_tool read_file
	res, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "hello.txt",
	})
	if err != nil {
		t.Fatalf("failed to call read_file tool: %v", err)
	}

	resMap = res.(map[string]interface{})
	content, _ := base64.StdEncoding.DecodeString(resMap["content"].(string))
	if string(content) != "hello mcp" {
		t.Errorf("unexpected read result content: %s", string(content))
	}

	// Test call_tool list_directory
	res, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("failed to call list_directory tool: %v", err)
	}

	resMap = res.(map[string]interface{})
	results := resMap["results"].([]map[string]interface{})
	if len(results) != 1 || results[0]["name"] != "hello.txt" {
		t.Errorf("unexpected list_directory result: %v", results)
	}

	// Test call_tool search_files
	res, err = mcp.CallTool(ctx, "search_files", map[string]interface{}{
		"directory": ".",
		"pattern": "*.txt",
	})
	if err != nil {
		t.Fatalf("failed to call search_files tool: %v", err)
	}

	resMap = res.(map[string]interface{})
	files := resMap["results"].([]string)
	if len(files) != 1 || files[0] != "hello.txt" {
		t.Errorf("unexpected search_files result: %v", files)
	}
}

func TestFactory(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	p := NewFileSystemProvider(".")
	if !p.IsLocal() {
		t.Errorf("expected local provider when OHC_STANDALONE is true")
	}

	os.Setenv("OHC_STANDALONE", "false")
	p = NewFileSystemProvider(".")
	if p.IsLocal() {
		t.Errorf("expected cloud provider when OHC_STANDALONE is false")
	}
}
