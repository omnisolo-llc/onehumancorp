package hybridfsmcp

import (
	"context"
	"encoding/base64"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	_, err = provider.resolvePath("../../../etc/passwd")
	if err == nil {
		t.Fatal("Expected error for path traversal, got nil")
	}

	_, err = provider.resolvePath("/etc/passwd")
	if err == nil {
		t.Fatal("Expected error for absolute path traversal, got nil")
	}

	// Valid path
	_, err = provider.resolvePath("test.txt")
	if err != nil {
		t.Fatalf("Expected valid path to succeed, got %v", err)
	}
}

func TestCloudFSProvider_Isolation(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	ctx := context.Background()

	// Missing claims
	_, err = provider.ReadFile(ctx, nil, "test.txt")
	if err == nil || err.Error() != "unauthorized: missing claims" {
		t.Fatalf("Expected missing claims error, got %v", err)
	}

	// Missing org ID
	claimsEmptyOrg := &auth.Claims{}
	_, err = provider.ReadFile(ctx, claimsEmptyOrg, "test.txt")
	if err == nil || err.Error() != "unauthorized: missing organization ID" {
		t.Fatalf("Expected missing org ID error, got %v", err)
	}

	// Traversal attempt
	claimsOrg1 := &auth.Claims{OrganizationID: "org1"}
	_, err = provider.resolvePath(claimsOrg1, "../org2/data.txt")
	if err == nil {
		t.Fatal("Expected error for path traversal out of tenant dir, got nil")
	}

	// Valid tenant isolation (creates specific subfolder)
	_, err = provider.resolvePath(claimsOrg1, "test.txt")
	if err != nil {
		t.Fatalf("Expected valid tenant path to succeed, got %v", err)
	}

	stat, err := os.Stat(filepath.Join(tempDir, "org1"))
	if err != nil || !stat.IsDir() {
		t.Fatalf("Expected org1 directory to be created automatically")
	}
}

func TestCallTool_Routing(t *testing.T) {
	tempDir := t.TempDir()

	originalStandalone := os.Getenv("OHC_STANDALONE")
	os.Setenv("OHC_STANDALONE", "true")
	defer func() {
		if originalStandalone == "" {
			os.Unsetenv("OHC_STANDALONE")
		} else {
			os.Setenv("OHC_STANDALONE", originalStandalone)
		}
	}()

	mcp, err := NewHybridFSMCP(tempDir)
	if err != nil {
		t.Fatalf("Failed to create MCP: %v", err)
	}

	ctx := context.Background()

	// 1. Write file
	content := []byte("hello mcp")
	b64Content := base64.StdEncoding.EncodeToString(content)

	res, err := mcp.CallTool(ctx, "write_file", map[string]interface{}{
		"path": "greeting.txt",
		"content_base64": b64Content,
	})
	if err != nil {
		t.Fatalf("CallTool write_file failed: %v", err)
	}

	resMap, ok := res.(map[string]interface{})
	if !ok || resMap["status"] != "success" {
		t.Fatalf("Expected status success, got %v", res)
	}

	// 2. Read file
	resRead, err := mcp.CallTool(ctx, "read_file", map[string]interface{}{
		"path": "greeting.txt",
	})
	if err != nil {
		t.Fatalf("CallTool read_file failed: %v", err)
	}
	resReadMap, ok := resRead.(map[string]interface{})
	if !ok || resReadMap["content_base64"] != b64Content {
		t.Fatalf("Expected to read back content %v, got %v", b64Content, resRead)
	}

	// 3. List directory
	resList, err := mcp.CallTool(ctx, "list_directory", map[string]interface{}{
		"path": ".",
	})
	if err != nil {
		t.Fatalf("CallTool list_directory failed: %v", err)
	}
	resListMap, ok := resList.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected list map, got %v", resList)
	}
	entries, ok := resListMap["entries"].([]string)
	if !ok || len(entries) != 1 || entries[0] != "greeting.txt" {
		t.Fatalf("Expected to list ['greeting.txt'], got %v", entries)
	}
}
