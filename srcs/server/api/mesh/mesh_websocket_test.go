package mesh

import (
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestWebSocketMeshHandler(t *testing.T) {
	svc := NewMemoryMeshService()
	handler := NewWebSocketMeshHandler(svc)

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		ctx := context.WithValue(r.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})
		handler.HandleWebSocket(w, r.WithContext(ctx))
	}))
	defer server.Close()

	url := "ws" + strings.TrimPrefix(server.URL, "http")
	conn, _, err := websocket.DefaultDialer.Dial(url, nil)
	if err != nil {
		t.Fatalf("failed to dial websocket: %v", err)
	}
	defer conn.Close()

	// Send a message
	err = conn.WriteMessage(websocket.TextMessage, []byte("test-intent"))
	if err != nil {
		t.Fatalf("failed to write message: %v", err)
	}

	// Wait and read it back
	conn.SetReadDeadline(time.Now().Add(2 * time.Second))
	_, msg, err := conn.ReadMessage()
	if err != nil {
		t.Fatalf("failed to read message: %v", err)
	}

	if string(msg) != "test-intent" {
		t.Errorf("expected 'test-intent', got '%s'", string(msg))
	}
}
