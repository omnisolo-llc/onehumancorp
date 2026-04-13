package builtinclient

import (
	"context"
	"net"
	"testing"
	"time"

	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"google.golang.org/grpc"
	"google.golang.org/protobuf/proto"
)

type testAgentService struct {
	agentservicepb.UnimplementedAgentServiceServer
}

func (s *testAgentService) RunTask(req *agentservicepb.RunTaskRequest, stream agentservicepb.AgentService_RunTaskServer) error {
	textChunk := agentservicepb.EventType_TEXT_CHUNK
	if err := stream.Send(agentservicepb.RunTaskEvent_builder{
		Type:    &textChunk,
		Content: proto.String("working"),
	}.Build()); err != nil {
		return err
	}
	taskComplete := agentservicepb.EventType_TASK_COMPLETE
	return stream.Send(agentservicepb.RunTaskEvent_builder{
		Type:    &taskComplete,
		Content: proto.String("done:" + req.GetTask()),
	}.Build())
}

func (s *testAgentService) Ping(context.Context, *agentservicepb.PingRequest) (*agentservicepb.PingResponse, error) {
	return agentservicepb.PingResponse_builder{
		AgentId: proto.String("builtin-cpp-agent"),
		Version: proto.String("1.0.0"),
	}.Build(), nil
}

func (s *testAgentService) DispatchToSubAgent(context.Context, *agentservicepb.SubAgentRequest) (*agentservicepb.SubAgentResponse, error) {
	return agentservicepb.SubAgentResponse_builder{Result: proto.String("subagent-finished")}.Build(), nil
}

func startTestServer(t *testing.T) (string, func()) {
	t.Helper()

	listener, err := net.Listen("tcp", "127.0.0.1:0")
	if err != nil {
		t.Fatalf("listen: %v", err)
	}

	server := grpc.NewServer()
	agentservicepb.RegisterAgentServiceServer(server, &testAgentService{})
	go func() {
		_ = server.Serve(listener)
	}()

	return listener.Addr().String(), func() {
		server.Stop()
		_ = listener.Close()
	}
}

func TestClientPingAndWaitForReady(t *testing.T) {
	address, stop := startTestServer(t)
	defer stop()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	if err := WaitForReady(ctx, address, 10*time.Millisecond); err != nil {
		t.Fatalf("WaitForReady: %v", err)
	}

	client, err := DialContext(ctx, address)
	if err != nil {
		t.Fatalf("DialContext: %v", err)
	}
	defer func() { _ = client.Close() }()

	info, err := client.Ping(ctx)
	if err != nil {
		t.Fatalf("Ping: %v", err)
	}
	if info.AgentID != "builtin-cpp-agent" {
		t.Fatalf("unexpected agent id: %q", info.AgentID)
	}
}

func TestClientRunTaskAndDispatchToSubAgent(t *testing.T) {
	address, stop := startTestServer(t)
	defer stop()

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	client, err := DialContext(ctx, address)
	if err != nil {
		t.Fatalf("DialContext: %v", err)
	}
	defer func() { _ = client.Close() }()

	var events []agentservicepb.EventType
	result, err := client.RunTask(ctx, RunTaskRequest{Task: "ship feature"}, func(event *agentservicepb.RunTaskEvent) {
		events = append(events, event.GetType())
	})
	if err != nil {
		t.Fatalf("RunTask: %v", err)
	}
	if result != "done:ship feature" {
		t.Fatalf("unexpected RunTask result: %q", result)
	}
	if len(events) != 2 {
		t.Fatalf("expected 2 streamed events, got %d", len(events))
	}

	subResult, err := client.DispatchToSubAgent(ctx, SubAgentRequest{
		Task:            "review change",
		SubAgentAddress: "127.0.0.1:60001",
	})
	if err != nil {
		t.Fatalf("DispatchToSubAgent: %v", err)
	}
	if subResult != "subagent-finished" {
		t.Fatalf("unexpected sub-agent result: %q", subResult)
	}
}

func TestAddressHelpers(t *testing.T) {
	if !IsLocalAddress(":50051") {
		t.Fatal("expected :50051 to be treated as local")
	}
	if !IsLocalAddress("127.0.0.1:50051") {
		t.Fatal("expected loopback address to be treated as local")
	}
	if IsLocalAddress("10.0.0.8:50051") {
		t.Fatal("expected non-loopback address to be treated as remote")
	}
	port, err := PortFromAddress("127.0.0.1:50051")
	if err != nil {
		t.Fatalf("PortFromAddress: %v", err)
	}
	if port != "50051" {
		t.Fatalf("unexpected port: %q", port)
	}
}
