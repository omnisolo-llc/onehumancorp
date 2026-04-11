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
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("Expected 'hello', got '%s'", string(data))
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", files)
	}

	// Test path traversal
	_, err = provider.ReadFile(ctx, "../test.txt")
	if err == nil {
		t.Error("Expected path traversal error, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewCloudFSProvider(tempDir)

	// Create context with claims
	claims := &auth.Claims{OrganizationID: "tenant1"}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it was written to tenant dir
	b, _ := os.ReadFile(filepath.Join(tempDir, "tenant1", "test.txt"))
	if string(b) != "hello cloud" {
		t.Errorf("Expected 'hello cloud' in tenant dir, got '%s'", string(b))
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello cloud" {
		t.Errorf("Expected 'hello cloud', got '%s'", string(data))
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", files)
	}

	// Test without claims
	ctxNoAuth := context.Background()
	_, err = provider.ReadFile(ctxNoAuth, "test.txt")
	if err == nil {
		t.Error("Expected unauthorized error, got nil")
	}

	// Test path traversal
	_, err = provider.ReadFile(ctx, "../tenant2/test.txt")
	if err == nil {
		t.Error("Expected path traversal error, got nil")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "mcp")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider := NewLocalFSProvider(tempDir)
	mcpServer := NewHybridFSMCP(provider)
	ctx := context.Background()

	// List Tools
	tools := mcpServer.ListTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	// Call write_file
	_, err = mcpServer.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp data",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	// Call read_file
	res, err := mcpServer.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "mcp_test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resMap, ok := res.(map[string]interface{})
	if !ok || resMap["content"] != "mcp data" {
		t.Errorf("Expected 'mcp data', got %v", res)
	}

	// Call list_directory
	res, err = mcpServer.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	resMap, ok = res.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected map, got %v", res)
	}
	files, ok := resMap["files"].([]string)
	if !ok || len(files) != 1 || files[0] != "mcp_test.txt" {
		t.Errorf("Expected ['mcp_test.txt'], got %v", resMap["files"])
	}
}

func TestNewProviderFactory(t *testing.T) {
    os.Setenv("OHC_MULTITENANT", "true")
    providerCloud := NewProviderFactory("/tmp")
    if _, ok := providerCloud.(*CloudFSProvider); !ok {
        t.Error("Expected CloudFSProvider")
    }

    os.Setenv("OHC_MULTITENANT", "false")
    providerLocal := NewProviderFactory("/tmp")
    if _, ok := providerLocal.(*LocalFSProvider); !ok {
        t.Error("Expected LocalFSProvider")
    }
}
