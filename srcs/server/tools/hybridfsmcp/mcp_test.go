package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_PathTraversal(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create local provider: %v", err)
	}

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "test-org"}

	// Attempt to escape the directory
	escapedPath := "../escaped.txt"
	err = provider.WriteFile(ctx, claims, escapedPath, []byte("secret"))
	if err == nil {
		t.Fatal("expected error when path escapes base directory, got nil")
	}
}

func TestLocalFSProvider_ReadWriteListSearch(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create local provider: %v", err)
	}

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "test-org"}

	// Write
	err = provider.WriteFile(ctx, claims, "test.txt", []byte("hello local"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read
	data, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "hello local" {
		t.Fatalf("expected 'hello local', got %s", string(data))
	}

	// List
	entries, err := provider.ListDir(ctx, claims, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name != "test.txt" {
		t.Fatalf("unexpected entries: %+v", entries)
	}

	// Search
	matches, err := provider.SearchFiles(ctx, claims, ".", "*.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(matches) != 1 || matches[0] != "test.txt" {
		t.Fatalf("unexpected matches: %+v", matches)
	}
}

func TestCloudFSProvider_TenantIsolation(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create cloud provider: %v", err)
	}

	ctx := context.Background()
	claimsOrg1 := &auth.Claims{OrganizationID: "org-1"}
	claimsOrg2 := &auth.Claims{OrganizationID: "org-2"}

	// Write file for org-1
	err = provider.WriteFile(ctx, claimsOrg1, "data.txt", []byte("org 1 data"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Read file for org-1
	data, err := provider.ReadFile(ctx, claimsOrg1, "data.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != "org 1 data" {
		t.Fatalf("expected 'org 1 data', got %s", string(data))
	}

	// Org-2 attempts to read org-1's file via traversal
	err = provider.WriteFile(ctx, claimsOrg2, "../org-1/data.txt", []byte("hacked"))
	if err == nil {
		t.Fatal("expected traversal error when escaping tenant directory")
	}

	// Search for org-2 should not see org-1 files
	matches, err := provider.SearchFiles(ctx, claimsOrg2, ".", "*.txt")
	if err != nil {
		t.Fatalf("SearchFiles failed: %v", err)
	}
	if len(matches) != 0 {
		t.Fatalf("expected 0 matches for org-2, got %d", len(matches))
	}
}

func TestCloudFSProvider_MissingClaims(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create cloud provider: %v", err)
	}

	ctx := context.Background()
	err = provider.WriteFile(ctx, nil, "data.txt", []byte("data"))
	if err == nil {
		t.Fatal("expected error for missing claims")
	}
}

func TestHybridFSMCP_CallTool(t *testing.T) {
	tempDir := t.TempDir()
	provider, _ := NewLocalFSProvider(tempDir)
	mcpServer := NewHybridFSMCP(provider)

	ctx := context.Background()
	// write_file
	writeArgs := map[string]interface{}{
		"path":    "mcp_test.txt",
		"content": "mcp data",
	}
	res, err := mcpServer.CallTool(ctx, "write_file", writeArgs)
	if err != nil {
		t.Fatalf("write_file tool failed: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); ok {
		if resMap["status"] != "success" {
			t.Fatalf("expected success, got %v", resMap["status"])
		}
	}

	// read_file
	readArgs := map[string]interface{}{
		"path": "mcp_test.txt",
	}
	res, err = mcpServer.CallTool(ctx, "read_file", readArgs)
	if err != nil {
		t.Fatalf("read_file tool failed: %v", err)
	}
	if resMap, ok := res.(map[string]interface{}); ok {
		if resMap["content"] != "mcp data" {
			t.Fatalf("expected 'mcp data', got %v", resMap["content"])
		}
	}
}

func TestFactory_Standalone(t *testing.T) {
	tempDir := t.TempDir()
	t.Setenv("OHC_STANDALONE", "true")
	provider, err := NewProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}
	if !provider.IsLocal() {
		t.Fatal("expected local provider when OHC_STANDALONE is true")
	}
}

func TestFactory_Cloud(t *testing.T) {
	tempDir := t.TempDir()
	t.Setenv("OHC_STANDALONE", "false")
	provider, err := NewProvider(tempDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}
	if provider.IsLocal() {
		t.Fatal("expected cloud provider when OHC_STANDALONE is false")
	}
}

// Partial match vulnerability check
func TestProvider_PartialPathMatchVulnerability(t *testing.T) {
	// Create two sibling directories: "tenant" and "tenant_hacked"
	baseDir := t.TempDir()
	tenantDir := filepath.Join(baseDir, "tenant")
	hackedDir := filepath.Join(baseDir, "tenant_hacked")

	_ = os.MkdirAll(tenantDir, 0755)
	_ = os.MkdirAll(hackedDir, 0755)

	// Write a file in hackedDir
	_ = os.WriteFile(filepath.Join(hackedDir, "secret.txt"), []byte("hacked secret"), 0644)

	// Create cloud provider pointing to baseDir
	provider, err := NewCloudFSProvider(baseDir)
	if err != nil {
		t.Fatalf("failed to create cloud provider: %v", err)
	}

	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "tenant"}

	// Attempt to read from "tenant_hacked" using traversal
	// tenantBase is baseDir/tenant.
	// We ask for "../tenant_hacked/secret.txt"
	_, err = provider.ReadFile(ctx, claims, "../tenant_hacked/secret.txt")
	if err == nil {
		t.Fatal("expected error when trying partial directory name match traversal")
	}
}
