package mesh

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

func TestPublishHandler_Success(t *testing.T) {
	pubsub := NewMemoryPubSub()
	handler := NewPublishHandler(pubsub)

	wrapper := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		ctx := mockSPIFFEContext(r.Context(), "spiffe://example.org/agent/123")
		handler.ServeHTTP(w, r.WithContext(ctx))
	})

	server := httptest.NewServer(wrapper)
	defer server.Close()

	// Subscribe to verify message is published
	ch, unsubscribe, err := pubsub.Subscribe(context.Background(), "test-topic")
	if err != nil {
		t.Fatalf("Failed to subscribe: %v", err)
	}
	defer unsubscribe()

	reqBody := []byte(`{"topic": "test-topic", "message": {"foo": "bar"}}`)
	resp, err := http.Post(server.URL, "application/json", bytes.NewBuffer(reqBody))
	if err != nil {
		t.Fatalf("Failed to post: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		t.Errorf("Expected status 200, got %d", resp.StatusCode)
	}

	select {
	case msg := <-ch:
		expected := `{"foo": "bar"}`
		if string(msg) != expected {
			t.Errorf("Expected message '%s', got '%s'", expected, string(msg))
		}
	case <-time.After(1 * time.Second):
		t.Error("Timeout waiting for message")
	}
}

func TestPublishHandler_Unauthorized(t *testing.T) {
	pubsub := NewMemoryPubSub()
	handler := NewPublishHandler(pubsub)

	server := httptest.NewServer(handler)
	defer server.Close()

	reqBody := []byte(`{"topic": "test-topic", "message": {"foo": "bar"}}`)
	resp, err := http.Post(server.URL, "application/json", bytes.NewBuffer(reqBody))
	if err != nil {
		t.Fatalf("Failed to post: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusUnauthorized {
		t.Errorf("Expected status 401, got %d", resp.StatusCode)
	}
}
