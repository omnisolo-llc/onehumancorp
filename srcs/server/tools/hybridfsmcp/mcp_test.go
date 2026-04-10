package hybridfsmcp

import (
	"context"
	"os"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "hybridfs-test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	provider := NewLocalFSProvider(tmpDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "test-org"}

	// Test WriteFile
	err = provider.WriteFile(ctx, claims, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Errorf("Expected 'hello local', got '%s'", string(data))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected ['test.txt'], got %v", entries)
	}

	// Test Directory Traversal Prevention
	err = provider.WriteFile(ctx, claims, "../escaped.txt", []byte("danger"))
	if err == nil {
		t.Error("Expected error for directory traversal, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	provider := NewCloudFSProvider()
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "test-org"}

	// Test WriteFile
	err := provider.WriteFile(ctx, claims, "config/settings.json", []byte("{}"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	data, err := provider.ReadFile(ctx, claims, "config/settings.json")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "{}" {
		t.Errorf("Expected '{}', got '%s'", string(data))
	}

	// Test ListDir
	err = provider.WriteFile(ctx, claims, "config/other.json", []byte("[]"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	entries, err := provider.ListDir(ctx, claims, "config")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 2 {
		t.Errorf("Expected 2 entries, got %d", len(entries))
	}

	// Test Missing Claims
	err = provider.WriteFile(ctx, nil, "fail.txt", []byte("fail"))
	if err == nil {
		t.Error("Expected error for missing claims, got nil")
	}
}

func TestHybridFSMCP(t *testing.T) {
	provider := NewCloudFSProvider()
	mcp := NewHybridFSMCP(provider)

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-123"})

	tools := mcp.ListTools()
	if len(tools) != 3 {
		t.Errorf("Expected 3 tools, got %d", len(tools))
	}

	_, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "test.txt",
		"data": "hello mcp",
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	res, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "test.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}

	resMap := res.(map[string]interface{})
	if resMap["data"] != "hello mcp" {
		t.Errorf("Expected 'hello mcp', got '%v'", resMap["data"])
	}

	_, err = mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": "/",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
}

func TestFactory(t *testing.T) {
	os.Setenv("OHC_STANDALONE", "true")
	p1 := NewProviderFromEnv()
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Error("Expected LocalFSProvider when OHC_STANDALONE=true")
	}

	os.Setenv("OHC_STANDALONE", "false")
	p2 := NewProviderFromEnv()
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Error("Expected CloudFSProvider when OHC_STANDALONE=false")
	}
}
