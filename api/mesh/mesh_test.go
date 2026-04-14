package mesh

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
)

func TestHandleBroadcast_ValidPayload(t *testing.T) {
	transport := NewMemoryMeshTransport()
	handler := NewMeshHandler(transport)

	payload := []byte(`{"agent_id": "link", "action": "scan", "status": "active"}`)
	req, err := http.NewRequest("POST", "/api/mesh/broadcast", bytes.NewBuffer(payload))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler.HandleBroadcast(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}

func TestHandleBroadcast_MissingFields(t *testing.T) {
	transport := NewMemoryMeshTransport()
	handler := NewMeshHandler(transport)

	payload := []byte(`{"agent_id": "link", "status": "active"}`)
	req, err := http.NewRequest("POST", "/api/mesh/broadcast", bytes.NewBuffer(payload))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler.HandleBroadcast(rr, req)

	if status := rr.Code; status != http.StatusBadRequest {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusBadRequest)
	}
}

func TestHandleSubscribe_SSE(t *testing.T) {
	transport := NewMemoryMeshTransport()
	handler := NewMeshHandler(transport)

	ctx, cancel := context.WithCancel(context.Background())
	req, err := http.NewRequestWithContext(ctx, "GET", "/api/mesh/subscribe", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()

	// Start subscriber in a goroutine
	go handler.HandleSubscribe(rr, req)

	// Wait for subscriber to attach
	time.Sleep(50 * time.Millisecond)

	// Broadcast a message
	payload := []byte(`{"agent_id": "link", "action": "ping", "status": "active"}`)
	bReq, err := http.NewRequest("POST", "/api/mesh/broadcast", bytes.NewBuffer(payload))
	if err != nil {
		t.Fatal(err)
	}
	bHandler := http.HandlerFunc(handler.HandleBroadcast)
	bHandler.ServeHTTP(httptest.NewRecorder(), bReq)

	// Give time for the message to propagate
	time.Sleep(50 * time.Millisecond)

	// Cancel the context to stop the subscriber
	cancel()

	body := rr.Body.String()
	if !strings.Contains(body, "data: {\"agent_id\":\"link\",\"action\":\"ping\",\"status\":\"active\"}") {
		t.Errorf("expected body to contain payload, got: %s", body)
	}
}

func TestHandleSubscribe_WebSocket(t *testing.T) {
	transport := NewMemoryMeshTransport()
	handler := NewMeshHandler(transport)

	server := httptest.NewServer(http.HandlerFunc(handler.HandleSubscribe))
	defer server.Close()

	url := "ws" + strings.TrimPrefix(server.URL, "http")
	ws, _, err := websocket.DefaultDialer.Dial(url, nil)
	if err != nil {
		t.Fatalf("could not open a ws connection on %s %v", url, err)
	}
	defer ws.Close()

	// Give it time to attach
	time.Sleep(50 * time.Millisecond)

	// Broadcast a message via transport directly so we don't have networking race conditions
	msg := SIPPayload{
		AgentID: "link",
		Action:  "ping",
		Status:  "active",
	}
	transport.Broadcast(context.Background(), msg)

	ws.SetReadDeadline(time.Now().Add(1 * time.Second))
	var receivedMsg SIPPayload
	err = ws.ReadJSON(&receivedMsg)
	if err != nil {
		t.Fatalf("could not read json: %v", err)
	}

	if receivedMsg.AgentID != "link" || receivedMsg.Action != "ping" || receivedMsg.Status != "active" {
		t.Errorf("received incorrect message: %+v", receivedMsg)
	}

	ws.WriteMessage(websocket.CloseMessage, websocket.FormatCloseMessage(websocket.CloseNormalClosure, ""))
	time.Sleep(50 * time.Millisecond)
}
