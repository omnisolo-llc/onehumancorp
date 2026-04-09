package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/agents/mcp"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHybridFSMCP_Local(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "hybrid_fs_local_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, _ := mcp.NewLocalFSProvider(tmpDir)
	server := NewHybridFSMCP(provider)

	tools := server.ListTools()
	if len(tools) != 3 {
		t.Fatalf("Expected 3 tools, got %d", len(tools))
	}

	ctx := context.Background()

	// Test write_file
	testData := []byte("hello world")
	encodedData := base64.StdEncoding.EncodeToString(testData)

	resWrite, err := server.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"data": encodedData,
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	writeRes := resWrite.(map[string]interface{})
	if writeRes["status"] != "success" || writeRes["mode"] != "standalone" {
		t.Errorf("Unexpected write response: %+v", writeRes)
	}

	// Test read_file
	resRead, err := server.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}

	readRes := resRead.(map[string]interface{})
	if readRes["status"] != "success" || readRes["mode"] != "standalone" || readRes["data"] != encodedData {
		t.Errorf("Unexpected read response: %+v", readRes)
	}

	// Test list_directory
	resList, err := server.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}

	listRes := resList.(map[string]interface{})
	if listRes["status"] != "success" || listRes["mode"] != "standalone" {
		t.Errorf("Unexpected list response: %+v", listRes)
	}
	entries := listRes["entries"].([]mcp.FileInfo)
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Errorf("Unexpected entries: %+v", entries)
	}
}

func TestHybridFSMCP_Cloud(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "hybrid_fs_cloud_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, _ := mcp.NewCloudFSProvider(tmpDir)
	server := NewHybridFSMCP(provider)

	claims := &auth.Claims{OrganizationID: "tenant-1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test write_file
	testData := []byte("cloud data")
	encodedData := base64.StdEncoding.EncodeToString(testData)

	resWrite, err := server.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"data": encodedData,
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	writeRes := resWrite.(map[string]interface{})
	if writeRes["status"] != "success" || writeRes["mode"] != "cloud" {
		t.Errorf("Unexpected write response: %+v", writeRes)
	}
}

func TestFactory(t *testing.T) {
	// Test standalone (default)
	os.Unsetenv("OHC_MULTITENANT")
	provider, err := NewHybridFSProvider()
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}
	if _, ok := provider.(*mcp.LocalFSProvider); !ok {
		t.Errorf("Expected LocalFSProvider")
	}

	// Test cloud
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")
	provider, err = NewHybridFSProvider()
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}
	if _, ok := provider.(*mcp.CloudFSProvider); !ok {
		t.Errorf("Expected CloudFSProvider")
	}
}
