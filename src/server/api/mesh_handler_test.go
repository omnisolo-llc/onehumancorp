package api

import (
	"bytes"
	"context"
	"fmt"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/src/server/mesh"
)

type mockFailingTransport struct {
	mesh.MeshTransport
}

func (m *mockFailingTransport) Publish(ctx context.Context, channel string, payload []byte) error {
	return fmt.Errorf("simulated publish error")
}

func (m *mockFailingTransport) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	return nil, fmt.Errorf("simulated subscribe error")
}

func TestMeshHandler_Broadcast(t *testing.T) {
	transport := mesh.NewMemoryTransport()
	handler := NewMeshHandler(transport)

	t.Run("Valid Broadcast", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodPost, "/api/v1/mesh/broadcast", bytes.NewBuffer([]byte(`{"channel":"test","data":{"msg":"hello"}}`)))
		w := httptest.NewRecorder()

		handler.Broadcast(w, req)

		if w.Result().StatusCode != http.StatusOK {
			t.Errorf("Expected status OK, got %d", w.Result().StatusCode)
		}
	})

	t.Run("Valid Broadcast Raw Fallback", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodPost, "/api/v1/mesh/broadcast", bytes.NewBuffer([]byte(`just some raw data`)))
		w := httptest.NewRecorder()

		handler.Broadcast(w, req)

		if w.Result().StatusCode != http.StatusOK {
			t.Errorf("Expected status OK, got %d", w.Result().StatusCode)
		}
	})

	t.Run("Invalid Method", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/api/v1/mesh/broadcast", nil)
		w := httptest.NewRecorder()

		handler.Broadcast(w, req)

		if w.Result().StatusCode != http.StatusMethodNotAllowed {
			t.Errorf("Expected status MethodNotAllowed, got %d", w.Result().StatusCode)
		}
	})

	t.Run("Publish Error", func(t *testing.T) {
		failHandler := NewMeshHandler(&mockFailingTransport{})
		req := httptest.NewRequest(http.MethodPost, "/api/v1/mesh/broadcast", bytes.NewBuffer([]byte(`{}`)))
		w := httptest.NewRecorder()

		failHandler.Broadcast(w, req)

		if w.Result().StatusCode != http.StatusInternalServerError {
			t.Errorf("Expected status InternalServerError, got %d", w.Result().StatusCode)
		}
	})
}

func TestMeshHandler_Subscribe(t *testing.T) {
	transport := mesh.NewMemoryTransport()
	handler := NewMeshHandler(transport)

	server := httptest.NewServer(http.HandlerFunc(handler.Subscribe))
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http") + "?channel=test"

	t.Run("Valid Subscribe", func(t *testing.T) {
		ws, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
		if err != nil {
			t.Fatalf("Could not open websocket connection: %v", err)
		}
		defer ws.Close()

		// Wait briefly to ensure the server has time to upgrade the connection
		// and subscribe to the transport channel before we publish.
		time.Sleep(50 * time.Millisecond)

		// Publish a message to see if it's received
		msgStr := "hello websocket"
		err = transport.Publish(context.Background(), "test", []byte(msgStr))
		if err != nil {
			t.Fatalf("Failed to publish: %v", err)
		}

		ws.SetReadDeadline(time.Now().Add(1 * time.Second))
		_, msg, err := ws.ReadMessage()
		if err != nil {
			t.Fatalf("Failed to read message: %v", err)
		}

		if string(msg) != msgStr {
			t.Errorf("Expected %s, got %s", msgStr, msg)
		}
	})

	t.Run("Invalid Method", func(t *testing.T) {
		resp, err := http.Post(server.URL, "application/json", nil)
		if err != nil {
			t.Fatalf("Failed to make POST request: %v", err)
		}
		if resp.StatusCode != http.StatusMethodNotAllowed {
			t.Errorf("Expected status MethodNotAllowed, got %d", resp.StatusCode)
		}
	})

	t.Run("Subscribe Error", func(t *testing.T) {
		failHandler := NewMeshHandler(&mockFailingTransport{})
		failServer := httptest.NewServer(http.HandlerFunc(failHandler.Subscribe))
		defer failServer.Close()

		failWsURL := "ws" + strings.TrimPrefix(failServer.URL, "http")

		// It might return an error during upgrade or connect and write the error message
		ws, _, err := websocket.DefaultDialer.Dial(failWsURL, nil)
		if err != nil {
			// This is fine, upgrade failed
			return
		}
		defer ws.Close()

		ws.SetReadDeadline(time.Now().Add(1 * time.Second))
		_, msg, err := ws.ReadMessage()
		if err == nil && !strings.Contains(string(msg), "Failed to subscribe") {
			t.Errorf("Expected error message, got: %s", string(msg))
		}
	})
}
