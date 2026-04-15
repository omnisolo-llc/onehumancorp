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

func TestHandleWebSocket(t *testing.T) {
	ctx := context.Background()
	claims := &auth.Claims{Subject: "test-user"}
	ctx = context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	meshService := NewMemoryMeshService()

	handler := HandleWebSocket(meshService)

	// Wrap handler to inject context
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		r = r.WithContext(ctx)
		handler(w, r)
	}))
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http")

	ws, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("could not open a ws connection: %v", err)
	}
	defer ws.Close()

	// Allow time for subscription to register
	time.Sleep(50 * time.Millisecond)

	testMsg := "hello mesh"
	err = meshService.BroadcastIntent(ctx, testMsg)
	if err != nil {
		t.Fatalf("failed to broadcast intent: %v", err)
	}

	ws.SetReadDeadline(time.Now().Add(1 * time.Second))
	_, msg, err := ws.ReadMessage()
	if err != nil {
		t.Fatalf("could not read message from ws: %v", err)
	}

	if string(msg) != testMsg {
		t.Errorf("expected %q, got %q", testMsg, string(msg))
	}
}
