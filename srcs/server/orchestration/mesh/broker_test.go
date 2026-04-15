package mesh

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"
)

func TestLocalMeshBroker(t *testing.T) {
	broker := NewLocalMeshBroker()
	ch := broker.Subscribe("test_channel")

	err := broker.Broadcast(context.Background(), "test_channel", []byte("hello"))
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	select {
	case msg := <-ch:
		if string(msg) != "hello" {
			t.Fatalf("expected hello, got %s", string(msg))
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for message")
	}
}

func TestHTTPHandler(t *testing.T) {
	broker := NewLocalMeshBroker()
	ch := broker.Subscribe("test_channel")

	handler := NewHTTPHandler(broker)

	reqBody := `{"channel":"test_channel", "event_type":"test_event", "data":{"key":"value"}}`
	req := httptest.NewRequest(http.MethodPost, "/", strings.NewReader(reqBody))
	req.Header.Set("Content-Type", "application/json")

	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected 200 OK, got %d", w.Code)
	}

	select {
	case msg := <-ch:
		var result map[string]interface{}
		if err := json.Unmarshal(msg, &result); err != nil {
			t.Fatalf("failed to unmarshal message: %v", err)
		}
		if result["event_type"] != "test_event" {
			t.Fatalf("expected event_type test_event, got %v", result["event_type"])
		}
	case <-time.After(1 * time.Second):
		t.Fatal("timeout waiting for broadcast message")
	}
}
