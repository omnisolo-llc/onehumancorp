package mcp_webhook_tunnel_test

import (
	"bytes"
	"context"
	"crypto/tls"
	"crypto/x509"
	"net"
	"net/http"
	"net/http/httptest"
	"net/url"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/integrations/mcp_webhook_tunnel"
	"github.com/onehumancorp/mono/src/server/tools/mcpwebhooktunnel"
	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/peer"
	"google.golang.org/grpc/test/bufconn"
)

const bufSize = 1024 * 1024

func mockSpiffeCert(spiffeID string) *x509.Certificate {
	u, _ := url.Parse(spiffeID)
	return &x509.Certificate{
		URIs: []*url.URL{u},
	}
}

// wrappedStream wraps grpc.ServerStream to inject peer information into its context.
type wrappedStream struct {
	grpc.ServerStream
	ctx context.Context
}

func (w *wrappedStream) Context() context.Context {
	return w.ctx
}

func TestCloudRelay_RegisterAndForward(t *testing.T) {
	listener := bufconn.Listen(bufSize)
	agentID := "test-agent-123"

	mockTLSInfo := credentials.TLSInfo{
		State: tls.ConnectionState{
			PeerCertificates: []*x509.Certificate{
				mockSpiffeCert("spiffe://onehumancorp.com/agent/" + agentID),
			},
		},
	}

	mockPeerInterceptor := func(srv interface{}, ss grpc.ServerStream, info *grpc.StreamServerInfo, handler grpc.StreamHandler) error {
		p := &peer.Peer{AuthInfo: mockTLSInfo}
		ctx := peer.NewContext(ss.Context(), p)
		return handler(srv, &wrappedStream{ServerStream: ss, ctx: ctx})
	}

	server := grpc.NewServer(grpc.StreamInterceptor(mockPeerInterceptor))
	relay := mcp_webhook_tunnel.NewCloudRelay()
	mcp_webhook_tunnel.RegisterWebhookTunnelServer(server, relay)

	go func() {
		server.Serve(listener)
	}()
	defer server.Stop()

	// Dial using insecure credentials since the server intercepts and injects the peer TLS info
	dialOpts := []grpc.DialOption{
		grpc.WithContextDialer(func(context.Context, string) (net.Conn, error) {
			return listener.Dial()
		}),
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	provider := db.NewTestProvider(t)
	defer provider.Close()

	client, err := mcpwebhooktunnel.NewLocalTunnelClient(ctx, agentID, "bufnet", provider, dialOpts...)
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}
	defer client.Close()

	// Wait for the connection to be established and registered
	time.Sleep(200 * time.Millisecond)

	// Simulate an external webhook hitting the CloudRelay HTTP endpoint
	reqBody := []byte(`{"event":"order_created"}`)
	req := httptest.NewRequest(http.MethodPost, "/api/v1/relay/webhook/"+agentID, bytes.NewReader(reqBody))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	relay.ServeHTTP(w, req)

	if w.Result().StatusCode != http.StatusOK {
		t.Fatalf("Expected HTTP 200 OK, got %d", w.Result().StatusCode)
	}

	// Allow the stream to receive the payload
	time.Sleep(200 * time.Millisecond)

	count := client.GetReceivedCount()
	if count != 1 {
		t.Fatalf("Expected 1 payload on bus, got %d", count)
	}
}

func TestCloudRelay_LargePayload(t *testing.T) {
	relay := mcp_webhook_tunnel.NewCloudRelay()

	// 1.5MB payload
	reqBody := make([]byte, 1500000)
	req := httptest.NewRequest(http.MethodPost, "/api/v1/relay/webhook/test", bytes.NewReader(reqBody))
	w := httptest.NewRecorder()

	relay.ServeHTTP(w, req)

	if w.Result().StatusCode != http.StatusRequestEntityTooLarge {
		t.Fatalf("Expected HTTP 413, got %d", w.Result().StatusCode)
	}
}
