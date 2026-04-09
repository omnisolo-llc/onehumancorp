package hybridfsmcp

import (
	"context"
	"os"
	"path/filepath"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestLocalFSProvider_ResolvePath(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)

	tests := []struct {
		name    string
		target  string
		wantErr bool
	}{
		{"valid file", "test.txt", false},
		{"valid sub-directory", "sub/test.txt", false},
		{"invalid directory traversal", "../test.txt", true},
		{"invalid directory traversal nested", "sub/../../test.txt", true},
		{"absolute path outside", "/tmp/test.txt", true},
		{"valid absolute path inside", filepath.Join(tempDir, "test.txt"), false},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			_, err := provider.resolvePath(tt.target)
			if (err != nil) != tt.wantErr {
				t.Errorf("resolvePath() error = %v, wantErr %v", err, tt.wantErr)
			}
		})
	}
}

func TestLocalFSProvider_Operations(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewLocalFSProvider(tempDir)
	ctx := context.Background()

	// Test WriteFile
	err := provider.WriteFile(ctx, nil, "folder/test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, nil, "folder/test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "hello" {
		t.Errorf("ReadFile returned %s, want hello", string(content))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, nil, "folder")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("ListDir returned unexpected entries: %v", entries)
	}
}

func TestCloudFSProvider_ResolvePath(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	claims := &auth.Claims{OrganizationID: "org123"}

	tests := []struct {
		name    string
		claims  *auth.Claims
		target  string
		wantErr bool
	}{
		{"valid file", claims, "test.txt", false},
		{"valid sub-directory", claims, "sub/test.txt", false},
		{"missing claims", nil, "test.txt", true},
		{"missing org ID", &auth.Claims{}, "test.txt", true},
		{"invalid directory traversal", claims, "../test.txt", true},
		{"invalid directory traversal nested", claims, "sub/../../test.txt", true},
		{"absolute path", claims, "/test.txt", true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			fullPath, err := provider.resolvePath(tt.claims, tt.target)
			if (err != nil) != tt.wantErr {
				t.Errorf("resolvePath() error = %v, wantErr %v", err, tt.wantErr)
			}
			if err == nil {
				expectedPrefix := filepath.Join(tempDir, tt.claims.OrganizationID)
				if fullPath[:len(expectedPrefix)] != expectedPrefix {
					t.Errorf("Path isolation failed, expected prefix %s, got %s", expectedPrefix, fullPath)
				}
			}
		})
	}
}

func TestCloudFSProvider_Operations(t *testing.T) {
	tempDir := t.TempDir()
	provider := NewCloudFSProvider(tempDir)
	ctx := context.Background()
	claims := &auth.Claims{OrganizationID: "org123"}

	// Test WriteFile
	err := provider.WriteFile(ctx, claims, "folder/test.txt", []byte("world"))
	if err != nil {
		t.Fatalf("WriteFile failed: %v", err)
	}

	// Ensure the file was actually written to the isolated tenant directory
	if _, err := os.Stat(filepath.Join(tempDir, "org123", "folder", "test.txt")); os.IsNotExist(err) {
		t.Errorf("File was not written to the tenant-isolated path")
	}

	// Test ReadFile
	content, err := provider.ReadFile(ctx, claims, "folder/test.txt")
	if err != nil {
		t.Fatalf("ReadFile failed: %v", err)
	}
	if string(content) != "world" {
		t.Errorf("ReadFile returned %s, want world", string(content))
	}

	// Test ListDir
	entries, err := provider.ListDir(ctx, claims, "folder")
	if err != nil {
		t.Fatalf("ListDir failed: %v", err)
	}
	if len(entries) != 1 || entries[0].Name() != "test.txt" {
		t.Errorf("ListDir returned unexpected entries: %v", entries)
	}
}
