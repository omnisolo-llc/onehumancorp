package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test successful write
	err := provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test successful read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got %s", string(data))
	}

	// Test list dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test path traversal write
	err = provider.WriteFile(ctx, "../escaped.txt", []byte("bad"))
	if err == nil || !strings.Contains(err.Error(), "traversal") {
		t.Errorf("Expected traversal error, got %v", err)
	}

	// Test path traversal read
	_, err = provider.ReadFile(ctx, "../escaped.txt")
	if err == nil || !strings.Contains(err.Error(), "traversal") {
		t.Errorf("Expected traversal error, got %v", err)
	}

	// Test path traversal list
	_, err = provider.ListDir(ctx, "../")
	if err == nil || !strings.Contains(err.Error(), "traversal") {
		t.Errorf("Expected traversal error, got %v", err)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	// Create context with claims
	claims := &auth.Claims{OrganizationID: "tenant-a"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test successful write
	err := provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify file was written to correct tenant dir
	fullPath := filepath.Join(tempDir, "tenant-a", "test.txt")
	if _, err := os.Stat(fullPath); os.IsNotExist(err) {
		t.Errorf("File not written to correct tenant dir: %s", fullPath)
	}

	// Test successful read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got %s", string(data))
	}

	// Test list dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test path traversal write
	err = provider.WriteFile(ctx, "../escaped.txt", []byte("bad"))
	if err == nil || !strings.Contains(err.Error(), "traversal") {
		t.Errorf("Expected traversal error, got %v", err)
	}

	// Test path traversal read
	_, err = provider.ReadFile(ctx, "../escaped.txt")
	if err == nil || !strings.Contains(err.Error(), "traversal") {
		t.Errorf("Expected traversal error, got %v", err)
	}

	// Test path traversal list
	_, err = provider.ListDir(ctx, "../")
	if err == nil || !strings.Contains(err.Error(), "traversal") {
		t.Errorf("Expected traversal error, got %v", err)
	}

	// Test no claims
	emptyCtx := context.Background()
	err = provider.WriteFile(emptyCtx, "test.txt", []byte("bad"))
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Errorf("Expected unauthorized error, got %v", err)
	}

	_, err = provider.ReadFile(emptyCtx, "test.txt")
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Errorf("Expected unauthorized error, got %v", err)
	}

	_, err = provider.ListDir(emptyCtx, ".")
	if err == nil || !strings.Contains(err.Error(), "unauthorized") {
		t.Errorf("Expected unauthorized error, got %v", err)
	}
}

func TestHybridFSProviderMCP(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "true")
	tempDir := t.TempDir()
	mcp := NewHybridFSProviderMCP(tempDir)
	ctx := context.Background()

	// Test ListTools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	// Test write_file tool
	writeArgs := map[string]interface{}{
		"path": "test.txt",
		"data": "mcp test",
	}
	writeRes, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	writeMap := writeRes.(map[string]interface{})
	if writeMap["status"] != "success" {
		t.Errorf("Expected status success, got %v", writeMap["status"])
	}

	// Test read_file tool
	readArgs := map[string]interface{}{
		"path": "test.txt",
	}
	readRes, err := mcp.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	readMap := readRes.(map[string]interface{})
	if readMap["status"] != "success" || readMap["data"] != "mcp test" {
		t.Errorf("Expected status success and data 'mcp test', got %v", readMap)
	}

	// Test list_directory tool
	listArgs := map[string]interface{}{
		"path": ".",
	}
	listRes, err := mcp.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	listMap := listRes.(map[string]interface{})
	if listMap["status"] != "success" {
		t.Errorf("Expected status success, got %v", listMap["status"])
	}
	entries := listMap["entries"].([]string)
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test invalid tool
	_, err = mcp.CallTool(ctx, "invalid_tool", nil)
	if err == nil {
		t.Errorf("Expected error for invalid tool, got nil")
	}

	// Test missing path arg
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Errorf("Expected error for missing path arg, got nil")
	}

	// Test missing data arg for write
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "a.txt"})
	if err == nil {
		t.Errorf("Expected error for missing data arg, got nil")
	}
}

func TestHybridFSProviderMCP_Cloud(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "false")
	tempDir := t.TempDir()
	mcp := NewHybridFSProviderMCP(tempDir)

	claims := &auth.Claims{OrganizationID: "tenant-b"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	writeArgs := map[string]interface{}{
		"path": "cloud.txt",
		"data": "cloud test",
	}
	_, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	readArgs := map[string]interface{}{
		"path": "cloud.txt",
	}
	readRes, err := mcp.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	readMap := readRes.(map[string]interface{})
	if readMap["data"] != "cloud test" {
		t.Errorf("Expected data 'cloud test', got %v", readMap["data"])
	}
}
