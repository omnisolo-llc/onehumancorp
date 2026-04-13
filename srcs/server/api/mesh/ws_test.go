package mesh

import (
	"context"
	"crypto/x509"
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/peer"
)

func mockSPIFFEContext(ctx context.Context, spiffeID string) context.Context {
	parsedURL, _ := url.Parse(spiffeID)
	tlsInfo := credentials.TLSInfo{
		State: credentials.TLSChannelConnectionState{
			PeerCertificates: []*x509.Certificate{
				{
					URIs: []*url.URL{parsedURL},
				},
			},
		},
	}
	p := &peer.Peer{
		AuthInfo: tlsInfo,
	}
	return peer.NewContext(ctx, p)
}

func TestWSHandler_Unauthorized(t *testing.T) {
	pubsub := NewMemoryPubSub()
	handler := NewWSHandler(pubsub)

	server := httptest.NewServer(handler)
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http") + "?topic=test"
	_, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err == nil {
		t.Fatal("Expected error dialing websocket without SPIFFE ID, got none")
	}
}

func TestWSHandler_Success(t *testing.T) {
	pubsub := NewMemoryPubSub()
	handler := NewWSHandler(pubsub)

	// Since we need to inject the mock context for the HTTP request,
	// we create a custom handler that wraps the WS handler.
	wrapper := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		ctx := mockSPIFFEContext(r.Context(), "spiffe://example.org/agent/123")
		handler.ServeHTTP(w, r.WithContext(ctx))
	})

	server := httptest.NewServer(wrapper)
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http") + "?topic=test"
	conn, _, err := websocket.DefaultDialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("Failed to dial websocket: %v", err)
	}
	defer conn.Close()

	// Wait briefly to allow subscribe
	time.Sleep(100 * time.Millisecond)

	// Publish message
	err = pubsub.Publish(context.Background(), "test", []byte("hello ws"))
	if err != nil {
		t.Fatalf("Failed to publish: %v", err)
	}

	// Read message
	conn.SetReadDeadline(time.Now().Add(1 * time.Second))
	msgType, p, err := conn.ReadMessage()
	if err != nil {
		t.Fatalf("Failed to read message: %v", err)
	}

	if msgType != websocket.TextMessage {
		t.Errorf("Expected text message, got %d", msgType)
	}

	if string(p) != "hello ws" {
		t.Errorf("Expected 'hello ws', got '%s'", string(p))
	}
}
