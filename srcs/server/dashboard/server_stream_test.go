package dashboard

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleStream(t *testing.T) {
	// Create dummy server
	s := &Server{}

	// Setup mock request with cancelled context to prevent infinite loops
	ctx, cancel := context.WithCancel(context.Background())
	req := httptest.NewRequest("GET", "/api/v1/stream", nil)
	req = req.WithContext(ctx)

	// Since httptest.ResponseRecorder does not implement http.Flusher natively before Go 1.20 in a fully functional way for SSE,
	// We'll test that it sets headers properly and errors when flusher isn't supported,
	// or we can test with a custom response writer that implements Flusher.

	// Create a dummy ResponseWriter that implements Flusher
	w := &mockFlusherResponseWriter{
		ResponseRecorder: httptest.NewRecorder(),
	}

	// Run handler in a goroutine
	done := make(chan struct{})
	go func() {
		s.handleStream(w, req)
		close(done)
	}()

	// Wait a moment then cancel
	time.Sleep(10 * time.Millisecond)
	cancel()

	// Wait for handler to exit
	select {
	case <-done:
	case <-time.After(1 * time.Second):
		t.Fatal("handleStream did not exit when context was cancelled")
	}

	// Check headers
	if w.Header().Get("Content-Type") != "text/event-stream" {
		t.Errorf("expected Content-Type text/event-stream, got %s", w.Header().Get("Content-Type"))
	}
	if w.Header().Get("Cache-Control") != "no-cache" {
		t.Errorf("expected Cache-Control no-cache, got %s", w.Header().Get("Cache-Control"))
	}
	if w.Header().Get("Connection") != "keep-alive" {
		t.Errorf("expected Connection keep-alive, got %s", w.Header().Get("Connection"))
	}
}

type mockFlusherResponseWriter struct {
	*httptest.ResponseRecorder
}

func (m *mockFlusherResponseWriter) Flush() {
	m.ResponseRecorder.Flush()
}
