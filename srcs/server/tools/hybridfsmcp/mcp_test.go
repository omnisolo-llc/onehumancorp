package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create local fs provider: %v", err)
	}

	ctx := context.Background()

	// Test WriteFile
	err = provider.WriteFile(ctx, "test.txt", []byte("hello world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello world" {
		t.Fatalf("Expected 'hello world', got '%s'", string(data))
	}

	// Test Boundary Check
	_, err = provider.ReadFile(ctx, "../../../../../etc/passwd")
	if err == nil {
		t.Fatalf("Expected boundary check to fail, but it succeeded")
	}

	// Test ListDir
	files, err := provider.ListDir(ctx, "")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(files) != 1 || files[0].Name != "test.txt" {
		t.Fatalf("Expected 1 file 'test.txt', got %v", files)
	}
}

func TestCloudFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	baseProvider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("Failed to create base provider: %v", err)
	}

	cloudProvider := NewCloudFSProvider(baseProvider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-123",
	})

	// Test WriteFile
	err = cloudProvider.WriteFile(ctx, "data.txt", []byte("tenant data"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it was written to tenant-123/data.txt in the base directory
	baseData, err := baseProvider.ReadFile(context.Background(), "tenant-123/data.txt")
	if err != nil {
		t.Fatalf("Failed to verify base data: %v", err)
	}
	if string(baseData) != "tenant data" {
		t.Fatalf("Expected 'tenant data', got '%s'", string(baseData))
	}

	// Test missing claims
	_, err = cloudProvider.ReadFile(context.Background(), "data.txt")
	if err == nil {
		t.Fatalf("Expected error for missing claims, but succeeded")
	}
}

func TestHybridFSMCP(t *testing.T) {
	tmpLocalDir, _ := os.MkdirTemp("", "mcp_local")
	defer os.RemoveAll(tmpLocalDir)
	tmpCloudDir, _ := os.MkdirTemp("", "mcp_cloud")
	defer os.RemoveAll(tmpCloudDir)

	localProvider, _ := NewLocalFSProvider(tmpLocalDir)
	cloudBaseProvider, _ := NewLocalFSProvider(tmpCloudDir)
	cloudProvider := NewCloudFSProvider(cloudBaseProvider)

	// Test Standalone Mode
	mcpStandalone := NewHybridFSMCP(true, localProvider, cloudProvider)
	ctx := context.Background()

	_, err := mcpStandalone.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "standalone.txt",
		"content": "standalone data",
	})
	if err != nil {
		t.Fatalf("CallTool write_file standalone failed: %v", err)
	}

	res, err := mcpStandalone.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "standalone.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file standalone failed: %v", err)
	}
	if res.(map[string]interface{})["content"] != "standalone data" {
		t.Fatalf("Expected 'standalone data', got %v", res)
	}

	// Test Cloud Mode
	mcpCloud := NewHybridFSMCP(false, localProvider, cloudProvider)

	// Should fail without claims
	_, err = mcpCloud.CallTool(ctx, "write_file", map[string]interface{}{
		"path":    "cloud.txt",
		"content": "cloud data",
	})
	if err == nil {
		t.Fatalf("Expected error without claims in cloud mode")
	}

	// Should succeed with claims
	ctxWithClaims := context.WithValue(ctx, auth.ClaimsContextKeyForTest, &auth.Claims{
		OrganizationID: "tenant-abc",
	})

	_, err = mcpCloud.CallTool(ctxWithClaims, "write_file", map[string]interface{}{
		"path":    "cloud.txt",
		"content": "cloud data",
	})
	if err != nil {
		t.Fatalf("CallTool write_file cloud failed: %v", err)
	}

	res, err = mcpCloud.CallTool(ctxWithClaims, "read_file", map[string]interface{}{
		"path": "cloud.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file cloud failed: %v", err)
	}
	if res.(map[string]interface{})["content"] != "cloud data" {
		t.Fatalf("Expected 'cloud data', got %v", res)
	}
}
