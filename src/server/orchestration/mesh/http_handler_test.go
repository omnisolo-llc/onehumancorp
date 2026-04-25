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

	meshpb "github.com/onehumancorp/mono/src/proto/ohc/mesh"
	"google.golang.org/protobuf/proto"
	"github.com/gorilla/websocket"
)

type mockMeshBroker struct {
	MeshBroker
	broadcastCalled bool
	lastChannel     string
	lastPayload     []byte
	subChan         chan []byte
}

func (m *mockMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	m.broadcastCalled = true
	m.lastChannel = channel
	m.lastPayload = payload
	return nil
}

type mockSubscription struct{}

func (s *mockSubscription) Close() error {
	return nil
}

func (m *mockMeshBroker) Subscribe(ctx context.Context, channel string, handler func(msg []byte)) (Subscription, error) {
	if m.subChan != nil {
		go func() {
			for {
				select {
				case <-ctx.Done():
					return
				case msg := <-m.subChan:
					handler(msg)
				}
			}
		}()
	}
	return &mockSubscription{}, nil
}

func createMockTLSRequest(method, urlStr string, body []byte, hasCert bool) *http.Request {
	req := httptest.NewRequest(method, urlStr, bytes.NewBuffer(body))
	if hasCert {
		req.TLS = &tls.ConnectionState{
			PeerCertificates: []*x509.Certificate{{
				URIs: []*url.URL{{Scheme: "spiffe"}},
			}},
		}
	} else {
		req.TLS = nil
	}
	return req
}

func TestHTTPHandler_HandleBroadcastV2(t *testing.T) {
	mockBroker := &mockMeshBroker{}
	handler := NewHTTPHandler(mockBroker)

	tests := []struct {
		name       string
		method     string
		body       []byte
		statusCode int
		hasCert    bool
	}{
		{"Method Not Allowed", http.MethodGet, nil, http.StatusMethodNotAllowed, true},
		{"Missing Cert", http.MethodPost, nil, http.StatusForbidden, false},
		{"Invalid Protobuf", http.MethodPost, []byte("invalid"), http.StatusBadRequest, true},
		{"Missing Channel", http.MethodPost, func() []byte { b, _ := proto.Marshal(&meshpb.MeshEvent{EventType: "TEST"}); return b }(), http.StatusBadRequest, true},
		{"Success", http.MethodPost, func() []byte { b, _ := proto.Marshal(&meshpb.MeshEvent{Channel: "test_channel"}); return b }(), http.StatusOK, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := createMockTLSRequest(tt.method, "/api/mesh/v2/broadcast", tt.body, tt.hasCert)
			w := httptest.NewRecorder()
			handler.HandleBroadcastV2(w, req)
			if w.Code != tt.statusCode {
				t.Errorf("expected %d, got %d", tt.statusCode, w.Code)
			}
		})
	}

	if !mockBroker.broadcastCalled {
		t.Errorf("expected Broadcast to be called")
	}
	if mockBroker.lastChannel != "test_channel" {
		t.Errorf("expected channel test_channel, got %s", mockBroker.lastChannel)
	}
}

func TestHTTPHandler_HandleSubscribeV2_Errors(t *testing.T) {
	mockBroker := &mockMeshBroker{}
	handler := NewHTTPHandler(mockBroker)

	req1 := createMockTLSRequest(http.MethodPost, "/api/mesh/v2/subscribe", nil, true)
	w1 := httptest.NewRecorder()
	handler.HandleSubscribeV2(w1, req1)
	if w1.Code != http.StatusMethodNotAllowed {
		t.Errorf("expected 405, got %d", w1.Code)
	}

	req2 := createMockTLSRequest(http.MethodGet, "/api/mesh/v2/subscribe", nil, false)
	w2 := httptest.NewRecorder()
	handler.HandleSubscribeV2(w2, req2)
	if w2.Code != http.StatusForbidden {
		t.Errorf("expected 403, got %d", w2.Code)
	}

	req3 := createMockTLSRequest(http.MethodGet, "/api/mesh/v2/subscribe", nil, true)
	w3 := httptest.NewRecorder()
	handler.HandleSubscribeV2(w3, req3)
	if w3.Code != http.StatusBadRequest {
		t.Errorf("expected 400, got %d", w3.Code)
	}
}

func handleSubscribeV2TestWrapper(h *HTTPHandler) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// Mock mTLS for the test
		r.TLS = &tls.ConnectionState{
			PeerCertificates: []*x509.Certificate{{
				URIs: []*url.URL{{Scheme: "spiffe"}},
			}},
		}
		h.HandleSubscribeV2(w, r)
	}
}

func TestHTTPHandler_HandleSubscribeV2_Success(t *testing.T) {
	mockBroker := &mockMeshBroker{
		subChan: make(chan []byte, 10),
	}
	handler := NewHTTPHandler(mockBroker)

	mux := http.NewServeMux()
	mux.HandleFunc("/api/mesh/v2/subscribe_test", handleSubscribeV2TestWrapper(handler))
	server2 := httptest.NewServer(mux)
	defer server2.Close()

    dialer := websocket.Dialer{
		TLSClientConfig: &tls.Config{InsecureSkipVerify: true},
	}
	wsURL2 := "ws" + strings.TrimPrefix(server2.URL, "http") + "/api/mesh/v2/subscribe_test?channel=test_sub_channel"
	ws, _, err := dialer.Dial(wsURL2, nil)
	if err != nil {
		t.Fatalf("failed to connect to websocket: %v", err)
	}
	defer ws.Close()

	expectedMsg := []byte("hello from mesh")
	mockBroker.subChan <- expectedMsg

	ws.SetReadDeadline(time.Now().Add(2 * time.Second))
	_, msg, err := ws.ReadMessage()
	if err != nil {
		t.Fatalf("failed to read message: %v", err)
	}

	if string(msg) != string(expectedMsg) {
		t.Errorf("expected %s, got %s", expectedMsg, msg)
	}
}
