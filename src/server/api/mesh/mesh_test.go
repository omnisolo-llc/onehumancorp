package mesh

import (
	"bytes"
	"context"
	"github.com/gorilla/websocket"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric/noop"
)

func TestMain(m *testing.M) {
	provider := noop.NewMeterProvider()
	otel.SetMeterProvider(provider)
	m.Run()
}

func TestMemoryMeshService(t *testing.T) {
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})
	svc := NewMemoryMeshService()

	sub, err := svc.Subscribe(ctx)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	err = svc.BroadcastIntent(ctx, "hello")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	select {
	case msg := <-sub:
		if msg != "hello" {
			t.Errorf("expected 'hello', got '%s'", msg)
		}
	case <-time.After(1 * time.Second):
		t.Error("timeout waiting for message")
	}
}

func TestAuthErrors(t *testing.T) {
	ctx := context.Background() // No claims
	svc := NewMemoryMeshService()

	err := svc.BroadcastIntent(ctx, "hello")
	if err == nil {
		t.Error("expected unauthorized error")
	}

	_, err = svc.Subscribe(ctx)
	if err == nil {
		t.Error("expected unauthorized error")
	}
}

func TestMeshHandlerBroadcastSIP(t *testing.T) {
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})
	svc := NewMemoryMeshService()
	handler := NewMeshHandler(svc)

	reqBody := []byte(`{"agent_id":"xyz","channel":"mesh:tasks","event_type":"TASK_TRANSITION","data":{"task_id":"123"}}`)
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer(reqBody))
	req = req.WithContext(ctx)
	w := httptest.NewRecorder()

	handler.Broadcast(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status %d, got %d", http.StatusOK, w.Code)
	}
}

func TestMeshHandlerBroadcast(t *testing.T) {
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})
	svc := NewMemoryMeshService()
	handler := NewMeshHandler(svc)

	reqBody := []byte(`{"agent_id":"test", "channel":"test", "event_type":"test", "data": {"intent":"hello handler"}}`)
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBuffer(reqBody))
	req = req.WithContext(ctx)
	w := httptest.NewRecorder()

	handler.Broadcast(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status %d, got %d", http.StatusOK, w.Code)
	}
}

func TestMeshHandlerStream(t *testing.T) {
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})
	svc := NewMemoryMeshService()
	handler := NewMeshHandler(svc)

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		r = r.WithContext(ctx)
		handler.Stream(w, r)
	}))
	defer server.Close()

	url := "ws" + strings.TrimPrefix(server.URL, "http")

	// Create a channel to catch the error from broadcast since Stream blocks
	errCh := make(chan error, 1)

	// Stream handles reading and writing for the duration of the request, wait a little before broadcasting
	go func() {
		time.Sleep(100 * time.Millisecond)
		errCh <- svc.BroadcastIntent(ctx, "hello stream")
	}()

	conn, _, err := websocket.DefaultDialer.Dial(url, nil)
	if err != nil {
		t.Fatalf("could not dial websocket: %v", err)
	}
	defer conn.Close()

	err = <-errCh
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	conn.SetReadDeadline(time.Now().Add(1 * time.Second))
	_, msg, err := conn.ReadMessage()
	if err != nil {
		t.Fatalf("could not read message: %v", err)
	}

	if string(msg) != "hello stream" {
		t.Errorf("expected 'hello stream', got '%s'", string(msg))
	}
}

func TestMeshHandlerBroadcastEvent(t *testing.T) {
    service := NewMemoryMeshService()
    handler := NewMeshHandler(service)

    body := `{"agent_id": "worker-1", "channel": "orchestration.tasks", "event_type": "TaskTransition", "data": {"status": "success"}}`
    req := httptest.NewRequest(http.MethodPost, "/api/v1/mesh/broadcast", bytes.NewBufferString(body))

    // Add auth claims to context
    claims := &auth.Claims{OrganizationID: "org-1"}
    ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
    req = req.WithContext(ctx)

    w := httptest.NewRecorder()

    handler.Broadcast(w, req)

    if w.Code != http.StatusOK {
        t.Errorf("expected status %d, got %d", http.StatusOK, w.Code)
    }
}

func TestMeshHandlerPublish(t *testing.T) {
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})
	svc := NewMemoryMeshService()
	handler := NewMeshHandler(svc)

	reqBody := []byte(`{"message":"hello publish"}`)
	req := httptest.NewRequest(http.MethodPost, "/mesh/publish", bytes.NewBuffer(reqBody))
	req = req.WithContext(ctx)
	w := httptest.NewRecorder()

	handler.Publish(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("expected status %d, got %d", http.StatusOK, w.Code)
	}
}

func TestMeshHandlerSubscribe(t *testing.T) {
	ctx := context.WithValue(context.Background(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"})
	svc := NewMemoryMeshService()
	handler := NewMeshHandler(svc)

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		r = r.WithContext(ctx)
		handler.Subscribe(w, r)
	}))
	defer server.Close()

	url := "ws" + strings.TrimPrefix(server.URL, "http")

	errCh := make(chan error, 1)

	go func() {
		time.Sleep(100 * time.Millisecond)
		errCh <- svc.BroadcastIntent(ctx, "hello subscribe")
	}()

	conn, _, err := websocket.DefaultDialer.Dial(url, nil)
	if err != nil {
		t.Fatalf("could not dial websocket: %v", err)
	}
	defer conn.Close()

	err = <-errCh
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	conn.SetReadDeadline(time.Now().Add(1 * time.Second))
	_, msg, err := conn.ReadMessage()
	if err != nil {
		t.Fatalf("could not read message: %v", err)
	}

	if string(msg) != "hello subscribe" {
		t.Errorf("expected 'hello subscribe', got '%s'", string(msg))
	}
}

func TestMeshHandler_Broadcast_LegacyPayload(t *testing.T) {
	svc := NewMemoryMeshService()
	handler := NewMeshHandler(svc)

	payload := `{"intent": "test-legacy-intent"}`
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBufferString(payload))
	req = req.WithContext(context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"}))
	w := httptest.NewRecorder()

	handler.Broadcast(w, req)

	if w.Result().StatusCode != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Result().StatusCode)
	}
}

func TestMeshHandler_Broadcast_ValidSIPPayload(t *testing.T) {
	svc := NewMemoryMeshService()
	handler := NewMeshHandler(svc)

	payload := `{"agent_id": "agent-1", "channel": "ch-1", "event_type": "event-1", "data": {"key": "value"}}`
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBufferString(payload))
	req = req.WithContext(context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"}))
	w := httptest.NewRecorder()

	handler.Broadcast(w, req)

	if w.Result().StatusCode != http.StatusOK {
		t.Errorf("expected status 200, got %d", w.Result().StatusCode)
	}
}

func TestMeshHandler_Broadcast_InvalidSIPPayload(t *testing.T) {
	svc := NewMemoryMeshService()
	handler := NewMeshHandler(svc)

	// Missing agent_id
	payload := `{"channel": "ch-1", "event_type": "event-1", "data": {"key": "value"}}`
	req := httptest.NewRequest(http.MethodPost, "/api/mesh/broadcast", bytes.NewBufferString(payload))
	req = req.WithContext(context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, &auth.Claims{OrganizationID: "org-1"}))
	w := httptest.NewRecorder()

	handler.Broadcast(w, req)

	if w.Result().StatusCode != http.StatusBadRequest {
		t.Errorf("expected status 400, got %d", w.Result().StatusCode)
	}
}
