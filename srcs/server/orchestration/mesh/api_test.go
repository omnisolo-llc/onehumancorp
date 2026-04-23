package mesh

import (
	"bytes"
	"context"
	"crypto/tls"
	"crypto/x509"
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
)

func createMockTLSRequest(method, urlPath string, body []byte, valid bool) *http.Request {
	req := httptest.NewRequest(method, urlPath, bytes.NewBuffer(body))
	if valid {
		req.TLS = &tls.ConnectionState{
			PeerCertificates: []*x509.Certificate{
				{URIs: []*url.URL{{Scheme: "spiffe"}}},
			},
		}
	} else {
		req.TLS = &tls.ConnectionState{}
	}
	return req
}

func TestHandleBroadcast(t *testing.T) {
	mesh := NewLocalMesh()
	api := NewMeshAPI(mesh)

	t.Run("ValidBroadcast", func(t *testing.T) {
		reqBody := `{"channel":"tasks","event":{"agent_id":"test_agent","action":"test_action","status":"test_status"}}`
		req := createMockTLSRequest(http.MethodPost, "/api/mesh/v2/broadcast", []byte(reqBody), true)
		rr := httptest.NewRecorder()

		api.HandleBroadcast(rr, req)

		if rr.Code != http.StatusOK {
			t.Errorf("expected status 200 OK, got %v", rr.Code)
		}
		if !strings.Contains(rr.Body.String(), `"status":"ok"`) {
			t.Errorf("expected body to contain status ok")
		}
	})

	t.Run("InvalidTLS", func(t *testing.T) {
		reqBody := `{"channel":"tasks","event":{"agent_id":"test_agent","action":"test_action","status":"test_status"}}`
		req := createMockTLSRequest(http.MethodPost, "/api/mesh/v2/broadcast", []byte(reqBody), false)
		rr := httptest.NewRecorder()

		api.HandleBroadcast(rr, req)

		if rr.Code != http.StatusForbidden {
			t.Errorf("expected status 403 Forbidden, got %v", rr.Code)
		}
	})
}

func TestHandleSubscribe(t *testing.T) {
	mesh := NewLocalMesh()
	api := NewMeshAPI(mesh)

	server := httptest.NewServer(http.HandlerFunc(api.HandleSubscribe))
	defer server.Close()

	wsURL := strings.Replace(server.URL, "http", "ws", 1) + "?channel=tasks"

	conn, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("Failed to dial websocket: %v", err)
	}
	defer conn.Close()

	err = mesh.Publish(context.Background(), "tasks", []byte("hello websocket"))
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	conn.SetReadDeadline(time.Now().Add(2 * time.Second))
	_, msg, err := conn.ReadMessage()
	if err != nil {
		t.Fatalf("Failed to read message: %v", err)
	}
	if string(msg) != "hello websocket" {
		t.Errorf("expected message 'hello websocket', got %s", string(msg))
	}
}

func TestLatencyGuarantees(t *testing.T) {
	mesh := NewLocalMesh()

	start := time.Now()
	err := mesh.Publish(context.Background(), "tasks", []byte("ping"))
	duration := time.Since(start)

	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}
	if duration >= 5*time.Millisecond {
		t.Errorf("Publish latency too high: %v", duration)
	}
}
