package agents

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestFileBasedMemory(t *testing.T) {
	tmpDir := t.TempDir()
	mem := NewFileBasedMemory(tmpDir)

	ctx := context.Background()
	namespace := "auto"
	key := "test_key.txt"
	data := []byte("test content")

	// Test Write
	err := mem.Write(ctx, namespace, key, data)
	if err != nil {
		t.Fatalf("failed to write memory: %v", err)
	}

	// Test Read
	readData, err := mem.Read(ctx, namespace, key)
	if err != nil {
		t.Fatalf("failed to read memory: %v", err)
	}

	if string(readData) != string(data) {
		t.Errorf("expected %q, got %q", data, readData)
	}

	// Test directory creation
	info, err := os.Stat(filepath.Join(tmpDir, namespace))
	if err != nil {
		t.Fatalf("directory was not created: %v", err)
	}
	if !info.IsDir() {
		t.Errorf("expected %s to be a directory", filepath.Join(tmpDir, namespace))
	}

	// Test Path Traversal
	err = mem.Write(ctx, "auto", "../../passwd", data)
	if err == nil {
		t.Fatalf("expected error on path traversal attempt")
	}

	_, err = mem.Read(ctx, "auto", "../../passwd")
	if err == nil {
		t.Fatalf("expected error on path traversal attempt during read")
	}
}
