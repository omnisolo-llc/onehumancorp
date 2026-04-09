package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestFSServer(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	server := NewFSServer(provider)
	ctx := context.Background()

	// Test Tools list
	tools := server.GetTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}

	// Test WriteFile
	writeArgs := WriteFileArgs{
		Path:    "hello.txt",
		Content: "world",
	}
	argsRaw, _ := json.Marshal(writeArgs)
	res, err := server.CallTool(ctx, "write_file", argsRaw)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if !strings.Contains(res, "Successfully wrote") {
		t.Errorf("unexpected response: %s", res)
	}

	// Test ReadFile
	readArgs := ReadFileArgs{
		Path: "hello.txt",
	}
	argsRaw, _ = json.Marshal(readArgs)
	res, err = server.CallTool(ctx, "read_file", argsRaw)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if res != "world" {
		t.Errorf("expected 'world', got '%s'", res)
	}

	// Test ListDirectory
	listArgs := ListDirArgs{
		Path: ".",
	}
	argsRaw, _ = json.Marshal(listArgs)
	res, err = server.CallTool(ctx, "list_directory", argsRaw)
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if !strings.Contains(res, "hello.txt") {
		t.Errorf("expected listing to contain 'hello.txt', got '%s'", res)
	}

	// Test Unknown Tool
	_, err = server.CallTool(ctx, "unknown_tool", []byte("{}"))
	if err == nil {
		t.Errorf("expected error for unknown tool, got nil")
	}
}

func TestNewProviderFromEnv(t *testing.T) {
	// Test Standalone (Default)
	os.Setenv("OHC_MULTITENANT", "")
	os.Setenv("OHC_STANDALONE_FS_BASE", "")
	provider, err := NewProviderFromEnv()
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if _, ok := provider.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider")
	}

	// Test Standalone with specific path
	tempDir := t.TempDir()
	os.Setenv("OHC_STANDALONE_FS_BASE", tempDir)
	provider, err = NewProviderFromEnv()
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if localProv, ok := provider.(*LocalFSProvider); !ok || localProv.baseDir != filepath.Clean(tempDir) {
		t.Errorf("expected LocalFSProvider with path %s", tempDir)
	}

	// Test Multitenant
	os.Setenv("OHC_MULTITENANT", "true")
	os.Setenv("OHC_CLOUD_FS_BASE", "")
	provider, err = NewProviderFromEnv()
	if err != nil {
		t.Errorf("expected no error, got %v", err)
	}
	if cloudProv, ok := provider.(*CloudFSProvider); !ok || cloudProv.baseDir != "/mnt/k8s/tenant-volumes" {
		t.Errorf("expected CloudFSProvider with default path")
	}

	// Clean up
	os.Setenv("OHC_MULTITENANT", "")
	os.Setenv("OHC_CLOUD_FS_BASE", "")
	os.Setenv("OHC_STANDALONE_FS_BASE", "")
}
