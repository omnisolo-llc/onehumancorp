package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := &LocalFSProvider{WorkspaceDir: tempDir}
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	// Test WriteFile
	testContent := []byte("hello local")
	err = provider.WriteFile(ctx, claims, "test.txt", testContent)
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	readContent, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(readContent) != string(testContent) {
		t.Errorf("Expected content %s, got %s", testContent, readContent)
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, claims, "")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("ListDir returned unexpected result: %v", infos)
	}

	// Test SearchFiles
	results, err := provider.SearchFiles(ctx, claims, "test")
	if err != nil {
		t.Errorf("SearchFiles failed: %v", err)
	}
	if len(results) != 1 || results[0] != "test.txt" {
		t.Errorf("SearchFiles returned unexpected result: %v", results)
	}

	// Test Path Escape
	err = provider.WriteFile(ctx, claims, "../escape.txt", testContent)
	if err == nil {
		t.Errorf("Expected error when escaping path, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := &CloudFSProvider{BaseCloudDir: tempDir}
	ctx := context.Background()
	claims := &Claims{OrganizationID: "org-123"}

	// Test WriteFile
	testContent := []byte("hello cloud")
	err = provider.WriteFile(ctx, claims, "test.txt", testContent)
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Verify it was written to the tenant directory
	tenantDir := filepath.Join(tempDir, "org-123")
	if _, err := os.Stat(filepath.Join(tenantDir, "test.txt")); os.IsNotExist(err) {
		t.Errorf("File not written to tenant directory: %v", err)
	}

	// Test ReadFile
	readContent, err := provider.ReadFile(ctx, claims, "test.txt")
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(readContent) != string(testContent) {
		t.Errorf("Expected content %s, got %s", testContent, readContent)
	}

	// Test ListDir
	infos, err := provider.ListDir(ctx, claims, "")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}
	if len(infos) != 1 || infos[0].Name() != "test.txt" {
		t.Errorf("ListDir returned unexpected result: %v", infos)
	}

	// Test SearchFiles
	results, err := provider.SearchFiles(ctx, claims, "test")
	if err != nil {
		t.Errorf("SearchFiles failed: %v", err)
	}
	if len(results) != 1 || results[0] != "test.txt" {
		t.Errorf("SearchFiles returned unexpected result: %v", results)
	}

	// Test Cross-Tenant Access
	otherClaims := &Claims{OrganizationID: "org-456"}
	_, err = provider.ReadFile(ctx, otherClaims, "test.txt")
	if err == nil {
		t.Errorf("Expected error when reading cross-tenant file, got nil")
	}

	// Test Missing Claims
	err = provider.WriteFile(ctx, nil, "test.txt", testContent)
	if err == nil {
		t.Errorf("Expected error with nil claims, got nil")
	}

	// Test Path Escape
	err = provider.WriteFile(ctx, claims, "../escape.txt", testContent)
	if err == nil {
		t.Errorf("Expected error when escaping path, got nil")
	}
}

func TestPrefixLeakVulnerability(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatalf("Failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tempDir)

	provider := &CloudFSProvider{BaseCloudDir: tempDir}
	ctx := context.Background()

	// Tenant 1
	claims1 := &Claims{OrganizationID: "org-1"}
	testContent1 := []byte("hello cloud 1")
	err = provider.WriteFile(ctx, claims1, "secret.txt", testContent1)
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Tenant 12 (trying to hack Tenant 1)
	claims12 := &Claims{OrganizationID: "org-12"}
	testContent12 := []byte("hello cloud 12")
	err = provider.WriteFile(ctx, claims12, "test.txt", testContent12)
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	// Try reading Tenant 12's files from Tenant 1 (using ../org-12 path)
	_, err = provider.ReadFile(ctx, claims1, "../org-12/test.txt")
	if err == nil {
		t.Errorf("Expected error reading prefix matched tenant directory, but read succeeded.")
	}
}
