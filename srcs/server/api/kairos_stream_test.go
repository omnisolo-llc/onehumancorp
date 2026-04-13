package api

import (
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestKairosStreamHandler_ServeWS(t *testing.T) {
	provider, err := db.NewSQLiteProvider(":memory:")
	if err != nil {
		t.Fatalf("failed to create db provider: %v", err)
	}
	defer provider.Close()

	tm := orchestration.NewLocalTeammateMesh(provider)
	handler := NewKairosStreamHandler(tm)

	server := httptest.NewServer(http.HandlerFunc(handler.ServeWS))
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http")
	ws, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("could not open websocket: %v", err)
	}
	defer ws.Close()

	ctx := context.Background()

	// Wait a tiny bit for subscriptions to register
	time.Sleep(100 * time.Millisecond)

	tm.Publish(ctx, "mesh:tasks", []byte("task1"))

	ws.SetReadDeadline(time.Now().Add(2 * time.Second))
	_, msg, err := ws.ReadMessage()
	if err != nil {
		t.Fatalf("could not read message: %v", err)
	}

	expected := `{"payload":"task1","type":"mesh:tasks"}`
	if string(msg) != expected {
		t.Fatalf("expected %v, got %v", expected, string(msg))
	}
}
