package fssync

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
	"time"
)

// HTTPUploader implements Uploader by making HTTP requests
type HTTPUploader struct {
	URL string
}

func (u *HTTPUploader) Upload(ctx context.Context, chunk FileChunk) error {
	data, err := json.Marshal(chunk)
	if err != nil {
		return err
	}
	req, err := http.NewRequestWithContext(ctx, http.MethodPost, u.URL, bytes.NewReader(data))
	if err != nil {
		return err
	}
	req.Header.Set("Content-Type", "application/json")
	resp, err := http.DefaultClient.Do(req)
	if err != nil {
		return err
	}
	defer resp.Body.Close()
	return nil
}

func TestWatcher(t *testing.T) {
	watcher := NewMockWatcher()
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	events, err := watcher.Watch(ctx)
	if err != nil {
		t.Fatalf("Failed to watch: %v", err)
	}

	expectedEvent := FileEvent{Path: "/test/path", Operation: "WRITE"}
	watcher.SimulateEvent(expectedEvent)

	select {
	case event := <-events:
		if event.Path != expectedEvent.Path || event.Operation != expectedEvent.Operation {
			t.Errorf("Expected event %v, got %v", expectedEvent, event)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for event")
	}

	watcher.Close()
}

func TestSyncDaemonProcessFile(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "test.txt")
	content := []byte("hello world this is a test")
	if err := os.WriteFile(filePath, content, 0644); err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	watcher := NewMockWatcher()
	uploader := NewMockUploader()
	// Set small chunk size to test chunking
	daemon := NewSyncDaemon(watcher, uploader, 5)

	err := daemon.processFile(context.Background(), filePath)
	if err != nil {
		t.Fatalf("processFile failed: %v", err)
	}

	if len(uploader.UploadedChunks) == 0 {
		t.Fatal("No chunks uploaded")
	}

	var reconstructed []byte
	for _, chunk := range uploader.UploadedChunks {
		if chunk.Path != filePath {
			t.Errorf("Expected path %v, got %v", filePath, chunk.Path)
		}
		reconstructed = append(reconstructed, chunk.Data...)
	}

	if !bytes.Equal(reconstructed, content) {
		t.Errorf("Reconstructed content mismatch. Expected %q, got %q", content, reconstructed)
	}
}

func TestSyncDaemonProcessEmptyFile(t *testing.T) {
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "empty.txt")
	if err := os.WriteFile(filePath, []byte{}, 0644); err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	watcher := NewMockWatcher()
	uploader := NewMockUploader()
	daemon := NewSyncDaemon(watcher, uploader, 5)

	err := daemon.processFile(context.Background(), filePath)
	if err != nil {
		t.Fatalf("processFile failed: %v", err)
	}

	if len(uploader.UploadedChunks) != 1 {
		t.Fatalf("Expected 1 chunk for empty file, got %d", len(uploader.UploadedChunks))
	}

	if len(uploader.UploadedChunks[0].Data) != 0 {
		t.Errorf("Expected empty chunk data, got %q", uploader.UploadedChunks[0].Data)
	}
}

func TestReceiver(t *testing.T) {
	receiver := NewReceiver()
	ts := httptest.NewServer(http.HandlerFunc(receiver.HandleSyncFS))
	defer ts.Close()

	chunk1 := FileChunk{
		Path:        "/test/file",
		ChunkIndex:  0,
		TotalChunks: 2,
		Data:        []byte("hello "),
	}
	chunk2 := FileChunk{
		Path:        "/test/file",
		ChunkIndex:  1,
		TotalChunks: 2,
		Data:        []byte("world"),
	}

	// Test GET method not allowed
	resp, err := http.Get(ts.URL)
	if err != nil {
		t.Fatalf("Failed to make request: %v", err)
	}
	if resp.StatusCode != http.StatusMethodNotAllowed {
		t.Errorf("Expected status 405, got %v", resp.StatusCode)
	}

	// Test POST chunks
	for _, chunk := range []FileChunk{chunk1, chunk2} {
		data, _ := json.Marshal(chunk)
		resp, err := http.Post(ts.URL, "application/json", bytes.NewReader(data))
		if err != nil {
			t.Fatalf("Failed to POST: %v", err)
		}
		if resp.StatusCode != http.StatusOK {
			t.Errorf("Expected status 200, got %v", resp.StatusCode)
		}
	}

	receiver.mu.Lock()
	defer receiver.mu.Unlock()

	reconstructed, ok := receiver.Reconstructed["/test/file"]
	if !ok {
		t.Fatal("File not reconstructed")
	}

	if string(reconstructed) != "hello world" {
		t.Errorf("Expected 'hello world', got '%s'", reconstructed)
	}
}

func TestE2ESyncFlow(t *testing.T) {
	// 1. Setup Receiver
	receiver := NewReceiver()
	ts := httptest.NewServer(http.HandlerFunc(receiver.HandleSyncFS))
	defer ts.Close()

	// 2. Setup file to watch
	tmpDir := t.TempDir()
	filePath := filepath.Join(tmpDir, "e2e.txt")
	content := []byte("End to end testing hybrid fs sync daemon.")
	if err := os.WriteFile(filePath, content, 0644); err != nil {
		t.Fatalf("Failed to write file: %v", err)
	}

	// 3. Setup Daemon
	watcher := NewMockWatcher()
	uploader := &HTTPUploader{URL: ts.URL}
	daemon := NewSyncDaemon(watcher, uploader, 10) // Small chunk size

	// 4. Start Daemon
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	errCh := make(chan error, 1)
	go func() {
		errCh <- daemon.Start(ctx)
	}()

	// 5. Simulate File Write Event
	watcher.SimulateEvent(FileEvent{Path: filePath, Operation: "WRITE"})

	// 6. Wait for processing
	time.Sleep(500 * time.Millisecond)

	// 7. Verify Receiver Reconstructed the file
	receiver.mu.Lock()
	reconstructed, ok := receiver.Reconstructed[filePath]
	receiver.mu.Unlock()

	if !ok {
		t.Fatal("File not reconstructed in receiver")
	}

	if !bytes.Equal(reconstructed, content) {
		t.Errorf("Reconstructed content mismatch. Expected %q, got %q", content, reconstructed)
	}

	// Check cancellation
	cancel()
	select {
	case err := <-errCh:
		if err != nil {
			t.Errorf("Daemon start returned error: %v", err)
		}
	case <-time.After(1 * time.Second):
		t.Fatal("Timeout waiting for daemon to exit")
	}
}
