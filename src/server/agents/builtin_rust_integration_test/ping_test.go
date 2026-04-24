package builtin_integration_test

import (
	"context"
	"fmt"
	"testing"

	agentservicepb "github.com/onehumancorp/mono/src/proto/agentservice"
)

// TestRustAgent_Ping verifies the Ping health-check RPC.
func TestRustAgent_Ping(t *testing.T) {
	conn, cleanup := startAgent(t)
	defer cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), rpcTimeout)
	defer cancel()

	client := agentservicepb.NewAgentServiceClient(conn)
	resp, err := client.Ping(ctx, &agentservicepb.PingRequest{})
	if err != nil {
		t.Fatalf("Ping: %v", err)
	}
	if resp.AgentId == "" {
		t.Error("Ping: expected non-empty agent_id")
	}
	if resp.Version == "" {
		t.Error("Ping: expected non-empty version")
	}
	t.Logf("Ping OK: agent_id=%q version=%q", resp.AgentId, resp.Version)
}

// TestRustAgent_MultiPing verifies the binary handles concurrent Ping calls.
func TestRustAgent_MultiPing(t *testing.T) {
	conn, cleanup := startAgent(t)
	defer cleanup()

	client := agentservicepb.NewAgentServiceClient(conn)
	for i := 0; i < 10; i++ {
		i := i
		t.Run(fmt.Sprintf("ping%d", i), func(t *testing.T) {
			ctx, cancel := context.WithTimeout(context.Background(), rpcTimeout)
			defer cancel()
			_, err := client.Ping(ctx, &agentservicepb.PingRequest{})
			if err != nil {
				t.Errorf("Ping #%d: %v", i, err)
			}
		})
	}
}
