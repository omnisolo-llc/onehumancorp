package mcp

import (
	"context"
	"errors"
	"os"
	"path/filepath"
	"testing"
)

type mockDB struct {
	enqueueErr error
	syncErr    error
	payloads   [][]byte
	toolNames  []string
	syncCalled bool
}

func (m *mockDB) EnqueueState(ctx context.Context, toolName string, payload []byte) error {
	if m.enqueueErr != nil {
		return m.enqueueErr
	}
	m.toolNames = append(m.toolNames, toolName)
	m.payloads = append(m.payloads, payload)
	return nil
}

func (m *mockDB) Sync(ctx context.Context) error {
	m.syncCalled = true
	return m.syncErr
}

func TestWorkspaceSyncTool_InvalidStrategy(t *testing.T) {
	proxy := NewMcpSyncProxy(&mockDB{})
	tool := NewWorkspaceSyncTool(proxy)

	err := tool.Execute(context.Background(), "/tmp", "invalid_strat")
	if err == nil || err.Error() != "invalid strategy: invalid_strat" {
		t.Fatalf("expected invalid strategy error, got: %v", err)
	}
}

func TestWorkspaceSyncTool_WalkError(t *testing.T) {
	proxy := NewMcpSyncProxy(&mockDB{})
	tool := NewWorkspaceSyncTool(proxy)

	// Non-existent path to trigger walk error
	err := tool.Execute(context.Background(), "/does/not/exist/ever/12345", "metadata_only")
	if err == nil {
		t.Fatal("expected walk error, got nil")
	}
}

func TestWorkspaceSyncTool_MetadataOnly(t *testing.T) {
	tmpDir := t.TempDir()
	file1 := filepath.Join(tmpDir, "file1.txt")
	os.WriteFile(file1, []byte("hello"), 0644)

	db := &mockDB{}
	proxy := NewMcpSyncProxy(db)
	tool := NewWorkspaceSyncTool(proxy)

	err := tool.Execute(context.Background(), tmpDir, "metadata_only")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if len(db.payloads) != 1 {
		t.Fatalf("expected 1 payload, got %d", len(db.payloads))
	}
	if !db.syncCalled {
		t.Fatal("expected sync to be called")
	}
}

func TestWorkspaceSyncTool_FullContent(t *testing.T) {
	tmpDir := t.TempDir()
	file1 := filepath.Join(tmpDir, "file1.txt")
	os.WriteFile(file1, []byte("hello"), 0644)

	db := &mockDB{}
	proxy := NewMcpSyncProxy(db)
	tool := NewWorkspaceSyncTool(proxy)

	err := tool.Execute(context.Background(), tmpDir, "full_content")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

func TestWorkspaceSyncTool_BufferError(t *testing.T) {
	tmpDir := t.TempDir()
	file1 := filepath.Join(tmpDir, "file1.txt")
	os.WriteFile(file1, []byte("hello"), 0644)

	db := &mockDB{enqueueErr: errors.New("buffer error")}
	proxy := NewMcpSyncProxy(db)
	tool := NewWorkspaceSyncTool(proxy)

	err := tool.Execute(context.Background(), tmpDir, "metadata_only")
	if err == nil || err.Error() != "failed to buffer state: buffer error" {
		t.Fatalf("expected buffer error, got: %v", err)
	}
}

func TestWorkspaceSyncTool_SyncError(t *testing.T) {
	tmpDir := t.TempDir()
	file1 := filepath.Join(tmpDir, "file1.txt")
	os.WriteFile(file1, []byte("hello"), 0644)

	db := &mockDB{syncErr: errors.New("sync error")}
	proxy := NewMcpSyncProxy(db)
	tool := NewWorkspaceSyncTool(proxy)

	err := tool.Execute(context.Background(), tmpDir, "metadata_only")
	if err == nil || err.Error() != "failed to sync states: sync error" {
		t.Fatalf("expected sync error, got: %v", err)
	}
}

// To get 100% code coverage, we also need to cover `json.Marshal(payload)` failing,
// but maps with string keys and serializable values don't fail `json.Marshal`.
// We can use a trick by intercepting json.Marshal if it was a variable, but
// standard library's `json.Marshal` failure is hard to trigger with just simple types.
// We'll leave it as is, and verify coverage first.
// Also test file > 1MB condition for "full_content" strategy.
func TestWorkspaceSyncTool_LargeFile(t *testing.T) {
	tmpDir := t.TempDir()
	file1 := filepath.Join(tmpDir, "large.txt")

	// Create a dummy file and manually pretend it's large by creating a file info mock,
	// but since we're using os.Walk, we'd actually need to write > 1MB.
	// Let's write slightly over 1MB.
	largeData := make([]byte, 1024*1024+10)
	os.WriteFile(file1, largeData, 0644)

	db := &mockDB{}
	proxy := NewMcpSyncProxy(db)
	tool := NewWorkspaceSyncTool(proxy)

	err := tool.Execute(context.Background(), tmpDir, "full_content")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
}

// How to trigger json.Marshal error?
// The payload contains:
// map[string]interface{}{
// 	"workspace_path": path,
// 	"strategy":       strategy,
// 	"files":          files,
// }
// `files` is `[]FileData`
// `FileData` contains standard types.
// It is impossible to trigger `json.Marshal` failure here without mocking or changing the struct to contain a channel or unmarshallable type, but standard types won't fail.
// So json.Marshal error path might be uncovered if we don't cheat or we just remove the error check (but that's bad practice).
func TestWorkspaceSyncTool_JSONMarshalError(t *testing.T) {
	tmpDir := t.TempDir()

	// Mock jsonMarshal to force an error
	originalJSONMarshal := jsonMarshal
	defer func() { jsonMarshal = originalJSONMarshal }()
	jsonMarshal = func(v any) ([]byte, error) {
		return nil, errors.New("mock marshal error")
	}

	db := &mockDB{}
	proxy := NewMcpSyncProxy(db)
	tool := NewWorkspaceSyncTool(proxy)

	err := tool.Execute(context.Background(), tmpDir, "metadata_only")
	if err == nil || err.Error() != "failed to marshal state: mock marshal error" {
		t.Fatalf("expected json marshal error, got: %v", err)
	}
}
