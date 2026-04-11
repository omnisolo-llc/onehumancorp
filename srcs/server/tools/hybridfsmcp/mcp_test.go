package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_ResolvePath(t *testing.T) {
	tmpDir := t.TempDir()
	provider := &LocalFSProvider{BaseDir: tmpDir}

	tests := []struct {
		name    string
		target  string
		wantErr bool
	}{
		{"Valid File", "test.txt", false},
		{"Valid Directory", "subdir/test.txt", false},
		{"Valid Absolute Path inside base", filepath.Join(tmpDir, "test.txt"), false},
		{"Path Traversal 1", "../test.txt", true},
		{"Path Traversal 2", "../../etc/passwd", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := provider.resolvePath(tt.target)
			if (err != nil) != tt.wantErr {
				t.Errorf("LocalFSProvider.resolvePath() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}

func TestLocalFSProvider_Operations(t *testing.T) {
	tmpDir := t.TempDir()
	provider := &LocalFSProvider{BaseDir: tmpDir}
	ctx := context.Background()

	// Write File
	err := provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read File
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// List Dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}
}

func TestCloudFSProvider_ResolvePath(t *testing.T) {
	tmpDir := t.TempDir()
	provider := &CloudFSProvider{BaseDir: tmpDir}

	claims := &auth.Claims{
		OrganizationID: "tenant-a",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	tests := []struct {
		name    string
		target  string
		wantErr bool
	}{
		{"Valid File", "test.txt", false},
		{"Valid Directory", "subdir/test.txt", false},
		{"Path Traversal to base", "../test.txt", true},
		{"Path Traversal to other tenant", "../tenant-b/test.txt", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			fullPath, err := provider.resolvePath(ctx, tt.target)
			if (err != nil) != tt.wantErr {
				t.Errorf("CloudFSProvider.resolvePath() error = %v, wantErr %v", err, tt.wantErr)
			}

			if err == nil {
				if !strings.Contains(fullPath, "tenant-a") {
					t.Errorf("Expected resolved path to contain tenant id 'tenant-a', got %s", fullPath)
				}
			}
		})
	}
}

func TestCloudFSProvider_Operations(t *testing.T) {
	tmpDir := t.TempDir()
	provider := &CloudFSProvider{BaseDir: tmpDir}

	claims := &auth.Claims{
		OrganizationID: "tenant-cloud",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write File
	err := provider.WriteFile(ctx, "cloud.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify isolation on disk
	diskData, err := os.ReadFile(filepath.Join(tmpDir, "tenant-cloud", "cloud.txt"))
	if err != nil {
		t.Fatalf("File not found at expected isolated path: %v", err)
	}
	if string(diskData) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(diskData))
	}

	// Read File
	data, err := provider.ReadFile(ctx, "cloud.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// List Dir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "cloud.txt" {
		t.Errorf("Expected ['cloud.txt'], got %v", entries)
	}

	// Test unauthorized
	ctxNoAuth := context.Background()
	_, err = provider.ReadFile(ctxNoAuth, "cloud.txt")
	if err == nil {
		t.Errorf("Expected error without claims, got nil")
	}
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	tmpDir := t.TempDir()

	// Test Factory (Standalone mode)
	os.Setenv("OHC_MULTITENANT", "false")
	mcp := NewHybridFSMCP(tmpDir)

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// 1. write_file
	writeArgs := map[string]interface{}{
		"path":    "hello.txt",
		"content": "hybrid content",
	}
	writeRes, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	if writeRes.(map[string]interface{})["status"] != "success" {
		t.Errorf("Expected success status, got %v", writeRes)
	}

	// 2. read_file
	readArgs := map[string]interface{}{
		"path": "hello.txt",
	}
	readRes, err := mcp.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if readRes.(map[string]interface{})["content"] != "hybrid content" {
		t.Errorf("Expected 'hybrid content', got %v", readRes)
	}

	// 3. list_directory
	listArgs := map[string]interface{}{
		"path": ".",
	}
	listRes, err := mcp.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	entries := listRes.(map[string]interface{})["entries"].([]string)
	if len(entries) != 1 || entries[0] != "hello.txt" {
		t.Errorf("Expected ['hello.txt'], got %v", entries)
	}

	// Test invalid tool
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Errorf("Expected error for unknown tool")
	}
}

func TestHybridFSMCP_CloudMode(t *testing.T) {
	tmpDir := t.TempDir()

	// Test Factory (Cloud mode)
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	mcp := NewHybridFSMCP(tmpDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-cloud-mcp",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	writeArgs := map[string]interface{}{
		"path":    "test.txt",
		"content": "cloud content",
	}

	_, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("CallTool write_file in cloud mode failed: %v", err)
	}
}
