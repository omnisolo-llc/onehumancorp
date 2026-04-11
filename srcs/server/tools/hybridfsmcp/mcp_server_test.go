package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestMCPServer(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp_server_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create LocalFSProvider: %v", err)
	}

	server := NewServer(provider)

	if server.Name() != "hybrid_fs" {
		t.Errorf("expected 'hybrid_fs', got '%s'", server.Name())
	}

	tools := server.Tools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// Test write_file
	writeParams := []byte(`{"path": "test.txt", "content": "hello mcp"}`)
	res := server.Execute(ctx, "write_file", writeParams)
	if res.Status != "success" {
		t.Errorf("write_file failed: %v", string(res.ResultData))
	}

	// Test read_file
	readParams := []byte(`{"path": "test.txt"}`)
	res = server.Execute(ctx, "read_file", readParams)
	if res.Status != "success" {
		t.Errorf("read_file failed: %v", string(res.ResultData))
	}
	var readResult map[string]string
	json.Unmarshal(res.ResultData, &readResult)
	if readResult["content"] != "hello mcp" {
		t.Errorf("expected 'hello mcp', got '%s'", readResult["content"])
	}

	// Test list_dir
	listParams := []byte(`{"path": ""}`)
	res = server.Execute(ctx, "list_dir", listParams)
	if res.Status != "success" {
		t.Errorf("list_dir failed: %v", string(res.ResultData))
	}
	var listResult map[string][]string
	json.Unmarshal(res.ResultData, &listResult)
	if len(listResult["files"]) != 1 || listResult["files"][0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", listResult["files"])
	}

	// Test unknown tool
	res = server.Execute(ctx, "unknown_tool", []byte(`{}`))
	if res.Status != "error" {
		t.Errorf("expected error for unknown tool, got %s", res.Status)
	}

	// Test invalid JSON
	res = server.Execute(ctx, "read_file", []byte(`{"path":`))
	if res.Status != "error" {
		t.Errorf("expected error for invalid json, got %s", res.Status)
	}
}

func TestMCPFSProviderStandalone(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	provider, err := MCPFSProvider("/tmp")
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider, got %T", provider)
	}
}

func TestMCPFSProviderCloud(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "false")
	defer os.Unsetenv("OHC_STANDALONE")

	provider, err := MCPFSProvider("/tmp")
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider, got %T", provider)
	}
}
