package hybridfsmcp

import (
	"context"
	"testing"
)

func TestLocalFSProvider(t *testing.T) {
	tempDir := t.TempDir()
	provider, err := NewLocalFSProvider(tempDir)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	ctx := context.Background()

	t.Run("Write and Read File", func(t *testing.T) {
		err := provider.WriteFile(ctx, nil, "test.txt", []byte("hello"))
		if err != nil {
			t.Fatalf("WriteFile failed: %v", err)
		}

		content, err := provider.ReadFile(ctx, nil, "test.txt")
		if err != nil {
			t.Fatalf("ReadFile failed: %v", err)
		}
		if string(content) != "hello" {
			t.Errorf("Expected 'hello', got '%s'", string(content))
		}
	})

	t.Run("Path Traversal Blocked", func(t *testing.T) {
		err := provider.WriteFile(ctx, nil, "../outside.txt", []byte("bad"))
		if err == nil {
			t.Error("Expected error for path traversal, got nil")
		}
	})

	t.Run("List Dir", func(t *testing.T) {
		provider.WriteFile(ctx, nil, "dir/file1.txt", []byte("1"))
		provider.WriteFile(ctx, nil, "dir/file2.txt", []byte("2"))
		entries, err := provider.ListDir(ctx, nil, "dir")
		if err != nil {
			t.Fatalf("ListDir failed: %v", err)
		}
		if len(entries) != 2 {
			t.Errorf("Expected 2 entries, got %d", len(entries))
		}
	})
}
