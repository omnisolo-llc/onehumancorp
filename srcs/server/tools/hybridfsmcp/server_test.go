package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"testing"
)

func TestProviderFactory(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	provider := NewProviderFactory()
	if _, ok := provider.(*CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider, got %T", provider)
	}

	os.Setenv("OHC_MULTITENANT", "false")
	provider = NewProviderFactory()
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider, got %T", provider)
	}
}

func TestServer_Tools(t *testing.T) {
	provider := NewLocalFSProvider()
	server := NewServer(provider)

	if server.Name() != "hybridfs" {
		t.Errorf("Expected name 'hybridfs', got %s", server.Name())
	}

	tools := server.Tools()
	if len(tools) != 4 {
		t.Errorf("Expected 4 tools, got %d", len(tools))
	}
}

func TestServer_Execute(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "server_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_FS_ROOT", tempDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	provider := NewLocalFSProvider()
	server := NewServer(provider)
	ctx := context.Background()

	// write_file
	params := map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	}
	res, err := server.Execute(ctx, "write_file", params)
	if err != nil {
		t.Fatalf("write_file failed: %v", err)
	}
	if res.Status != "success" {
		t.Errorf("write_file status: expected 'success', got '%s'", res.Status)
	}

	// read_file
	params = map[string]interface{}{
		"path": "hello.txt",
	}
	res, err = server.Execute(ctx, "read_file", params)
	if err != nil {
		t.Fatalf("read_file failed: %v", err)
	}
	var resData map[string]string
	if err := json.Unmarshal(res.ResultData, &resData); err != nil {
		t.Fatalf("failed to unmarshal read_file result: %v", err)
	}
	if resData["content"] != "world" {
		t.Errorf("Expected 'world', got '%s'", resData["content"])
	}

	// list_directory
	params = map[string]interface{}{
		"path": ".",
	}
	res, err = server.Execute(ctx, "list_directory", params)
	if err != nil {
		t.Fatalf("list_directory failed: %v", err)
	}
	var resList map[string][]map[string]interface{}
	if err := json.Unmarshal(res.ResultData, &resList); err != nil {
		t.Fatalf("failed to unmarshal list_directory result: %v", err)
	}
	if len(resList["files"]) != 1 || resList["files"][0]["name"] != "hello.txt" {
		t.Errorf("Expected exactly one file named hello.txt, got %v", resList["files"])
	}

	// search_files
	params = map[string]interface{}{
		"path":    ".",
		"pattern": "hello",
	}
	res, err = server.Execute(ctx, "search_files", params)
	if err != nil {
		t.Fatalf("search_files failed: %v", err)
	}
	var resSearch map[string][]string
	if err := json.Unmarshal(res.ResultData, &resSearch); err != nil {
		t.Fatalf("failed to unmarshal search_files result: %v", err)
	}
	if len(resSearch["matches"]) != 1 || resSearch["matches"][0] != "hello.txt" {
		t.Errorf("Expected hello.txt match, got %v", resSearch["matches"])
	}

	// search_files empty pattern
	params = map[string]interface{}{
		"path":    ".",
		"pattern": "",
	}
	res, err = server.Execute(ctx, "search_files", params)
	if err != nil {
		t.Fatalf("search_files failed: %v", err)
	}
	if err := json.Unmarshal(res.ResultData, &resSearch); err != nil {
		t.Fatalf("failed to unmarshal search_files result: %v", err)
	}
	if len(resSearch["matches"]) != 1 || resSearch["matches"][0] != "hello.txt" {
		t.Errorf("Expected hello.txt match, got %v", resSearch["matches"])
	}

	// invalid tool
	_, err = server.Execute(ctx, "invalid_tool", params)
	if err == nil {
		t.Errorf("Expected error for invalid tool")
	}

	// missing params testing
	_, err = server.Execute(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Errorf("Expected error for missing path")
	}

	_, err = server.Execute(ctx, "write_file", map[string]interface{}{})
	if err == nil {
		t.Errorf("Expected error for missing path")
	}

	_, err = server.Execute(ctx, "write_file", map[string]interface{}{"path": "test.txt"})
	if err == nil {
		t.Errorf("Expected error for missing content")
	}

	_, err = server.Execute(ctx, "list_directory", map[string]interface{}{})
	if err == nil {
		t.Errorf("Expected error for missing path")
	}

	_, err = server.Execute(ctx, "search_files", map[string]interface{}{})
	if err == nil {
		t.Errorf("Expected error for missing path")
	}

	_, err = server.Execute(ctx, "search_files", map[string]interface{}{"path": "."})
	if err == nil {
		t.Errorf("Expected error for missing pattern")
	}
}

func TestStringContains(t *testing.T) {
	if !stringContains("hello", "ell") {
		t.Errorf("Expected true")
	}
	if stringContains("hello", "world") {
		t.Errorf("Expected false")
	}
	if !stringContains("hello", "") {
		t.Errorf("Expected true")
	}
	if stringContains("hi", "hello") {
		t.Errorf("Expected false")
	}
}
