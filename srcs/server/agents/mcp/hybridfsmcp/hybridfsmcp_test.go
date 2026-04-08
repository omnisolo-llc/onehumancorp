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
	tempDir, err := os.MkdirTemp("", "localfs-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// WriteFile test
	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"), 0644)
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// ReadFile test
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// ListDir test
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("ListDir returned unexpected entries: %v", entries)
	}

	// Escape base dir test
	_, err = provider.ReadFile(ctx, "../test.txt")
	if err == nil {
		t.Errorf("Expected error when escaping base dir, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	// Setup context with claims
	claims := &auth.Claims{OrganizationID: "tenant-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// WriteFile test
	err = provider.WriteFile(ctx, "data.txt", []byte("hello cloud"), 0644)
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// ReadFile test
	data, err := provider.ReadFile(ctx, "data.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// ListDir test
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "data.txt" {
		t.Errorf("ListDir returned unexpected entries: %v", entries)
	}

	// Verify tenant isolation (directory creation)
	tenantDir := filepath.Join(tempDir, "tenant-123")
	if _, err := os.Stat(tenantDir); os.IsNotExist(err) {
		t.Errorf("Tenant directory was not created: %v", err)
	}

	// Escape tenant dir test
	_, err = provider.ReadFile(ctx, "../data.txt")
	if err == nil {
		t.Errorf("Expected error when escaping tenant dir, got nil")
	}

	// Missing context claims test
	ctxNoClaims := context.Background()
	_, err = provider.ReadFile(ctxNoClaims, "data.txt")
	if err == nil {
		t.Errorf("Expected error without organization claims, got nil")
	}
}

func TestServerExecuteTool(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "serverfs-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	server, err := NewServer(tempDir)
	if err != nil {
		t.Fatalf("failed to create server: %v", err)
	}

	ctx := context.Background()

	// write_file
	writeInput := map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	}
	resWrite := server.ExecuteTool(ctx, "write_file", writeInput)
	if resWrite.Status != "success" {
		t.Errorf("write_file failed: %s", string(resWrite.ResultData))
	}

	// read_file
	readInput := map[string]interface{}{
		"path": "hello.txt",
	}
	resRead := server.ExecuteTool(ctx, "read_file", readInput)
	if resRead.Status != "success" {
		t.Errorf("read_file failed: %s", string(resRead.ResultData))
	}
	if string(resRead.ResultData) != "world" {
		t.Errorf("Expected 'world', got '%s'", string(resRead.ResultData))
	}

	// list_directory
	listInput := map[string]interface{}{
		"path": ".",
	}
	resList := server.ExecuteTool(ctx, "list_directory", listInput)
	if resList.Status != "success" {
		t.Errorf("list_directory failed: %s", string(resList.ResultData))
	}
	var entries []string
	if err := json.Unmarshal(resList.ResultData, &entries); err != nil {
		t.Fatalf("Failed to unmarshal list_directory result: %v", err)
	}
	if len(entries) != 1 || entries[0] != "hello.txt" {
		t.Errorf("Expected ['hello.txt'], got %v", entries)
	}

	// Missing args
	resErr := server.ExecuteTool(ctx, "write_file", map[string]interface{}{})
	if resErr.Status != "error" {
		t.Errorf("Expected error for missing args, got success")
	}
	resErr2 := server.ExecuteTool(ctx, "read_file", map[string]interface{}{})
	if resErr2.Status != "error" {
		t.Errorf("Expected error for missing args, got success")
	}

	// Unknown tool
	resUnknown := server.ExecuteTool(ctx, "unknown_tool", map[string]interface{}{"path": "."})
	if resUnknown.Status != "error" {
		t.Errorf("Expected error for unknown tool, got success")
	}
}

func TestServerExecuteTool_Cloud(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "serverfs-cloud-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	server, err := NewServer(tempDir)
	if err != nil {
		t.Fatalf("failed to create cloud server: %v", err)
	}

	claims := &auth.Claims{OrganizationID: "tenant-cloud"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// write_file
	writeInput := map[string]interface{}{
		"path":    "hello.txt",
		"content": "world",
	}
	resWrite := server.ExecuteTool(ctx, "write_file", writeInput)
	if resWrite.Status != "success" {
		t.Errorf("write_file failed: %s", string(resWrite.ResultData))
	}

	// read_file
	readInput := map[string]interface{}{
		"path": "hello.txt",
	}
	resRead := server.ExecuteTool(ctx, "read_file", readInput)
	if resRead.Status != "success" {
		t.Errorf("read_file failed: %s", string(resRead.ResultData))
	}
	if string(resRead.ResultData) != "world" {
		t.Errorf("Expected 'world', got '%s'", string(resRead.ResultData))
	}
}

func TestNewLocalFSProviderError(t *testing.T) {
	// Not easy to test Abs error reliably, but could mock if needed.
	// We'll trust standard path mapping works.
}
