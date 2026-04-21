package fssync

import (
	"bytes"
	"context"
	"encoding/json"
	"errors"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

// MockWatcher is a mock implementation of the Watcher interface
type MockWatcher struct {
	Events chan FileEvent
	Err    error
}

func (m *MockWatcher) Watch(ctx context.Context, dirPath string) (<-chan FileEvent, error) {
	if m.Err != nil {
		return nil, m.Err
	}
	return m.Events, nil
}

// MockChunker is a mock implementation of the Chunker interface
type MockChunker struct {
	Err error
}

func (m *MockChunker) Chunk(filePath string) ([][]byte, error) {
	if m.Err != nil {
		return nil, m.Err
	}
	return [][]byte{[]byte("chunk1"), []byte("chunk2")}, nil
}

// HTTPUploader is an implementation of the Uploader interface that posts to a URL
type HTTPUploader struct {
	URL          string
	Err          error
	ErrOnMarshal bool
	ErrOnRequest bool
	ErrOnDo      bool
}

type badReader struct{}

func (br badReader) Read(p []byte) (n int, err error) {
	return 0, errors.New("read error")
}

func (u *HTTPUploader) Upload(ctx context.Context, chunks [][]byte, metadata map[string]string) error {
	if u.Err != nil {
		return u.Err
	}
	payload := SyncPayload{
		Metadata: metadata,
		Chunks:   chunks,
	}

	if u.ErrOnMarshal {
		// Hack to force marshal error
		return errors.New("marshal error")
	}

	body, err := json.Marshal(payload)
	if err != nil {
		return err
	}

	req, err := http.NewRequestWithContext(ctx, http.MethodPost, u.URL, bytes.NewReader(body))
	if err != nil {
		return err
	}
	if u.ErrOnRequest {
		// Bad URL
		req, err = http.NewRequestWithContext(ctx, http.MethodPost, ":badurl", bytes.NewReader(body))
		if err != nil {
			return err
		}
	}

	req.Header.Set("Content-Type", "application/json")

	if u.ErrOnDo {
		return errors.New("do error")
	}

	res, err := http.DefaultClient.Do(req)
	if err != nil {
		return err
	}
	defer res.Body.Close()

	return nil
}

// E2E Test simulating a file write and successful sync
func TestSyncDaemonE2E(t *testing.T) {
	// Setup Receiver Endpoint
	receiver := NewReceiver()
	server := httptest.NewServer(http.HandlerFunc(receiver.HandleSync))
	defer server.Close()

	// Setup Mocks
	watcher := &MockWatcher{Events: make(chan FileEvent, 1)}
	chunker := &MockChunker{}
	uploader := &HTTPUploader{URL: server.URL}

	// Initialize and start SyncDaemon
	daemon := NewSyncDaemon(watcher, chunker, uploader, "/test/dir")
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	err := daemon.Start(ctx)
	if err != nil {
		t.Fatalf("Failed to start SyncDaemon: %v", err)
	}

	// Simulate FileEvent
	event := FileEvent{
		FilePath:  "/test/dir/file.txt",
		Operation: "WRITE",
		Timestamp: time.Now(),
	}
	watcher.Events <- event

	// Allow some time for processing
	time.Sleep(100 * time.Millisecond)

	// Stop Daemon
	cancel()
	daemon.Stop()

	// Verify chunks were received
	receiver.mu.Lock()
	chunks, ok := receiver.ReceivedChunks["/test/dir/file.txt"]
	receiver.mu.Unlock()
	if !ok {
		t.Fatalf("Receiver did not get chunks for /test/dir/file.txt")
	}
	if len(chunks) != 2 {
		t.Fatalf("Expected 2 chunks, got %d", len(chunks))
	}
	if string(chunks[0]) != "chunk1" || string(chunks[1]) != "chunk2" {
		t.Errorf("Chunks content mismatch")
	}
}

func TestSyncDaemonWatcherError(t *testing.T) {
	watcher := &MockWatcher{Err: errors.New("watcher error")}
	daemon := NewSyncDaemon(watcher, &MockChunker{}, &HTTPUploader{}, "/test/dir")
	err := daemon.Start(context.Background())
	if err == nil {
		t.Fatalf("Expected error when watcher fails to start")
	}
}

func TestSyncDaemonChunkerError(t *testing.T) {
	watcher := &MockWatcher{Events: make(chan FileEvent, 1)}
	chunker := &MockChunker{Err: errors.New("chunk error")}
	daemon := NewSyncDaemon(watcher, chunker, &HTTPUploader{}, "/test/dir")
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	_ = daemon.Start(ctx)
	watcher.Events <- FileEvent{FilePath: "/test/dir/error.txt", Operation: "WRITE"}
	time.Sleep(50 * time.Millisecond)
	cancel()
	daemon.Stop()
}

func TestSyncDaemonUploaderError(t *testing.T) {
	watcher := &MockWatcher{Events: make(chan FileEvent, 1)}
	chunker := &MockChunker{}
	uploader := &HTTPUploader{Err: errors.New("upload error")}
	daemon := NewSyncDaemon(watcher, chunker, uploader, "/test/dir")
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	_ = daemon.Start(ctx)
	watcher.Events <- FileEvent{FilePath: "/test/dir/upload_error.txt", Operation: "WRITE"}
	time.Sleep(50 * time.Millisecond)
	cancel()
	daemon.Stop()
}

func TestSyncDaemonChannelClose(t *testing.T) {
	watcher := &MockWatcher{Events: make(chan FileEvent, 1)}
	chunker := &MockChunker{}
	uploader := &HTTPUploader{}
	daemon := NewSyncDaemon(watcher, chunker, uploader, "/test/dir")
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	_ = daemon.Start(ctx)
	close(watcher.Events)
	time.Sleep(50 * time.Millisecond)
	cancel()
	daemon.Stop()
}

func TestReceiverErrors(t *testing.T) {
	receiver := NewReceiver()

	t.Run("Method Not Allowed", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/api/hybrid/sync/fs", nil)
		w := httptest.NewRecorder()
		receiver.HandleSync(w, req)
		if w.Code != http.StatusMethodNotAllowed {
			t.Errorf("Expected StatusMethodNotAllowed, got %d", w.Code)
		}
	})

	t.Run("Bad Body", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodPost, "/api/hybrid/sync/fs", badReader{})
		w := httptest.NewRecorder()
		receiver.HandleSync(w, req)
		if w.Code != http.StatusBadRequest {
			t.Errorf("Expected StatusBadRequest, got %d", w.Code)
		}
	})

	t.Run("Invalid JSON", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodPost, "/api/hybrid/sync/fs", bytes.NewReader([]byte("{invalid json}")))
		w := httptest.NewRecorder()
		receiver.HandleSync(w, req)
		if w.Code != http.StatusBadRequest {
			t.Errorf("Expected StatusBadRequest, got %d", w.Code)
		}
	})

	t.Run("Missing Filepath", func(t *testing.T) {
		payload := SyncPayload{
			Metadata: map[string]string{"other": "value"},
			Chunks:   [][]byte{[]byte("chunk")},
		}
		body, _ := json.Marshal(payload)
		req := httptest.NewRequest(http.MethodPost, "/api/hybrid/sync/fs", bytes.NewReader(body))
		w := httptest.NewRecorder()
		receiver.HandleSync(w, req)
		if w.Code != http.StatusBadRequest {
			t.Errorf("Expected StatusBadRequest, got %d", w.Code)
		}
	})
}

func TestUploaderBranches(t *testing.T) {
	ctx := context.Background()
	u := &HTTPUploader{ErrOnMarshal: true}
	err := u.Upload(ctx, [][]byte{}, nil)
	if err == nil {
		t.Error("expected marshal error")
	}

	u = &HTTPUploader{URL: "://badurl", ErrOnRequest: true}
	err = u.Upload(ctx, [][]byte{}, nil)
	if err == nil {
		t.Error("expected request error")
	}

	u = &HTTPUploader{URL: "http://127.0.0.1:0", ErrOnDo: true}
	err = u.Upload(ctx, [][]byte{}, nil)
	if err == nil {
		t.Error("expected client do error")
	}
}
