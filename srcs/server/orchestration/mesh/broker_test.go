package mesh

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

type mockBroker struct {
	channel string
	payload []byte
}

func (m *mockBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	m.channel = channel
	m.payload = append([]byte(nil), payload...)
	return nil
}

func TestBroadcastHandler_ServeHTTP(t *testing.T) {
	broker := &mockBroker{}
	handler := NewBroadcastHandler(broker)

	reqData := BroadcastRequest{
		Channel:   "mesh:tasks",
		EventType: "TASK_TRANSITION",
		Data:      json.RawMessage(`{"task_id":"123"}`),
	}
	bodyBytes, _ := json.Marshal(reqData)

	req := httptest.NewRequest(http.MethodPost, "/api/mesh/v2/broadcast", bytes.NewReader(bodyBytes))
	w := httptest.NewRecorder()

	handler.ServeHTTP(w, req)

	res := w.Result()
	if res.StatusCode != http.StatusOK {
		t.Errorf("Expected status OK, got %v", res.StatusCode)
	}

	if broker.channel != "mesh:tasks" {
		t.Errorf("Expected channel mesh:tasks, got %s", broker.channel)
	}

	if string(broker.payload) != `{"task_id":"123"}` {
		t.Errorf("Expected payload, got %s", string(broker.payload))
	}
}

func TestBroadcastHandler_MethodNotAllowed(t *testing.T) {
	broker := &mockBroker{}
	handler := NewBroadcastHandler(broker)
	req := httptest.NewRequest(http.MethodGet, "/api/mesh/v2/broadcast", nil)
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)
	res := w.Result()
	if res.StatusCode != http.StatusMethodNotAllowed {
		t.Errorf("Expected status Method Not Allowed, got %v", res.StatusCode)
	}
}

func TestBroadcastHandler_BadRequest(t *testing.T) {
	broker := &mockBroker{}
	handler := NewBroadcastHandler(broker)
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/v2/broadcast", bytes.NewReader([]byte(`{"invalid"}`)))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)
	res := w.Result()
	if res.StatusCode != http.StatusBadRequest {
		t.Errorf("Expected status Bad Request, got %v", res.StatusCode)
	}
}

func TestBroadcastHandler_PayloadTooLarge(t *testing.T) {
	broker := &mockBroker{}
	handler := NewBroadcastHandler(broker)

	largeData := make([]byte, 1024*1024*2) // 2MB
	for i := range largeData {
		largeData[i] = 'a'
	}

	reqData := BroadcastRequest{
		Channel:   "mesh:tasks",
		EventType: "TASK_TRANSITION",
		Data:      json.RawMessage(`"` + string(largeData) + `"`),
	}
	bodyBytes, _ := json.Marshal(reqData)

	req := httptest.NewRequest(http.MethodPost, "/api/mesh/v2/broadcast", bytes.NewReader(bodyBytes))
	w := httptest.NewRecorder()
	handler.ServeHTTP(w, req)
	res := w.Result()
	if res.StatusCode != http.StatusRequestEntityTooLarge {
		t.Errorf("Expected status Request Entity Too Large, got %v", res.StatusCode)
	}
}

func TestLocalMeshBroker_Broadcast(t *testing.T) {
	broker := NewLocalMeshBroker()

	ch := make(chan []byte, 1)
	broker.Subscribe("mesh:tasks", ch)

	err := broker.Broadcast(context.Background(), "mesh:tasks", []byte(`{"test":"data"}`))
	if err != nil {
		t.Errorf("Broadcast failed: %v", err)
	}

	select {
	case msg := <-ch:
		if string(msg) != `{"test":"data"}` {
			t.Errorf("Expected msg `{\"test\":\"data\"}`, got %s", string(msg))
		}
	case <-time.After(1 * time.Second):
		t.Errorf("Timed out waiting for message")
	}
}
