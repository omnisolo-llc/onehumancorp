package mcp

import (
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_ValidPath(t *testing.T) {
	tempDir := t.TempDir()
	p := NewLocalFSProvider(tempDir)

	err := p.WriteFile("test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	content, err := p.ReadFile("test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(content) != "hello" {
		t.Errorf("Expected hello, got %s", content)
	}

	entries, err := p.ListDir(".")
	if err != nil {
		t.Fatalf("Failed to list dir: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("Expected [test.txt], got %v", entries)
	}
}

func TestLocalFSProvider_InvalidPath(t *testing.T) {
	tempDir := t.TempDir()
	p := NewLocalFSProvider(tempDir)

	err := p.WriteFile("../test.txt", []byte("hello"))
	if err != ErrAccessDenied {
		t.Errorf("Expected ErrAccessDenied, got %v", err)
	}

	err = p.WriteFile("/etc/passwd", []byte("hello"))
	if err != ErrInvalidPath {
		t.Errorf("Expected ErrInvalidPath, got %v", err)
	}
}

func TestCloudFSProvider_ValidPath(t *testing.T) {
	tempDir := t.TempDir()
	claims := &auth.Claims{OrganizationID: "org-123"}
	p := NewCloudFSProvider(tempDir, claims)

	err := p.WriteFile("test.txt", []byte("hello cloud"))
	if err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	content, err := p.ReadFile("test.txt")
	if err != nil {
		t.Fatalf("Failed to read file: %v", err)
	}
	if string(content) != "hello cloud" {
		t.Errorf("Expected hello cloud, got %s", content)
	}

	// Verify it actually went to the right tenant dir
	tenantPath := filepath.Join(tempDir, "org-123", "test.txt")
	if _, err := os.Stat(tenantPath); os.IsNotExist(err) {
		t.Errorf("Expected file at %s", tenantPath)
	}
}

func TestCloudFSProvider_InvalidPath(t *testing.T) {
	tempDir := t.TempDir()
	claims := &auth.Claims{OrganizationID: "org-123"}
	p := NewCloudFSProvider(tempDir, claims)

	err := p.WriteFile("../test.txt", []byte("hello"))
	if err != ErrAccessDenied {
		t.Errorf("Expected ErrAccessDenied, got %v", err)
	}

	err = p.WriteFile("/etc/passwd", []byte("hello"))
	if err != ErrInvalidPath {
		t.Errorf("Expected ErrInvalidPath, got %v", err)
	}
}
