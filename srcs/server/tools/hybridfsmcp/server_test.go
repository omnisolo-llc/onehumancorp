package hybridfsmcp

import (
	"context"
	"os"
	"testing"
)

func TestServerTools(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "server_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	server := NewServer(provider)
	ctx := context.Background()

	// Test write_file
	writeRes, err := server.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "test.txt",
		"content": "hello world",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	if writeMap, ok := writeRes.(map[string]interface{}); !ok || writeMap["success"] != true {
		t.Errorf("write_file didn't return success")
	}

	// Test read_file
	readRes, err := server.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if readMap, ok := readRes.(map[string]interface{}); !ok || readMap["content"] != "hello world" {
		t.Errorf("read_file returned wrong content")
	}

	// Test list_directory
	listRes, err := server.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	listMap, ok := listRes.(map[string]interface{})
	if !ok {
		t.Fatalf("list_directory didn't return map")
	}
	files, ok := listMap["files"].([]string)
	if !ok || len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("list_directory returned wrong files: %v", listMap["files"])
	}

	// Test search_files
	searchRes, err := server.CallTool(ctx, "search_files", map[string]interface{}{
		"path":    ".",
		"pattern": "*.txt",
	})
	if err != nil {
		t.Fatalf("CallTool search_files failed: %v", err)
	}
	searchMap, ok := searchRes.(map[string]interface{})
	if !ok {
		t.Fatalf("search_files didn't return map")
	}
	results, ok := searchMap["results"].([]string)
	if !ok || len(results) != 1 || results[0] != "test.txt" {
		t.Errorf("search_files returned wrong results: %v", searchMap["results"])
	}

	// Test unknown tool
	_, err = server.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Errorf("CallTool unknown_tool should have failed")
	}

	// Test schema
	schema := server.GetToolDescription()
	if len(schema) == 0 {
		t.Errorf("schema should not be empty")
	}
}

func TestNewHybridFSProvider(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	provider, err := NewHybridFSProvider(".")
	if err != nil {
		t.Fatalf("failed to create hybrid provider: %v", err)
	}

	_, ok := provider.(*LocalFSProvider)
	if !ok {
		t.Errorf("expected LocalFSProvider in standalone mode")
	}

	os.Setenv("OHC_STANDALONE", "false")
	provider, err = NewHybridFSProvider(".")
	if err != nil {
		t.Fatalf("failed to create hybrid provider: %v", err)
	}

	_, ok = provider.(*CloudFSProvider)
	if !ok {
		t.Errorf("expected CloudFSProvider in cloud mode")
	}
}
