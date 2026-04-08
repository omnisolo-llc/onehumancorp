package hybridfsmcp

import (
	"os"
	"testing"
)

func TestLocalFSProvider_PathBounding(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "localfs_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	provider, err := NewLocalFSProvider(tmpDir)
	if err != nil {
		t.Fatalf("failed to create provider: %v", err)
	}

	// Test valid paths
	err = provider.WriteFile("valid.txt", []byte("hello"))
	if err != nil {
		t.Errorf("expected success for valid path, got %v", err)
	}

	_, err = provider.ReadFile("valid.txt")
	if err != nil {
		t.Errorf("expected success for valid read, got %v", err)
	}

	// Test path escaping
	err = provider.WriteFile("../escape.txt", []byte("hello"))
	if err == nil {
		t.Errorf("expected error for escaping path, got nil")
	}

	_, err = provider.ReadFile("../../etc/passwd")
	if err == nil {
		t.Errorf("expected error for escaping path, got nil")
	}
}

func TestCloudFSProvider_TenantIsolation(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "cloudfs_test_*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	org1 := "org-1"
	org2 := "org-2"

	provider1, err := NewCloudFSProvider(tmpDir, org1)
	if err != nil {
		t.Fatalf("failed to create provider1: %v", err)
	}

	provider2, err := NewCloudFSProvider(tmpDir, org2)
	if err != nil {
		t.Fatalf("failed to create provider2: %v", err)
	}

	// org1 writes a file
	err = provider1.WriteFile("secret.txt", []byte("org1 secret"))
	if err != nil {
		t.Fatalf("org1 failed to write file: %v", err)
	}

	// org2 cannot read org1's file directly through its root
	_, err = provider2.ReadFile("secret.txt")
	if err == nil {
		t.Errorf("org2 should not see org1's files")
	}

	// org2 cannot path traverse into org1
	_, err = provider2.ReadFile("../org-1/secret.txt")
	if err == nil {
		t.Errorf("org2 should not be able to traverse into org1")
	}

	// org1 can read its own file
	content, err := provider1.ReadFile("secret.txt")
	if err != nil {
		t.Errorf("org1 should be able to read its own file: %v", err)
	}
	if string(content) != "org1 secret" {
		t.Errorf("expected 'org1 secret', got %s", content)
	}
}
