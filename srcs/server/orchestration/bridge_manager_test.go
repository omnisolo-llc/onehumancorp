package orchestration

import (
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

func TestBridgeManager_Connect(t *testing.T) {
	telemetry.InitTelemetry()
	prov := db.NewTestProvider(t)
	defer prov.Close()

	hub := NewCentrifugeNode("mem://")
	bm := NewBridgeManager(prov, hub)

	upgrader := websocket.Upgrader{}
	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		conn, _ := upgrader.Upgrade(w, r, nil)
		defer conn.Close()
		for {
			_, _, err := conn.ReadMessage()
			if err != nil {
				break
			}
		}
	}))
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http")

	ctx := context.Background()
	err := bm.Connect(ctx, "org1", wsURL, "org2")
	if err != nil {
		t.Fatalf("expected connect to succeed, got %v", err)
	}

	bm.mu.Lock()
	_, ok := bm.bridges["org2"]
	bm.mu.Unlock()

	if !ok {
		t.Fatalf("expected bridge connection to be saved")
	}

	err = bm.ForwardEvent(ctx, "org1", "org2", map[string]interface{}{"event": "test"})
	if err != nil {
		t.Fatalf("failed to forward event: %v", err)
	}
}
