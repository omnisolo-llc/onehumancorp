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
	tempDir, err := os.MkdirTemp("", "localfs_test_*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}

	ctx := context.Background()

	// Test Write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello local"))
	if err != nil {
		t.Errorf("expected no error writing file, got %v", err)
	}

	// Test Read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("expected no error reading file, got %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("expected 'hello local', got %s", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("expected no error listing dir, got %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	// Test Bounding
	_, err = provider.ReadFile(ctx, "../outside.txt")
	if err == nil {
		t.Errorf("expected error when path escapes workspace")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test_*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-1",
	})

	// Test Write
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Errorf("expected no error writing file, got %v", err)
	}

	// Test Read
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Errorf("expected no error reading file, got %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("expected 'hello cloud', got %s", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Errorf("expected no error listing dir, got %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}

	// Check underlying file structure to ensure isolation
	expectedPath := filepath.Join(tempDir, "tenant-1", "test.txt")
	if _, err := os.Stat(expectedPath); os.IsNotExist(err) {
		t.Errorf("expected file to exist at tenant path: %s", expectedPath)
	}

	// Test Bounding
	_, err = provider.ReadFile(ctx, "../tenant-2/test.txt")
	if err == nil {
		t.Errorf("expected error when path escapes tenant workspace")
	}

	// Test Missing Claims
	badCtx := context.Background()
	_, err = provider.ReadFile(badCtx, "test.txt")
	if err == nil {
		t.Errorf("expected error when context missing tenant claims")
	}
}

func TestFactory(t *testing.T) {
	// Standalone mode
	os.Setenv("OHC_STANDALONE", "true")
	p1, _ := NewFileSystemProvider(".", ".")
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Errorf("expected LocalFSProvider when OHC_STANDALONE is true")
	}

	// Cloud mode
	os.Setenv("OHC_STANDALONE", "false")
	p2, _ := NewFileSystemProvider(".", ".")
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Errorf("expected CloudFSProvider when OHC_STANDALONE is false")
	}
}

func TestMCPServer(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcpserver_test_*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, _ := NewLocalFSProvider(tempDir)
	tools := GetFileSystemTools(provider)

	ctx := context.Background()

	// Find tools
	var writeTool, readTool, listTool func(context.Context, json.RawMessage) (string, error)
	for _, tool := range tools {
		switch tool.Name {
		case "write_file":
			writeTool = tool.Execute
		case "read_file":
			readTool = tool.Execute
		case "list_directory":
			listTool = tool.Execute
		}
	}

	// Write
	writeArgs := json.RawMessage(`{"path": "mcp.txt", "content": "mcp test"}`)
	res, err := writeTool(ctx, writeArgs)
	if err != nil {
		t.Errorf("writeTool err: %v", err)
	}
	if res != "File written successfully." {
		t.Errorf("unexpected write result: %s", res)
	}

	// Read
	readArgs := json.RawMessage(`{"path": "mcp.txt"}`)
	res, err = readTool(ctx, readArgs)
	if err != nil {
		t.Errorf("readTool err: %v", err)
	}
	if res != "mcp test" {
		t.Errorf("unexpected read result: %s", res)
	}

	// List
	listArgs := json.RawMessage(`{"path": "."}`)
	res, err = listTool(ctx, listArgs)
	if err != nil {
		t.Errorf("listTool err: %v", err)
	}
	if res != "mcp.txt" {
		t.Errorf("unexpected list result: %s", res)
	}
}

func TestErrorsAndCoverage(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "error_coverage_*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}
	ctx := context.Background()

	// Test ReadFile error propagation
	_, err = provider.ReadFile(ctx, "does_not_exist.txt")
	if err == nil {
		t.Errorf("expected error reading nonexistent file")
	}

	// Test WriteFile path error propagation
	err = provider.WriteFile(ctx, "../outside.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error writing outside workspace")
	}

	// Test ListDir error propagation
	_, err = provider.ListDir(ctx, "../outside")
	if err == nil {
		t.Errorf("expected error listing outside workspace")
	}

	_, err = provider.ListDir(ctx, "does_not_exist_dir")
	if err == nil {
		t.Errorf("expected error listing nonexistent dir")
	}

	// Cloud errors
	cloudProvider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}
	cloudCtx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-3",
	})

	_, err = cloudProvider.ReadFile(cloudCtx, "does_not_exist.txt")
	if err == nil {
		t.Errorf("expected error reading nonexistent file cloud")
	}

	err = cloudProvider.WriteFile(cloudCtx, "../outside.txt", []byte("bad"))
	if err == nil {
		t.Errorf("expected error writing outside tenant workspace")
	}

	_, err = cloudProvider.ListDir(cloudCtx, "../outside")
	if err == nil {
		t.Errorf("expected error listing outside tenant workspace")
	}

	_, err = cloudProvider.ListDir(cloudCtx, "does_not_exist_dir")
	if err == nil {
		t.Errorf("expected error listing nonexistent dir cloud")
	}

}

func TestMCPJSONErrors(t *testing.T) {
    provider, _ := NewLocalFSProvider(".")
	tools := GetFileSystemTools(provider)

	ctx := context.Background()

	var writeTool, readTool, listTool func(context.Context, json.RawMessage) (string, error)
	for _, tool := range tools {
		switch tool.Name {
		case "write_file":
			writeTool = tool.Execute
		case "read_file":
			readTool = tool.Execute
		case "list_directory":
			listTool = tool.Execute
		}
	}

    badArgs := json.RawMessage(`{bad json}`)
    _, err := writeTool(ctx, badArgs)
    if err == nil {
        t.Errorf("expected error for bad write json")
    }

    _, err = readTool(ctx, badArgs)
    if err == nil {
        t.Errorf("expected error for bad read json")
    }

    _, err = listTool(ctx, badArgs)
    if err == nil {
        t.Errorf("expected error for bad list json")
    }
}
