package hybridfsmcp

import (
	"context"
	"path/filepath"
	"testing"
)

func TestLocalFSProvider_ResolvePath(t *testing.T) {
	provider := NewLocalFSProvider("/base")

	_, err := provider.resolvePath("/etc/passwd")
	if err == nil {
		t.Errorf("expected error for absolute path")
	}

	_, err = provider.resolvePath("../foo")
	if err == nil {
		t.Errorf("expected error for directory traversal")
	}

	path, err := provider.resolvePath("test.txt")
	if err != nil {
		t.Errorf("unexpected error: %v", err)
	}
	expected := filepath.Join("/base", "test.txt")
	if path != expected {
		t.Errorf("expected %s, got %s", expected, path)
	}
}

func TestLocalFSProvider_WriteReadList(t *testing.T) {
	dir := t.TempDir()
	provider := NewLocalFSProvider(dir)

	ctx := context.Background()
	err := provider.WriteFile(ctx, "test.txt", []byte("hello"))
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	data, err := provider.ReadFile(ctx, "test.txt")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if string(data) != "hello" {
		t.Errorf("expected 'hello', got %s", string(data))
	}

	entries, err := provider.ListDir(ctx, ".")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if len(entries) != 1 || entries[0] != "test.txt" {
		t.Errorf("expected ['test.txt'], got %v", entries)
	}
}
