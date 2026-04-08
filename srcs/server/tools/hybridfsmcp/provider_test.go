package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "localfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}

	ctx := context.Background()

	// Test write and read
	testPath := "test.txt"
	testContent := []byte("hello local")
	err = provider.WriteFile(ctx, testPath, testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	data, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("Expected %s, got %s", testContent, data)
	}

	// Test list dir
	subDir := "subdir"
	err = provider.WriteFile(ctx, filepath.Join(subDir, "subfile.txt"), testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}

	foundTest := false
	foundSubDir := false
	for _, e := range entries {
		if e == "test.txt" {
			foundTest = true
		}
		if strings.HasPrefix(e, "subdir") {
			foundSubDir = true
		}
	}
	if !foundTest || !foundSubDir {
		t.Errorf("ListDir did not find expected entries: %v", entries)
	}

	// Test path bounding
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Error("Expected error for path escape, got nil")
	}
}

func TestCloudFSProvider(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "cloudfs_test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	provider, err := NewCloudFSProvider(tempDir)
	if err != nil {
		t.Fatal(err)
	}
	tenantID := "tenant-123"

	// Create context with claims
	claims := &auth.Claims{OrganizationID: tenantID}
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, claims)

	// Test context without claims
	_, err = provider.ReadFile(context.Background(), "test.txt")
	if err == nil {
		t.Error("Expected error for missing claims, got nil")
	}

	// Test write and read
	testPath := "test.txt"
	testContent := []byte("hello cloud")
	err = provider.WriteFile(ctx, testPath, testContent)
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Verify it was written to the correct tenant directory
	tenantDirPath := filepath.Join(tempDir, tenantID)
	tenantFilePath := filepath.Join(tenantDirPath, testPath)
	if _, err := os.Stat(tenantFilePath); os.IsNotExist(err) {
		t.Errorf("File was not written to tenant directory: %s", tenantFilePath)
	}

	data, err := provider.ReadFile(ctx, testPath)
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("Expected %s, got %s", testContent, data)
	}

	// Test path bounding
	_, err = provider.ReadFile(ctx, "../escape.txt")
	if err == nil {
		t.Error("Expected error for path escape, got nil")
	}
}

func TestNewProvider(t *testing.T) {
	p1, err := NewProvider(true, "/tmp")
	if err != nil {
		t.Fatal(err)
	}
	if _, ok := p1.(*LocalFSProvider); !ok {
		t.Error("Expected LocalFSProvider")
	}

	p2, err := NewProvider(false, "/tmp")
	if err != nil {
		t.Fatal(err)
	}
	if _, ok := p2.(*CloudFSProvider); !ok {
		t.Error("Expected CloudFSProvider")
	}
}
