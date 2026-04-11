package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_ReadFile(t *testing.T) {
	tempDir := t.TempDir()
	p := NewLocalFSProvider(tempDir)

	testPath := "test.txt"
	testContent := []byte("hello local")

	err := os.WriteFile(filepath.Join(tempDir, testPath), testContent, 0644)
	if err != nil {
		t.Fatalf("failed to write test file: %v", err)
	}

	data, err := p.ReadFile(context.Background(), nil, testPath)
	if err != nil {
		t.Errorf("ReadFile failed: %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("expected %s, got %s", testContent, data)
	}

	// Test directory traversal protection
	_, err = p.ReadFile(context.Background(), nil, "../outside.txt")
	if err == nil {
		t.Errorf("expected error for directory traversal, got nil")
	}
}

func TestLocalFSProvider_WriteFile(t *testing.T) {
	tempDir := t.TempDir()
	p := NewLocalFSProvider(tempDir)

	testPath := "out.txt"
	testContent := []byte("writing test")

	err := p.WriteFile(context.Background(), nil, testPath, testContent)
	if err != nil {
		t.Errorf("WriteFile failed: %v", err)
	}

	data, err := os.ReadFile(filepath.Join(tempDir, testPath))
	if err != nil {
		t.Fatalf("failed to read test file: %v", err)
	}
	if string(data) != string(testContent) {
		t.Errorf("expected %s, got %s", testContent, data)
	}

	// Test directory traversal protection
	err = p.WriteFile(context.Background(), nil, "../outside_write.txt", testContent)
	if err == nil {
		t.Errorf("expected error for directory traversal, got nil")
	}
}

func TestLocalFSProvider_ListDir(t *testing.T) {
	tempDir := t.TempDir()
	p := NewLocalFSProvider(tempDir)

	os.WriteFile(filepath.Join(tempDir, "a.txt"), []byte("a"), 0644)
	os.WriteFile(filepath.Join(tempDir, "b.txt"), []byte("b"), 0644)

	entries, err := p.ListDir(context.Background(), nil, "")
	if err != nil {
		t.Errorf("ListDir failed: %v", err)
	}

	if len(entries) != 2 {
		t.Errorf("expected 2 entries, got %d", len(entries))
	}

	// Test directory traversal protection
	_, err = p.ListDir(context.Background(), nil, "../")
	if err == nil {
		t.Errorf("expected error for directory traversal, got nil")
	}
}

func TestCloudFSProvider_Isolation(t *testing.T) {
	p := NewCloudFSProvider()

	claimsA := &auth.Claims{OrganizationID: "org-a"}
	claimsB := &auth.Claims{OrganizationID: "org-b"}

	err := p.WriteFile(context.Background(), claimsA, "data.txt", []byte("a-data"))
	if err != nil {
		t.Fatalf("WriteFile A failed: %v", err)
	}

	err = p.WriteFile(context.Background(), claimsB, "data.txt", []byte("b-data"))
	if err != nil {
		t.Fatalf("WriteFile B failed: %v", err)
	}

	// Read A
	dataA, err := p.ReadFile(context.Background(), claimsA, "data.txt")
	if err != nil {
		t.Errorf("ReadFile A failed: %v", err)
	}
	if string(dataA) != "a-data" {
		t.Errorf("expected a-data, got %s", dataA)
	}

	// Read B
	dataB, err := p.ReadFile(context.Background(), claimsB, "data.txt")
	if err != nil {
		t.Errorf("ReadFile B failed: %v", err)
	}
	if string(dataB) != "b-data" {
		t.Errorf("expected b-data, got %s", dataB)
	}

	// No claims should fail
	_, err = p.ReadFile(context.Background(), nil, "data.txt")
	if err == nil {
		t.Errorf("expected error when claims are missing, got nil")
	}
}
