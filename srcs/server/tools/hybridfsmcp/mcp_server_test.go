package hybridfsmcp

import (
	"context"
	"encoding/json"
	"github.com/onehumancorp/mono/srcs/server/auth"
	"os"
	"path/filepath"
	"testing"
)

type MockFSProvider struct {
	ReadFileFunc    func(ctx context.Context, path string) ([]byte, error)
	WriteFileFunc   func(ctx context.Context, path string, data []byte) error
	ListDirFunc     func(ctx context.Context, path string) ([]os.DirEntry, error)
	SearchFilesFunc func(ctx context.Context, path, pattern string) ([]string, error)
}

func (m *MockFSProvider) ReadFile(ctx context.Context, path string) ([]byte, error) {
	if m.ReadFileFunc != nil {
		return m.ReadFileFunc(ctx, path)
	}
	return nil, nil
}

func (m *MockFSProvider) WriteFile(ctx context.Context, path string, data []byte) error {
	if m.WriteFileFunc != nil {
		return m.WriteFileFunc(ctx, path, data)
	}
	return nil
}

func (m *MockFSProvider) ListDir(ctx context.Context, path string) ([]os.DirEntry, error) {
	if m.ListDirFunc != nil {
		return m.ListDirFunc(ctx, path)
	}
	return nil, nil
}

func (m *MockFSProvider) SearchFiles(ctx context.Context, path, pattern string) ([]string, error) {
	if m.SearchFilesFunc != nil {
		return m.SearchFilesFunc(ctx, path, pattern)
	}
	return nil, nil
}

func TestFSMCPServer(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	server := NewFSMCPServer(provider)
	ctx := context.Background()

	// write_file
	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "test.txt", Data: "mcp test"})
	res, err := server.ExecuteTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("ExecuteTool write_file failed: %v", err)
	}
	if res.Status != "success" {
		t.Errorf("Expected success, got %s: %s", res.Status, string(res.ResultData))
	}

	// read_file
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "test.txt"})
	res, err = server.ExecuteTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("ExecuteTool read_file failed: %v", err)
	}
	if res.Status != "success" {
		t.Errorf("Expected success, got %s: %s", res.Status, string(res.ResultData))
	}
	var readResult map[string]string
	json.Unmarshal(res.ResultData, &readResult)
	if readResult["content"] != "mcp test" {
		t.Errorf("Expected 'mcp test', got '%s'", readResult["content"])
	}

	// list_directory
	listArgs, _ := json.Marshal(ListDirArgs{Path: "."})
	res, err = server.ExecuteTool(ctx, "list_directory", listArgs)
	if err != nil {
		t.Fatalf("ExecuteTool list_directory failed: %v", err)
	}
	if res.Status != "success" {
		t.Errorf("Expected success, got %s: %s", res.Status, string(res.ResultData))
	}
	var listResult map[string][]string
	json.Unmarshal(res.ResultData, &listResult)
	if len(listResult["entries"]) != 1 || listResult["entries"][0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", listResult["entries"])
	}

	// unknown tool
	_, err = server.ExecuteTool(ctx, "unknown_tool", []byte(`{}`))
	if err == nil {
		t.Errorf("Expected error for unknown tool")
	}
}

func TestFSMCPServer_Errors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	server := NewFSMCPServer(provider)
	ctx := context.Background()

	// read non-existent
	readArgs, _ := json.Marshal(ReadFileArgs{Path: "missing.txt"})
	res, _ := server.ExecuteTool(ctx, "read_file", readArgs)
	if res.Status != "error" {
		t.Errorf("Expected error status for missing file, got %s", res.Status)
	}
	if !res.Escalation {
		t.Errorf("Expected escalation for error")
	}
}

func TestFSMCPServer_SearchFiles(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	server := NewFSMCPServer(provider)
	ctx := context.Background()

	// write files for search
	provider.WriteFile(ctx, "test_search1.txt", []byte("mcp test1"))
	provider.WriteFile(ctx, "other_file.txt", []byte("mcp test2"))
	provider.WriteFile(ctx, "sub/test_search2.txt", []byte("mcp test3"))

	// search_files
	searchArgs, _ := json.Marshal(SearchFilesArgs{Path: ".", Pattern: "search"})
	res, err := server.ExecuteTool(ctx, "search_files", searchArgs)
	if err != nil {
		t.Fatalf("ExecuteTool search_files failed: %v", err)
	}
	if res.Status != "success" {
		t.Errorf("Expected success, got %s: %s", res.Status, string(res.ResultData))
	}
	var searchResult map[string][]string
	json.Unmarshal(res.ResultData, &searchResult)
	if len(searchResult["matches"]) != 2 {
		t.Errorf("Expected 2 matches, got %v", searchResult["matches"])
	}
}

func TestFSMCPServer_InvalidJSON(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	server := NewFSMCPServer(provider)
	ctx := context.Background()

	// invalid json for read_file
	_, err := server.ExecuteTool(ctx, "read_file", []byte(`{invalid`))
	if err == nil {
		t.Errorf("Expected error for invalid read_file args")
	}

	// invalid json for write_file
	_, err = server.ExecuteTool(ctx, "write_file", []byte(`{invalid`))
	if err == nil {
		t.Errorf("Expected error for invalid write_file args")
	}

	// invalid json for list_directory
	_, err = server.ExecuteTool(ctx, "list_directory", []byte(`{invalid`))
	if err == nil {
		t.Errorf("Expected error for invalid list_directory args")
	}

	// invalid json for search_files
	_, err = server.ExecuteTool(ctx, "search_files", []byte(`{invalid`))
	if err == nil {
		t.Errorf("Expected error for invalid search_files args")
	}
}

func TestFSMCPServer_ProviderErrors(t *testing.T) {
	mockProvider := &MockFSProvider{
		WriteFileFunc: func(ctx context.Context, path string, data []byte) error {
			return os.ErrPermission
		},
	}
	server := NewFSMCPServer(mockProvider)
	ctx := context.Background()

	writeArgs, _ := json.Marshal(WriteFileArgs{Path: "test.txt", Data: "test"})
	res, _ := server.ExecuteTool(ctx, "write_file", writeArgs)
	if res.Status != "error" {
		t.Errorf("Expected error status, got %s", res.Status)
	}
}

func TestFSMCPServer_ProviderErrorsMore(t *testing.T) {
	mockProvider := &MockFSProvider{
		ListDirFunc: func(ctx context.Context, path string) ([]os.DirEntry, error) {
			return nil, os.ErrPermission
		},
		SearchFilesFunc: func(ctx context.Context, path, pattern string) ([]string, error) {
			return nil, os.ErrPermission
		},
	}
	server := NewFSMCPServer(mockProvider)
	ctx := context.Background()

	listArgs, _ := json.Marshal(ListDirArgs{Path: "."})
	res, _ := server.ExecuteTool(ctx, "list_directory", listArgs)
	if res.Status != "error" {
		t.Errorf("Expected error status for list_directory, got %s", res.Status)
	}

	searchArgs, _ := json.Marshal(SearchFilesArgs{Path: ".", Pattern: "test"})
	res, _ = server.ExecuteTool(ctx, "search_files", searchArgs)
	if res.Status != "error" {
		t.Errorf("Expected error status for search_files, got %s", res.Status)
	}
}

func TestWriteFileErrors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	err := provider.WriteFile(ctx, "../outside.txt", []byte("test"))
	if err == nil {
		t.Errorf("Expected error for writing outside workspace")
	}

	_, err = provider.ListDir(ctx, "../outside")
	if err == nil {
		t.Errorf("Expected error for listing outside workspace")
	}

	_, err = provider.SearchFiles(ctx, "../outside", "test")
	if err == nil {
		t.Errorf("Expected error for searching outside workspace")
	}
}

func TestCloudProviderErrors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)

	claims := &auth.Claims{
		OrganizationID: "tenant-123",
	}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	err := provider.WriteFile(ctx, "../outside.txt", []byte("test"))
	if err == nil {
		t.Errorf("Expected error for writing outside tenant path")
	}

	_, err = provider.ListDir(ctx, "../outside")
	if err == nil {
		t.Errorf("Expected error for listing outside tenant path")
	}

	_, err = provider.SearchFiles(ctx, "../outside", "test")
	if err == nil {
		t.Errorf("Expected error for searching outside tenant path")
	}

	// Test Absolute Path
	err = provider.WriteFile(ctx, "/etc/passwd", []byte("test"))
	if err == nil {
		t.Errorf("Expected error for absolute path")
	}

	_, err = provider.resolvePath(ctx, "/etc/passwd")
	if err == nil {
		t.Errorf("Expected error for absolute path in resolvePath")
	}
}

func TestMkdirErrors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Create a file where a directory needs to go
	os.WriteFile(filepath.Join(tempDir, "blocker"), []byte("file"), 0644)

	err := provider.WriteFile(ctx, "blocker/file.txt", []byte("test"))
	if err == nil {
		t.Errorf("Expected error when mkdir fails")
	}

	cloudProvider := NewCloudFSProvider(tempDir)
	claims := &auth.Claims{OrganizationID: "tenant-123"}
	cloudCtx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	os.Mkdir(filepath.Join(tempDir, "tenant-123"), 0755)
	os.WriteFile(filepath.Join(tempDir, "tenant-123", "blocker"), []byte("file"), 0644)

	err = cloudProvider.WriteFile(cloudCtx, "blocker/file.txt", []byte("test"))
	if err == nil {
		t.Errorf("Expected error when mkdir fails in cloud provider")
	}
}

func TestSearchFilesWalkErrors(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	os.Mkdir(filepath.Join(tempDir, "bad_dir"), 0000)
	defer os.Chmod(filepath.Join(tempDir, "bad_dir"), 0755)

	_, err := provider.SearchFiles(ctx, ".", "test")
	if err == nil {
		t.Errorf("Expected error from WalkDir due to permissions")
	}

	cloudProvider := NewCloudFSProvider(tempDir)
	claims := &auth.Claims{OrganizationID: "tenant-123"}
	cloudCtx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	os.Mkdir(filepath.Join(tempDir, "tenant-123"), 0755)
	os.Mkdir(filepath.Join(tempDir, "tenant-123", "bad_dir"), 0000)
	defer os.Chmod(filepath.Join(tempDir, "tenant-123", "bad_dir"), 0755)

	_, err = cloudProvider.SearchFiles(cloudCtx, ".", "test")
	if err == nil {
		t.Errorf("Expected error from WalkDir due to permissions in cloud mode")
	}
}
