package mcpwebhooktunnel_test

import (
	"context"
	"crypto/tls"
	"crypto/x509"
	"net"
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

func TestLocalTunnelClient(t *testing.T) {
	listener := bufconn.Listen(bufSize)
	agentID := "test-agent"

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

	// Create mock server with mock SPIFFE verification
	server := grpc.NewServer(grpc.StreamInterceptor(mockPeerInterceptor))
	relay := mcp_webhook_tunnel.NewCloudRelay()
	mcp_webhook_tunnel.RegisterWebhookTunnelServer(server, relay)

	go func() {
		server.Serve(listener)
	}()
	defer server.Stop()

	dialOpts := []grpc.DialOption{
		grpc.WithContextDialer(func(context.Context, string) (net.Conn, error) {
			return listener.Dial()
		}),
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	}

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	// Use test sqlite DB
	provider := db.NewTestProvider(t)
	defer provider.Close()

	client, err := mcpwebhooktunnel.NewLocalTunnelClient(ctx, agentID, "bufnet", provider, dialOpts...)
	if err != nil {
		t.Fatalf("Failed to create client: %v", err)
	}
	defer client.Close()

	// Wait for the connection to be established and registered
	time.Sleep(50 * time.Millisecond)

	tools := client.ListTools()
	if len(tools) != 1 {
		t.Fatalf("Expected 1 tool, got %d", len(tools))
	}
	if tools[0].Name != "get_tunnel_status" {
		t.Errorf("Expected tool name 'get_tunnel_status', got '%s'", tools[0].Name)
	}

	// Send a payload through the relay
	payload := &mcp_webhook_tunnel.WebhookPayloadMessage{
		AgentId: agentID,
		Body: []byte(`{"test":true}`),
	}

	err = relay.HandleWebhook(context.Background(), payload)
	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	// Wait to receive the payload
	time.Sleep(50 * time.Millisecond)

	count := client.GetReceivedCount()
	if count != 1 {
		t.Fatalf("Expected 1 payload, got %d", count)
	}

	// Verify it was injected into SQLite
	rows, err := provider.Query(ctx, "SELECT count(*) FROM local_webhook_events WHERE agent_id = ?", agentID)
	if err != nil {
		t.Fatalf("Failed to query db: %v", err)
	}
	defer rows.Close()

	if rows.Next() {
		var dbCount int
		if err := rows.Scan(&dbCount); err != nil {
			t.Fatalf("Failed to scan db count: %v", err)
		}
		if dbCount != 1 {
			t.Fatalf("Expected 1 payload in db, got %d", dbCount)
		}
	} else {
		t.Fatal("Failed to retrieve row from local_webhook_events")
	}

	res, err := client.CallTool(ctx, "get_tunnel_status", nil)
	if err != nil {
		t.Fatalf("CallTool error: %v", err)
	}

	statusMap, ok := res.(map[string]interface{})
	if !ok {
		t.Fatalf("Expected map[string]interface{}, got %T", res)
	}
	if statusMap["status"] != "connected" {
		t.Errorf("Expected status 'connected', got '%v'", statusMap["status"])
	}
	if statusMap["received"] != 1 {
		t.Errorf("Expected 1 received, got '%v'", statusMap["received"])
	}

	_, err = client.CallTool(ctx, "unknown_tool", nil)
	if err == nil {
		t.Error("Expected error calling unknown tool, got nil")
	}
}
