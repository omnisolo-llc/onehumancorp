package settings

import (
	"os"
	"path/filepath"
	"testing"
)

func TestStore_SaveAndLoad(t *testing.T) {
	tmpDir, err := os.MkdirTemp("", "settings-test")
	if err != nil {
		t.Fatal(err)
	}
	defer os.RemoveAll(tmpDir)

	path := filepath.Join(tmpDir, "openclaw.json")
	store, err := FromFile(path)
	if err != nil {
		t.Fatalf("failed to create store: %v", err)
	}

	s := store.Get()
	s.ListenAddr = "1.2.3.4:5678"
	s.AiProviders = []AiProvider{
		{Name: "test", Model: "gpt-4o-mini", Enabled: true},
	}

	if err := store.Update(s); err != nil {
		t.Fatalf("failed to update store: %v", err)
	}

	// Load again
	store2, err := FromFile(path)
	if err != nil {
		t.Fatalf("failed to load store again: %v", err)
	}

	s2 := store2.Get()
	if s2.ListenAddr != "1.2.3.4:5678" {
		t.Errorf("expected listen addr 1.2.3.4:5678, got %s", s2.ListenAddr)
	}
	if len(s2.AiProviders) != 1 || s2.AiProviders[0].Name != "test" {
		t.Errorf("expected 1 AI provider 'test', got %v", s2.AiProviders)
	}
}

func TestStore_SetExtra(t *testing.T) {
	store := NewStore()
	if err := store.SetExtra("foo", "bar"); err != nil {
		t.Fatalf("failed to set extra: %v", err)
	}

	s := store.Get()
	if s.Extras["foo"] != "bar" {
		t.Errorf("expected extra 'foo' to be 'bar', got %s", s.Extras["foo"])
	}
}

func TestDefaultSettingsCentrifugeURL(t *testing.T) {
	defaults := DefaultSettings()
	if defaults.CentrifugeURL == "" {
		t.Error("DefaultSettings().CentrifugeURL must not be empty")
	}
}

func TestStore_CentrifugeAndMinimax(t *testing.T) {
	store := NewStore()
	s := store.Get()
	s.CentrifugeURL = "ws://centrifuge:8000/connection/websocket"
	s.MinimaxAPIKey = "sk-test-key"
	if err := store.Update(s); err != nil {
		t.Fatalf("failed to update store with centrifuge/minimax fields: %v", err)
	}
	loaded := store.Get()
	if loaded.CentrifugeURL != "ws://centrifuge:8000/connection/websocket" {
		t.Errorf("CentrifugeURL = %q, want %q", loaded.CentrifugeURL, "ws://centrifuge:8000/connection/websocket")
	}
	if loaded.MinimaxAPIKey != "sk-test-key" {
		t.Errorf("MinimaxAPIKey = %q, want %q", loaded.MinimaxAPIKey, "sk-test-key")
	}
}

func TestStore_FromFileErrors(t *testing.T) {
	// write invalid json
	f, _ := os.CreateTemp("", "bad-*.json")
	f.Write([]byte("{bad json"))
	f.Close()
	defer os.Remove(f.Name())

	_, err := FromFile(f.Name())
	if err == nil {
		t.Fatal("expected error on bad json")
	}

	// Unreadable file (directory)
	_, err = FromFile(t.TempDir())
	if err == nil {
		t.Fatal("expected error on dir")
	}
}

func TestStore_SaveErrors(t *testing.T) {
	// Cannot create dir because a path component is already a file.
	dir := t.TempDir()
	parentFile := filepath.Join(dir, "parent")
	if err := os.WriteFile(parentFile, []byte("not a directory"), 0o600); err != nil {
		t.Fatalf("failed to create parent file: %v", err)
	}
	store := &Store{path: filepath.Join(parentFile, "file.json")}
	err := store.Save()
	if err == nil {
		t.Fatal("expected error creating dir")
	}

	// Cannot write file because the destination path is an existing directory.
	d := t.TempDir()
	targetDir := filepath.Join(d, "file.json")
	if err := os.Mkdir(targetDir, 0o755); err != nil {
		t.Fatalf("failed to create target directory: %v", err)
	}
	store2 := &Store{path: targetDir}
	err = store2.Save()
	if err == nil {
		t.Fatal("expected error writing file")
	}
}

func TestStore_SetExtraEmptyExtras(t *testing.T) {
	store := NewStore()
	store.data.Extras = nil
	if err := store.SetExtra("key1", "val1"); err != nil {
		t.Fatalf("failed to set extra on nil map: %v", err)
	}
}
