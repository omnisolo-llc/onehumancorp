package api

import (
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestKairosStreamHandler_NoHub(t *testing.T) {
	handler := KairosStreamHandler(nil)
	server := httptest.NewServer(handler)
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http")
	conn, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("failed to connect: %v", err)
	}
	defer conn.Close()

	conn.SetReadDeadline(time.Now().Add(1 * time.Second))
	_, msg, err := conn.ReadMessage()
	if err != nil {
		t.Fatalf("expected error message, got error reading: %v", err)
	}
	if !strings.Contains(string(msg), "hub not configured") {
		t.Errorf("expected hub error, got: %s", msg)
	}
}

func TestKairosStreamHandler_HubButNoMesh(t *testing.T) {
	hub := orchestration.NewHub()
	handler := KairosStreamHandler(hub)
	server := httptest.NewServer(handler)
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http")
	conn, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("failed to connect: %v", err)
	}
	defer conn.Close()

	conn.SetReadDeadline(time.Now().Add(1 * time.Second))
	_, msg, err := conn.ReadMessage()
	if err != nil {
		t.Fatalf("expected error message, got error reading: %v", err)
	}
	if !strings.Contains(string(msg), "mesh not configured") {
		t.Errorf("expected mesh error, got: %s", msg)
	}
}
