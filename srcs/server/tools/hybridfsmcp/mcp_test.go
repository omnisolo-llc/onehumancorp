package hybridfsmcp

import (
	"context"
	"encoding/json"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_NormalOps(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Write file
	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"), 0644)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read file
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello world" {
		t.Errorf("expected 'hello world', got '%s'", string(data))
	}

	// List dir
	infos, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("expected 1 file named 'test.txt', got %v", infos)
	}
}

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	// Attempt path traversal
	err = provider.WriteFile(ctx, "../outside.txt", []byte("evil"), 0644)
	if err == nil {
		t.Errorf("expected error on path traversal, got none")
	}

	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("expected error on path traversal read, got none")
	}

	_, err = provider.ListDir(ctx, "../")
	if err == nil {
		t.Errorf("expected error on path traversal list, got none")
	}
}

func TestCloudFSProvider_NormalOpsAndIsolation(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)

	// Context with organization ID
	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write file for org-123
	err = provider.WriteFile(ctx, "data.txt", []byte("org data"), 0644)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read file
	data, err := provider.ReadFile(ctx, "data.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "org data" {
		t.Errorf("expected 'org data', got '%s'", string(data))
	}

	// Verify file is correctly isolated in filesystem
	_, err = os.Stat(filepath.Join(tmpDir, "org-123", "data.txt"))
	if err != nil {
		t.Errorf("expected file to be created in isolated directory, got err: %v", err)
	}

	// Context with DIFFERENT organization ID
	claims2 := &auth.Claims{OrganizationID: "org-456"}
	ctx2 := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims2)

	// Attempt to read org-123's file using path traversal
	_, err = provider.ReadFile(ctx2, "../org-123/data.txt")
	if err == nil {
		t.Errorf("expected error reading other org's file, got none")
	}

	// Context without claims
	ctx3 := context.Background()
	_, err = provider.ReadFile(ctx3, "data.txt")
	if err == nil {
		t.Errorf("expected error reading without claims, got none")
	}
}

func TestCloudFSProvider_ListDir_Error(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewCloudFSProvider(tmpDir)
	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err = provider.ListDir(ctx, "nonexistent")
	if err == nil {
		t.Errorf("expected error listing nonexistent dir, got none")
	}
}


func TestHybridFSServer_Tools(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_FS_ROOT", tmpDir)
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_FS_ROOT")
	defer os.Unsetenv("OHC_MULTITENANT")

	server, err := NewServer(context.Background())
	if err != nil {
		t.Fatalf("NewServer failed: %v", err)
	}

	ctx := context.Background()

	// Write File Tool
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "mcp.txt", Content: "mcp test"})
	res := server.CallTool(ctx, "write_file", writeArgs)
	if res.Status != "success" {
		t.Fatalf("write_file tool failed: %v", string(res.ResultData))
	}

	// Read File Tool
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "mcp.txt"})
	res = server.CallTool(ctx, "read_file", readArgs)
	if res.Status != "success" {
		t.Fatalf("read_file tool failed: %v", string(res.ResultData))
	}
	if string(res.ResultData) != "mcp test" {
		t.Errorf("expected 'mcp test', got '%s'", string(res.ResultData))
	}

	// List Dir Tool
	listArgs, _ := json.Marshal(ListDirArgs{Path: "."})
	res = server.CallTool(ctx, "list_directory", listArgs)
	if res.Status != "success" {
		t.Fatalf("list_directory tool failed: %v", string(res.ResultData))
	}

	// Error paths
	res = server.CallTool(ctx, "unknown_tool", nil)
	if res.Status != "error" {
		t.Errorf("expected error for unknown tool")
	}

	res = server.CallTool(ctx, "read_file", []byte("invalid json"))
	if res.Status != "error" {
		t.Errorf("expected error for invalid json on read_file")
	}

	res = server.CallTool(ctx, "write_file", []byte("invalid json"))
	if res.Status != "error" {
		t.Errorf("expected error for invalid json on write_file")
	}

	res = server.CallTool(ctx, "list_directory", []byte("invalid json"))
	if res.Status != "error" {
		t.Errorf("expected error for invalid json on list_directory")
	}
}

func TestHybridFSServer_CloudMode(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "mcp_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	os.Setenv("OHC_FS_ROOT", tmpDir)
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_FS_ROOT")
	defer os.Unsetenv("OHC_MULTITENANT")

	server, err := NewServer(context.Background())
	if err != nil {
		t.Fatalf("NewServer failed: %v", err)
	}

	claims := &auth.Claims{OrganizationID: "org-mcp"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Write File Tool
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "mcp-cloud.txt", Content: "cloud data"})
	res := server.CallTool(ctx, "write_file", writeArgs)
	if res.Status != "success" {
		t.Fatalf("write_file tool failed: %v", string(res.ResultData))
	}

	// Read File Tool
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "mcp-cloud.txt"})
	res = server.CallTool(ctx, "read_file", readArgs)
	if res.Status != "success" {
		t.Fatalf("read_file tool failed: %v", string(res.ResultData))
	}
	if string(res.ResultData) != "cloud data" {
		t.Errorf("expected 'cloud data', got '%s'", string(res.ResultData))
	}

	// Error on write without permissions
	ctxNoAuth := context.Background()
	res = server.CallTool(ctxNoAuth, "write_file", writeArgs)
	if res.Status != "error" {
		t.Errorf("expected error writing without auth, got success")
	}

	res = server.CallTool(ctxNoAuth, "read_file", readArgs)
	if res.Status != "error" {
		t.Errorf("expected error reading without auth, got success")
	}
}


func TestLocalFSProvider_Errors(t *testing.T) {
	provider := NewLocalFSProvider("/nonexistent/dir")
	ctx := context.Background()
	_, err := provider.ReadFile(ctx, "test.txt")
	if err == nil {
		t.Errorf("expected error reading nonexistent file")
	}

	err = provider.WriteFile(ctx, "test.txt", []byte("data"), 0644)
	if err == nil {
		t.Errorf("expected error writing to nonexistent base dir")
	}

	_, err = provider.ListDir(ctx, "test.txt")
	if err == nil {
		t.Errorf("expected error listing nonexistent dir")
	}
}

func TestCloudFSProvider_Errors(t *testing.T) {
	provider := NewCloudFSProvider("/nonexistent/dir")
	claims := &auth.Claims{OrganizationID: "org-123"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	_, err := provider.ReadFile(ctx, "test.txt")
	if err == nil {
		t.Errorf("expected error reading nonexistent file")
	}

	err = provider.WriteFile(ctx, "test.txt", []byte("data"), 0644)
	if err == nil {
		t.Errorf("expected error writing to nonexistent base dir")
	}
}

func TestListDir_ErrorPaths(t *testing.T) {
	// Test file read dir error (passing file path to list dir)
	tmpDir, err := os.MkdirTemp("", "listdir_err_test")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	err = os.WriteFile(filepath.Join(tmpDir, "file.txt"), []byte("data"), 0644)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()

	_, err = provider.ListDir(ctx, "file.txt")
	if err == nil {
		t.Errorf("expected error listing a file, got none")
	}

	cloudProvider := NewCloudFSProvider(tmpDir)
	claims := &auth.Claims{OrganizationID: "org-123"}
	ctxCloud := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err = cloudProvider.WriteFile(ctxCloud, "cfile.txt", []byte("data"), 0644)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	_, err = cloudProvider.ListDir(ctxCloud, "cfile.txt")
	if err == nil {
		t.Errorf("expected error listing a file, got none")
	}
}

func TestListDir_ErrorPaths2(t *testing.T) {
	provider := NewLocalFSProvider("/dev/null/foo")
	ctx := context.Background()
	_, err := provider.ListDir(ctx, ".")
	if err == nil {
		t.Errorf("expected error listing dir on invalid base")
	}

	cProvider := NewCloudFSProvider("/dev/null/foo")
	claims := &auth.Claims{OrganizationID: "org"}
	ctxC := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)
	_, err = cProvider.ListDir(ctxC, ".")
	if err == nil {
		t.Errorf("expected error listing dir on invalid base")
	}
}

func TestHybridFSServer_ToolArgsUnmarshalError(t *testing.T) {
	tmpDir := "/tmp/foo"
	os.Setenv("OHC_FS_ROOT", tmpDir)
	defer os.Unsetenv("OHC_FS_ROOT")

	server, _ := NewServer(context.Background())
	ctx := context.Background()

	res := server.CallTool(ctx, "read_file", []byte("{invalid}"))
	if res.Status != "error" {
		t.Errorf("expected error")
	}

	res = server.CallTool(ctx, "write_file", []byte("{invalid}"))
	if res.Status != "error" {
		t.Errorf("expected error")
	}

	res = server.CallTool(ctx, "list_directory", []byte("{invalid}"))
	if res.Status != "error" {
		t.Errorf("expected error")
	}
}
