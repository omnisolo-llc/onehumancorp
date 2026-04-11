package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Test WriteFile
	testPath := "test.txt"
	testContent := []byte("hello local")
	if err := provider.WriteFile(ctx, testPath, testContent); err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	readContent, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readContent) != string(testContent) {
		t.Errorf("expected %q, got %q", string(testContent), string(readContent))
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("expected 1 file named test.txt, got %d files", len(infos))
	}

	// Test Path Traversal
	if _, err := provider.ReadFile(ctx, "../outside.txt"); err == nil {
		t.Error("expected error when reading outside workspace, got nil")
	}

	if err := provider.WriteFile(ctx, "../outside.txt", testContent); err == nil {
		t.Error("expected error when writing outside workspace, got nil")
	}

	if _, err := provider.ListDir(ctx, "../outside"); err == nil {
		t.Error("expected error when listing outside workspace, got nil")
	}

	// Test ListDir empty dir
	emptyDir := "empty"
	os.Mkdir(filepath.Join(tmpDir, emptyDir), 0755)
	infos, err = provider.ListDir(ctx, emptyDir)
	if err != nil {
		t.Fatalf("ListDir empty failed: %v", err)
	}
	if len(infos) != 0 {
		t.Errorf("expected 0 files, got %d files", len(infos))
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	// Create context with tenant claims
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	testPath := "test.txt"
	testContent := []byte("hello cloud")
	if err := provider.WriteFile(ctx, testPath, testContent); err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify file is actually in tenant dir
	fullPath := filepath.Join(tmpDir, "tenant1", testPath)
	if _, err := os.Stat(fullPath); os.IsNotExist(err) {
		t.Errorf("expected file to be created in tenant dir: %s", fullPath)
	}

	// Test ReadFile
	readContent, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(readContent) != string(testContent) {
		t.Errorf("expected %q, got %q", string(testContent), string(readContent))
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("expected 1 file named test.txt, got %d files", len(infos))
	}

	// Test Missing Claims
	ctxNoClaims := context.Background()
	if _, err := provider.ReadFile(ctxNoClaims, testPath); err == nil {
		t.Error("expected error when no claims present, got nil")
	}

	// Test Path Traversal within tenant scope
	if _, err := provider.ReadFile(ctx, "../tenant2/test.txt"); err == nil {
		t.Error("expected error when reading outside tenant scope, got nil")
	}

	if err := provider.WriteFile(ctx, "../tenant2/test.txt", testContent); err == nil {
		t.Error("expected error when writing outside tenant scope, got nil")
	}

	if _, err := provider.ListDir(ctx, "../tenant2"); err == nil {
		t.Error("expected error when listing outside tenant scope, got nil")
	}

	// Test Invalid Path
	invalidCtx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: ""})
	if _, err := provider.ReadFile(invalidCtx, testPath); err == nil {
		t.Error("expected error when org id empty, got nil")
	}
}

func TestFileSystemMCP(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "fsmcp-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	mcp := NewFileSystemMCP("false", tmpDir)
	ctx := context.Background()

	// ListTools
	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("expected 3 tools, got %d", len(tools))
	}

	// CallTool: write_file
	writeArgs := map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp hello",
	}
	writeRes, err := mcp.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}
	if writeRes.(map[string]interface{})["status"] != "success" {
		t.Errorf("expected success status, got %v", writeRes)
	}

	// CallTool: write_file missing args
	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{})
	if err == nil {
		t.Error("expected error with missing args")
	}

	_, err = mcp.CallTool(ctx, "write_file", map[string]interface{}{"path": "test"})
	if err == nil {
		t.Error("expected error with missing content")
	}

	// CallTool: read_file
	readArgs := map[string]interface{}{
		"path": "mcp_test.txt",
	}
	readRes, err := mcp.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	if readRes.(map[string]interface{})["content"] != "mcp hello" {
		t.Errorf("expected 'mcp hello', got %v", readRes)
	}

	// CallTool: read_file missing args
	_, err = mcp.CallTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Error("expected error with missing args")
	}

	// CallTool: list_directory
	listArgs := map[string]interface{}{
		"path": ".",
	}
	listRes, err := mcp.CallTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	files := listRes.(map[string]interface{})["files"].([]map[string]interface{})
	if len(files) != 1 || files[0]["name"] != "mcp_test.txt" {
		t.Errorf("expected 1 file named mcp_test.txt, got %v", files)
	}

	// CallTool: list_directory default path
	listResDefault, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{})
	if err != nil {
		t.Fatalf("CallTool list_directory default failed: %v", err)
	}
	filesDefault := listResDefault.(map[string]interface{})["files"].([]map[string]interface{})
	if len(filesDefault) != 1 || filesDefault[0]["name"] != "mcp_test.txt" {
		t.Errorf("expected 1 file named mcp_test.txt, got %v", filesDefault)
	}

	// CallTool: unknown
	_, err = mcp.CallTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Error("expected error with unknown tool")
	}

	// Test multitenant mode
	mcpCloud := NewFileSystemMCP("true", tmpDir)
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctxCloud := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err = mcpCloud.CallTool(ctxCloud, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("CallTool cloud write_file failed: %v", err)
	}
}
