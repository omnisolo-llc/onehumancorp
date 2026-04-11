package builtin

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestFileEditTool(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "fileedit_test_*")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	filePath := filepath.Join(tmpDir, "test.txt")
	err = os.WriteFile(filePath, []byte("Hello, world!\nThis is a test file.\n"), 0644)
	if err != nil {
		t.Fatal(err)
	}

	// Test successful replace
	args := []byte(`{"file_path":"` + filePath + `","old_string":"world","new_string":"universe"}`)
	_, err = FileEditTool.Execute(context.Background(), args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	content, _ := os.ReadFile(filePath)
	if string(content) != "Hello, universe!\nThis is a test file.\n" {
		t.Fatalf("unexpected content: %s", string(content))
	}

	// Test replace_all
	err = os.WriteFile(filePath, []byte("apple apple apple"), 0644)
	if err != nil {
		t.Fatal(err)
	}

	args = []byte(`{"file_path":"` + filePath + `","old_string":"apple","new_string":"banana","replace_all":true}`)
	_, err = FileEditTool.Execute(context.Background(), args)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	content, _ = os.ReadFile(filePath)
	if string(content) != "banana banana banana" {
		t.Fatalf("unexpected content: %s", string(content))
	}
}
