package mcp

import (
	"context"
	"encoding/json"
	"io/ioutil"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathEscape(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	tmpDir, err := ioutil.TempDir("", "localfs_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewFileSystemProvider(context.Background(), tmpDir)

	ctx := context.Background()
	_, err = provider.ReadFile(ctx, "../escape")
	if err != ErrPathEscape {
		t.Errorf("expected ErrPathEscape, got %v", err)
	}

	_, err = provider.ListDir(ctx, "..")
	if err != ErrPathEscape {
		t.Errorf("expected ErrPathEscape, got %v", err)
	}

	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err != ErrPathEscape {
		t.Errorf("expected ErrPathEscape, got %v", err)
	}
}

func TestLocalFSProvider_Success(t *testing.T) {
	// Set standalone mode
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	tmpDir, err := ioutil.TempDir("", "localfs_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewFileSystemProvider(context.Background(), tmpDir)
	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}
}

func TestCloudFSProvider_TenantScoping(t *testing.T) {
	os.Unsetenv("OHC_STANDALONE")

	tmpDir, err := ioutil.TempDir("", "cloudfs_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewFileSystemProvider(context.Background(), tmpDir)

	// Create context with claims
	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err = provider.WriteFile(ctx, "test.txt", []byte("tenant data"))
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Check if data is actually in tenant-123 dir
	fullPath := filepath.Join(tmpDir, "tenant-123", "test.txt")
	if _, err := os.Stat(fullPath); os.IsNotExist(err) {
		t.Errorf("file not written to correct tenant directory")
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "tenant data" {
		t.Errorf("expected 'tenant data', got '%s'", string(data))
	}

	// Test Path Escape for CloudFS
	_, err = provider.ReadFile(ctx, "../escape")
	if err != ErrPathEscape {
		t.Errorf("expected ErrPathEscape, got %v", err)
	}

	// Test No Auth
	ctxNoAuth := context.Background()
	_, err = provider.ReadFile(ctxNoAuth, "test.txt")
	if err != ErrNoAuth {
		t.Errorf("expected ErrNoAuth, got %v", err)
	}
}

func TestFileSystemServer(t *testing.T) {
	tmpDir, err := ioutil.TempDir("", "server_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")
	provider := NewFileSystemProvider(context.Background(), tmpDir)
	server := NewFileSystemServer(provider)
	ctx := context.Background()

	// Write File Tool
	writeArgs := map[string]string{"path": "file.txt", "data": "tool data"}
	writeBytes, _ := json.Marshal(writeArgs)
	resWrite := server.HandleToolCall(ctx, "write_file", writeBytes)
	if resWrite.Status != "success" {
		t.Errorf("write_file tool failed: %s", string(resWrite.ResultData))
	}

	// Read File Tool
	readArgs := map[string]string{"path": "file.txt"}
	readBytes, _ := json.Marshal(readArgs)
	resRead := server.HandleToolCall(ctx, "read_file", readBytes)
	if resRead.Status != "success" {
		t.Errorf("read_file tool failed: %s", string(resRead.ResultData))
	}

	// List Dir Tool
	listArgs := map[string]string{"path": "."}
	listBytes, _ := json.Marshal(listArgs)
	resList := server.HandleToolCall(ctx, "list_directory", listBytes)
	if resList.Status != "success" {
		t.Errorf("list_directory tool failed: %s", string(resList.ResultData))
	}

	// Error path: Invalid JSON
	resErr := server.HandleToolCall(ctx, "read_file", []byte("invalid json"))
	if resErr.Status != "error" {
		t.Errorf("expected error status for invalid json")
	}

	// Error path: Unknown tool
	resUnknown := server.HandleToolCall(ctx, "unknown_tool", []byte("{}"))
	if resUnknown.Status != "error" {
		t.Errorf("expected error status for unknown tool")
	}
}
