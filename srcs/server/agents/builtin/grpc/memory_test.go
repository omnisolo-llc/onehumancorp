package agentgrpc

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
	"time"
)

func TestMemoryStore_StandalonePersistence(t *testing.T) {
	// Setup temporary directory for .ohc/memory
	tmpDir, err := os.MkdirTemp("", "ohc-memory-test-*")
	if err != nil {
		t.Fatalf("failed to create temp dir: %v", err)
	}
	defer os.RemoveAll(tmpDir)

	origWd, _ := os.Getwd()
	defer os.Chdir(origWd)
	os.Chdir(tmpDir)

	// Mock OHC_STANDALONE=true
	os.Setenv("OHC_STANDALONE", "true")
	defer os.Unsetenv("OHC_STANDALONE")

	store := newMemoryStore(10)
	entry := MemoryEntry{
		TaskID:      "test-task-1",
		Summary:     "Did some work",
		Outcome:     "success",
		CompletedAt: time.Now().Round(time.Second), // Round to avoid precision issues in JSON
	}

	store.Write(entry)

	// Give it a moment for the goroutine to finish writing
	time.Sleep(200 * time.Millisecond)

	// Verify file exists
	expectedPath := filepath.Join(tmpDir, ".ohc", "memory", "auto", "test-task-1.json")
	if _, err := os.Stat(expectedPath); os.IsNotExist(err) {
		t.Fatalf("memory file was not created at %s", expectedPath)
	}

	data, err := os.ReadFile(expectedPath)
	if err != nil {
		t.Fatalf("failed to read memory file: %v", err)
	}

	var readEntry MemoryEntry
	if err := json.Unmarshal(data, &readEntry); err != nil {
		t.Fatalf("failed to unmarshal memory file: %v", err)
	}

	if readEntry.TaskID != entry.TaskID {
		t.Errorf("expected TaskID %s, got %s", entry.TaskID, readEntry.TaskID)
	}

	// Test Loading from disk
	// We create a new store in a new process simulation
	newStore := newMemoryStore(10)
	// newMemoryStore calls LoadFromDisk if STANDALONE is true

	if len(newStore.entries) == 0 {
		t.Fatal("new store should have loaded entries from disk")
	}

	found := false
	for _, e := range newStore.entries {
		if e.TaskID == entry.TaskID {
			found = true
			break
		}
	}
	if !found {
		t.Errorf("expected TaskID %s to be loaded from disk", entry.TaskID)
	}
}
