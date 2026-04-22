package obsidian

import (
	"context"
	"os"
	"path/filepath"
	"testing"
)

func TestObsidianTool_ListNotes(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "obsidian_vault")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	notes := []string{"note1.md", "folder/note2.md", "not_a_note.txt"}
	for _, n := range notes {
		path := filepath.Join(tempDir, n)
		os.MkdirAll(filepath.Dir(path), 0755)
		os.WriteFile(path, []byte("content"), 0644)
	}

	t.Setenv("OHC_STANDALONE", "true")
	tool := NewObsidianTool(nil)
	found, err := tool.ListNotes(context.Background(), tempDir)
	if err != nil {
		t.Errorf("ListNotes failed: %v", err)
	}

	if len(found) != 2 {
		t.Errorf("expected 2 notes, got %d", len(found))
	}
}

func TestObsidianTool_ReadNote(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "obsidian_vault")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	notePath := "test.md"
	content := "hello world"
	os.WriteFile(filepath.Join(tempDir, notePath), []byte(content), 0644)

	t.Setenv("OHC_STANDALONE", "true")
	tool := NewObsidianTool(nil)
	note, err := tool.ReadNote(context.Background(), tempDir, notePath)
	if err != nil {
		t.Errorf("ReadNote failed: %v", err)
	}

	if note.Content != content {
		t.Errorf("expected content %s, got %s", content, note.Content)
	}
}

func TestObsidianTool_Execute_Standalone(t *testing.T) {
	tempDir, err := os.MkdirTemp("", "obsidian_vault")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tempDir)

	notePath := "test.md"
	content := "hello world"
	os.WriteFile(filepath.Join(tempDir, notePath), []byte(content), 0644)

	t.Setenv("OHC_STANDALONE", "true")
	tool := NewObsidianTool(nil)

	payload := map[string]interface{}{
		"action": "read_note",
		"vault_path": tempDir,
		"note_path": notePath,
	}

	result, err := tool.Execute(context.Background(), payload)
	if err != nil {
		t.Errorf("Execute failed: %v", err)
	}

	if result.Status != "success" {
		t.Errorf("expected status success, got %s", result.Status)
	}
}

func TestObsidianTool_Execute_Cloud(t *testing.T) {
	t.Setenv("OHC_STANDALONE", "false")
	tool := NewObsidianTool(nil)

	payload := map[string]interface{}{
		"action": "list_notes",
		"vault_path": "/some/path",
	}

	result, err := tool.Execute(context.Background(), payload)
	if err != nil {
		t.Errorf("Execute failed: %v", err)
	}

	if result.Status != "success" {
		t.Errorf("expected status success, got %s", result.Status)
	}

	if !os.IsPathSeparator('/') || !os.IsPathSeparator('\\') { // Simple check to see if we got mocked data
		if string(result.ResultData) == "" {
			t.Errorf("expected result data, got empty")
		}
	}
}
