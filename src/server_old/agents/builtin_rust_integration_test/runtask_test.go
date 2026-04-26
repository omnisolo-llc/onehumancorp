package builtin_integration_test

import (
	"context"
	"io"
	"testing"

	agentservicepb "github.com/onehumancorp/mono/src/proto/agentservice"
)

// TestRustAgent_RunTask_NoLLM starts a task with an empty prompt to verify
// the streaming events are emitted. Without a real LLM, the task will fail
// with a provider error — we just check the stream delivers events and closes.
func TestRustAgent_RunTask_NoLLM(t *testing.T) {
	conn, cleanup := startAgent(t)
	defer cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), rpcTimeout)
	defer cancel()

	client := agentservicepb.NewAgentServiceClient(conn)
	stream, err := client.RunTask(ctx, &agentservicepb.RunTaskRequest{
		Task:        "echo hello",
		Model:       "test",
		LlmProvider: "ollama",
		LlmEndpoint: "http://127.0.0.1:1", // invalid endpoint → fast fail
		MaxTokens:   16,
	})
	if err != nil {
		t.Fatalf("RunTask: %v", err)
	}

	var sawRunStarted bool
	for {
		evt, err := stream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			// gRPC error from server side is acceptable (LLM provider failed)
			t.Logf("stream error (expected with no LLM): %v", err)
			break
		}
		t.Logf("event: type=%v", evt.Type)
		if evt.Type == agentservicepb.EventType_RUN_STARTED {
			sawRunStarted = true
		}
		// TASK_ERROR or TASK_COMPLETE terminate the stream.
		if evt.Type == agentservicepb.EventType_TASK_ERROR ||
			evt.Type == agentservicepb.EventType_TASK_COMPLETE {
			break
		}
	}
	if !sawRunStarted {
		t.Error("did not receive RUN_STARTED event")
	}
}
