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

func TestHandleWS(t *testing.T) {
	pubsub := NewMemoryPubSub()
	handler := HandleWS(pubsub)

	s := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		claims := &auth.Claims{Subject: "test-agent"}
		ctx := context.WithValue(r.Context(), auth.ClaimsContextKeyForTest, claims)
		handler.ServeHTTP(w, r.WithContext(ctx))
	}))
	defer s.Close()

	wsURL := "ws" + strings.TrimPrefix(s.URL, "http") + "?topic=global"
	dialer := websocket.Dialer{}
	conn, _, err := dialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("Failed to dial WebSocket: %v", err)
	}
	defer conn.Close()

	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{Subject: "test-agent"})

	// Ensure subscriber is registered
	time.Sleep(100 * time.Millisecond)

	err = pubsub.Publish(ctx, "global", []byte(`{"test":"true"}`))
	if err != nil {
		t.Fatalf("Failed to broadcast: %v", err)
	}

	conn.SetReadDeadline(time.Now().Add(time.Second))
	_, msg, err := conn.ReadMessage()
	if err != nil {
		t.Fatalf("Failed to read message: %v", err)
	}

	if string(msg) != `{"test":"true"}` {
		t.Errorf("Expected message, got %s", string(msg))
	}
}
