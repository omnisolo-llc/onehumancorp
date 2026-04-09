package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Test write and read
	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello world" {
		t.Fatalf("expected 'hello world', got %q", string(data))
	}

	// Test ListDir
	list, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(list) != 1 || list[0] != "test.txt" {
		t.Fatalf("expected ['test.txt'], got %v", list)
	}

	// Test path escaping
	err = provider.WriteFile(ctx, "../escape.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("expected error when path escapes workspace")
	}

	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Fatalf("expected error when reading path that escapes workspace")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	// Unauthenticated
	ctx := context.Background()
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err == nil {
		t.Fatalf("expected error when no claims present")
	}

	// Authenticated
	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Fatalf("expected 'hello cloud', got %q", string(data))
	}

	// Test ListDir
	list, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(list) != 1 || list[0] != "test.txt" {
		t.Fatalf("expected ['test.txt'], got %v", list)
	}

	// Test path escaping
	err = provider.WriteFile(ctx, "../tenant-other/test.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("expected error when path escapes tenant workspace")
	}
}

func TestServer(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "serverfs")
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

	// Write
	writeArgs := map[string]interface{}{
		"path": "data.txt",
		"data": "server data",
	}
	res, err := server.ExecuteTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("write_file tool failed: %v", err)
	}
	if res != "ok" {
		t.Fatalf("expected 'ok', got %v", res)
	}

	// Read
	readArgs := map[string]interface{}{
		"path": "data.txt",
	}
	res, err = server.ExecuteTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("read_file tool failed: %v", err)
	}
	if res != "server data" {
		t.Fatalf("expected 'server data', got %v", res)
	}

	// Search
	searchArgs := map[string]interface{}{
		"path":    "",
		"pattern": "data",
	}
	resSearch, err := server.ExecuteTool(ctx, "search_files", searchArgs)
	if err != nil {
		t.Fatalf("search_files tool failed: %v", err)
	}
	resList, ok := resSearch.([]string)
	if !ok || len(resList) != 1 || resList[0] != "data.txt" {
		t.Fatalf("expected ['data.txt'], got %v", resSearch)
	}
}

func TestFactory(t *testing.T) {
	// Test Cloud
	t.Setenv("OHC_MULTITENANT", "true")
	t.Setenv("OHC_STANDALONE", "false")
	provCloud, err := NewFileSystemProvider("/tmp")
	if err != nil {
		t.Fatalf("NewFileSystemProvider failed: %v", err)
	}
	if _, ok := provCloud.(*CloudFSProvider); !ok {
		t.Fatalf("expected *CloudFSProvider")
	}

	// Test Standalone
	t.Setenv("OHC_MULTITENANT", "false")
	t.Setenv("OHC_STANDALONE", "true")
	provLocal, err := NewFileSystemProvider("/tmp")
	if err != nil {
		t.Fatalf("NewFileSystemProvider failed: %v", err)
	}
	if _, ok := provLocal.(*LocalFSProvider); !ok {
		t.Fatalf("expected *LocalFSProvider")
	}

	// Test Default
	t.Setenv("OHC_MULTITENANT", "")
	t.Setenv("OHC_STANDALONE", "")
	provDefault, err := NewFileSystemProvider("/tmp")
	if err != nil {
		t.Fatalf("NewFileSystemProvider failed: %v", err)
	}
	if _, ok := provDefault.(*LocalFSProvider); !ok {
		t.Fatalf("expected *LocalFSProvider by default")
	}
}

func TestServerErrors(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "serverfs-err")
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

	// Missing args for read_file
	_, err = server.ExecuteTool(ctx, "read_file", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for read_file with missing args")
	}

	// Missing args for write_file
	_, err = server.ExecuteTool(ctx, "write_file", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for write_file with missing path")
	}
	_, err = server.ExecuteTool(ctx, "write_file", map[string]interface{}{"path": "test.txt"})
	if err == nil {
		t.Fatalf("expected error for write_file with missing data")
	}

	// Missing args for list_directory
	_, err = server.ExecuteTool(ctx, "list_directory", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for list_directory with missing path")
	}

	// Missing args for search_files
	_, err = server.ExecuteTool(ctx, "search_files", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for search_files with missing path")
	}
	_, err = server.ExecuteTool(ctx, "search_files", map[string]interface{}{"path": ""})
	if err == nil {
		t.Fatalf("expected error for search_files with missing pattern")
	}

	// Unknown tool
	_, err = server.ExecuteTool(ctx, "unknown_tool", map[string]interface{}{})
	if err == nil {
		t.Fatalf("expected error for unknown_tool")
	}
}

func TestLocalFSProviderErrors(t *testing.T) {

	// Create an unreadable directory
	tempDir, _ := os.MkdirTemp("", "unreadable")
	defer os.RemoveAll(tempDir)

	provider, _ := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	os.Chmod(tempDir, 0000)
	_, err := provider.ListDir(ctx, "")
	if err == nil {
		t.Fatalf("expected error when reading unreadable dir")
	}
	os.Chmod(tempDir, 0755)
}

func TestCloudFSProviderErrors(t *testing.T) {

	tempDir, _ := os.MkdirTemp("", "cloudfs-err")
	defer os.RemoveAll(tempDir)

	provider, _ := NewCloudFSProvider(tempDir)
	ctx := context.Background()

	// Use valid context
	claims := &auth.Claims{OrganizationID: "tenant-123"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Read non-existent
	_, err := provider.ReadFile(ctx, "does-not-exist.txt")
	if err == nil {
		t.Fatalf("expected error reading non-existent file")
	}

	// List non-existent
	_, err = provider.ListDir(ctx, "does-not-exist-dir")
	if err == nil {
		t.Fatalf("expected error listing non-existent directory")
	}
}

func TestServerListDirErr(t *testing.T) {
	tempDir, _ := os.MkdirTemp("", "unreadable")
	defer os.RemoveAll(tempDir)

	provider, _ := NewLocalFSProvider(tempDir)
	server := NewServer(provider)
	ctx := context.Background()

	os.Chmod(tempDir, 0000)

	_, err := server.ExecuteTool(ctx, "list_directory", map[string]interface{}{"path": ""})
	if err == nil {
		t.Fatalf("expected error for list_directory unreadable dir")
	}

	_, err = server.ExecuteTool(ctx, "search_files", map[string]interface{}{"path": "", "pattern": "a"})
	if err == nil {
		t.Fatalf("expected error for search_files unreadable dir")
	}

	os.Chmod(tempDir, 0755)
}

func TestServerReadFileErr(t *testing.T) {
	tempDir, _ := os.MkdirTemp("", "readerr")
	defer os.RemoveAll(tempDir)

	provider, _ := NewLocalFSProvider(tempDir)
	server := NewServer(provider)
	ctx := context.Background()

	_, err := server.ExecuteTool(ctx, "read_file", map[string]interface{}{"path": "does-not-exist.txt"})
	if err == nil {
		t.Fatalf("expected error for read_file does-not-exist")
	}
}

func TestServerWriteFileErr(t *testing.T) {
	tempDir, _ := os.MkdirTemp("", "writeerr")
	defer os.RemoveAll(tempDir)

	provider, _ := NewLocalFSProvider(tempDir)
	server := NewServer(provider)
	ctx := context.Background()

	os.Chmod(tempDir, 0000)

	_, err := server.ExecuteTool(ctx, "write_file", map[string]interface{}{"path": "test.txt", "data": "data"})
	if err == nil {
		t.Fatalf("expected error for write_file unreadable dir")
	}

	os.Chmod(tempDir, 0755)
}

func TestLocalFSErrors(t *testing.T) {
	tempDir, _ := os.MkdirTemp("", "writeerr")
	defer os.RemoveAll(tempDir)
	provider, _ := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Create an unreadable file to test ReadFile error
	provider.WriteFile(ctx, "unreadable.txt", []byte("bad"))
	os.Chmod(filepath.Join(tempDir, "unreadable.txt"), 0000)
	_, err := provider.ReadFile(ctx, "unreadable.txt")
	if err == nil {
		t.Fatalf("expected error when reading unreadable file")
	}

	// Try creating a file in unreadable directory
	os.Chmod(tempDir, 0000)
	err = provider.WriteFile(ctx, "newfile.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("expected error when writing to unreadable dir")
	}
	os.Chmod(tempDir, 0755)
}

func TestCloudFSErrors(t *testing.T) {
	tempDir, _ := os.MkdirTemp("", "writeerr")
	defer os.RemoveAll(tempDir)
	provider, _ := NewCloudFSProvider(tempDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant-123"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	// Create an unreadable file to test ReadFile error
	provider.WriteFile(ctx, "unreadable.txt", []byte("bad"))

	tenantDir := filepath.Join(tempDir, "tenant-123")
	os.Chmod(filepath.Join(tenantDir, "unreadable.txt"), 0000)
	_, err := provider.ReadFile(ctx, "unreadable.txt")
	if err == nil {
		t.Fatalf("expected error when reading unreadable file")
	}

	// Try creating a file in unreadable directory
	os.Chmod(tenantDir, 0000)
	err = provider.WriteFile(ctx, "newfile.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("expected error when writing to unreadable dir")
	}
	os.Chmod(tenantDir, 0755)
}

func TestWriteFileMkdirErr(t *testing.T) {
	tempDir, _ := os.MkdirTemp("", "writeerr2")
	defer os.RemoveAll(tempDir)

	// Make tempDir unwriteable to fail MkdirAll
	os.Chmod(tempDir, 0555)

	providerLocal, _ := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	err := providerLocal.WriteFile(ctx, "sub/file.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("expected error when MkdirAll fails in local provider")
	}

	providerCloud, _ := NewCloudFSProvider(tempDir)
	claims := &auth.Claims{OrganizationID: "tenant-123"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	err = providerCloud.WriteFile(ctx, "sub/file.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("expected error when MkdirAll fails in cloud provider")
	}

	os.Chmod(tempDir, 0755)
}

func TestCloudProviderContextErr(t *testing.T) {
	tempDir, _ := os.MkdirTemp("", "cloudctx")
	defer os.RemoveAll(tempDir)

	providerCloud, _ := NewCloudFSProvider(tempDir)
	ctx := context.Background()

	_, err := providerCloud.ReadFile(ctx, "file.txt")
	if err == nil {
		t.Fatalf("expected error when reading without claims")
	}

	err = providerCloud.WriteFile(ctx, "file.txt", []byte("bad"))
	if err == nil {
		t.Fatalf("expected error when writing without claims")
	}

	_, err = providerCloud.ListDir(ctx, "")
	if err == nil {
		t.Fatalf("expected error when listing without claims")
	}
}
