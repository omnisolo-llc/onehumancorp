package builtin_integration_test

import (
	"context"
	"testing"

	agentservicepb "github.com/onehumancorp/mono/src/proto/agentservice"
)

// TestRustAgent_DispatchToSubAgent_InProcess verifies in-process sub-agent
// dispatch (empty sub_agent_address). With an invalid LLM endpoint the
// DispatchToSubAgent RPC should still return a SubAgentResponse (with an error
// field) rather than a gRPC error.
func TestRustAgent_DispatchToSubAgent_InProcess(t *testing.T) {
	conn, cleanup := startAgent(t)
	defer cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), rpcTimeout)
	defer cancel()

	client := agentservicepb.NewAgentServiceClient(conn)
	resp, err := client.DispatchToSubAgent(ctx, &agentservicepb.SubAgentRequest{
		Task:        "noop",
		Model:       "test",
		LlmProvider: "ollama",
		// Empty sub_agent_address → in-process dispatch.
	})
	if err != nil {
		t.Fatalf("DispatchToSubAgent: %v", err)
	}
	// Response must be non-nil; error field is expected (no valid LLM).
	if resp == nil {
		t.Fatal("nil response")
	}
	t.Logf("DispatchToSubAgent OK: result=%q error=%q", resp.Result, resp.Error)
}
