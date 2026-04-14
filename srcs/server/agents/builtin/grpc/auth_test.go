package agentgrpc_test

import (
	"context"
	"testing"

	agentgrpc "github.com/onehumancorp/mono/srcs/server/agents/builtin/grpc"
	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/metadata"
	"google.golang.org/grpc/status"
	"net"
)

// startAuthTestServer creates a test server with the given AuthConfig applied.
func startAuthTestServer(t *testing.T, authCfg agentgrpc.AuthConfig) (*grpc.ClientConn, func()) {
	t.Helper()
	lis, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("listen: %v", err)
	}

	srv := grpc.NewServer(
		grpc.ChainUnaryInterceptor(authCfg.UnaryInterceptor()),
		grpc.ChainStreamInterceptor(authCfg.StreamInterceptor()),
	)
	svc := agentgrpc.NewAgentServiceServer("auth-test", agentgrpc.AgentConfig{}, nil)
	agentservicepb.RegisterAgentServiceServer(srv, svc)

	go func() { _ = srv.Serve(lis) }()

	conn, err := grpc.NewClient(lis.Addr().String(),
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	)
	if err != nil {
		t.Fatalf("dial: %v", err)
	}

	return conn, func() {
		conn.Close()
		srv.Stop()
	}
}

// TestAuth_Disabled verifies that auth disabled allows any call through.
func TestAuth_Disabled(t *testing.T) {
	authCfg := agentgrpc.AuthConfig{} // mode=disabled via zero value
	// Force disabled explicitly:
	t.Setenv("OHC_AGENT_AUTH_DISABLED", "true")
	authCfg = agentgrpc.AuthConfigFromEnv()

	conn, cleanup := startAuthTestServer(t, authCfg)
	defer cleanup()

	client := agentservicepb.NewAgentServiceClient(conn)
	_, err := client.Ping(context.Background(), &agentservicepb.PingRequest{})
	if err != nil {
		t.Fatalf("Ping with auth disabled failed: %v", err)
	}
}

// TestAuth_Token_Valid verifies that a correct bearer token passes.
func TestAuth_Token_Valid(t *testing.T) {
	const secret = "test-secret-123"
	t.Setenv("OHC_AGENT_TOKEN", secret)
	t.Setenv("OHC_AGENT_AUTH_DISABLED", "")
	authCfg := agentgrpc.AuthConfigFromEnv()

	conn, cleanup := startAuthTestServer(t, authCfg)
	defer cleanup()

	ctx := metadata.NewOutgoingContext(context.Background(), metadata.Pairs(
		"authorization", "Bearer "+secret,
	))
	client := agentservicepb.NewAgentServiceClient(conn)
	_, err := client.Ping(ctx, &agentservicepb.PingRequest{})
	if err != nil {
		t.Fatalf("Ping with valid token failed: %v", err)
	}
}

// TestAuth_Token_Invalid verifies that a wrong token is rejected.
func TestAuth_Token_Invalid(t *testing.T) {
	const secret = "correct-secret"
	t.Setenv("OHC_AGENT_TOKEN", secret)
	t.Setenv("OHC_AGENT_AUTH_DISABLED", "")
	authCfg := agentgrpc.AuthConfigFromEnv()

	conn, cleanup := startAuthTestServer(t, authCfg)
	defer cleanup()

	ctx := metadata.NewOutgoingContext(context.Background(), metadata.Pairs(
		"authorization", "Bearer wrong-secret",
	))
	client := agentservicepb.NewAgentServiceClient(conn)
	_, err := client.Ping(ctx, &agentservicepb.PingRequest{})
	if err == nil {
		t.Fatal("expected error with invalid token, got nil")
	}
	if code := status.Code(err); code != codes.Unauthenticated {
		t.Errorf("expected Unauthenticated, got %v", code)
	}
}

// TestAuth_Token_Missing verifies that a missing header is rejected.
func TestAuth_Token_Missing(t *testing.T) {
	const secret = "some-secret"
	t.Setenv("OHC_AGENT_TOKEN", secret)
	t.Setenv("OHC_AGENT_AUTH_DISABLED", "")
	authCfg := agentgrpc.AuthConfigFromEnv()

	conn, cleanup := startAuthTestServer(t, authCfg)
	defer cleanup()

	// No metadata at all.
	client := agentservicepb.NewAgentServiceClient(conn)
	_, err := client.Ping(context.Background(), &agentservicepb.PingRequest{})
	if err == nil {
		t.Fatal("expected error without token, got nil")
	}
	if code := status.Code(err); code != codes.Unauthenticated {
		t.Errorf("expected Unauthenticated, got %v", code)
	}
}

// TestAuth_SPIFFE_NoTLS verifies that SPIFFE mode rejects plaintext connections
// (no peer cert available).
func TestAuth_SPIFFE_NoTLS(t *testing.T) {
	// Force SPIFFE mode by clearing the token env var.
	t.Setenv("OHC_AGENT_TOKEN", "")
	t.Setenv("OHC_AGENT_AUTH_DISABLED", "")
	authCfg := agentgrpc.AuthConfigFromEnv()

	conn, cleanup := startAuthTestServer(t, authCfg)
	defer cleanup()

	client := agentservicepb.NewAgentServiceClient(conn)
	_, err := client.Ping(context.Background(), &agentservicepb.PingRequest{})
	if err == nil {
		t.Fatal("expected error with no TLS peer cert, got nil")
	}
	if code := status.Code(err); code != codes.Unauthenticated {
		t.Errorf("expected Unauthenticated, got %v (%v)", code, err)
	}
}
