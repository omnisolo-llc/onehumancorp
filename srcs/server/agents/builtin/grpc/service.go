// Package agentgrpc implements the gRPC AgentService for the builtin agent.
//
// The service exposes three RPCs:
//   - RunTask: server-streaming; streams RunTaskEvents as the agent executes
//     a ReAct loop using the configured LLM backend and tool set.
//   - Ping: unary health-check returning the agent ID and version.
//   - DispatchToSubAgent: unary; delegates a task to a sub-agent.
//     When SubAgentRequest.sub_agent_address is empty the sub-agent is run
//     in-process as a goroutine, and the result is returned over a channel.
//     When sub_agent_address is set the request is forwarded to that remote
//     address over a new gRPC connection.
package agentgrpc

import (
	"context"
	"fmt"
	"io"
	"log/slog"

	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/status"
)

const agentVersion = "1.0.0"

// AgentServiceServer implements agentservicepb.AgentServiceServer using the
// builtin Go agent loop.
type AgentServiceServer struct {
	agentservicepb.UnimplementedAgentServiceServer

	agentID string
	// cfg holds the default runtime configuration for new agent instances.
	cfg AgentConfig
	// llmOverride, when non-nil, is used instead of constructing a client from cfg.
	// This is intended for unit tests only.
	llmOverride builtin.LLMClient
}

// SetLLMClientOverride sets a test LLM client that bypasses newLLMClient.
// This is only intended for use in tests.
func (s *AgentServiceServer) SetLLMClientOverride(c builtin.LLMClient) {
	s.llmOverride = c
}

// AgentConfig is the default configuration for agent instances created by the
// server.  Individual RunTaskRequest fields can override these defaults.
type AgentConfig struct {
	LLMProvider        string
	Model              string
	LLMEndpoint        string
	SystemPrompt       string
	MaxTokens          int
	Temperature        float32
	MaxIterations      int
	MaxContextMessages int
}

// NewAgentServiceServer creates a new AgentServiceServer.
func NewAgentServiceServer(agentID string, cfg AgentConfig) *AgentServiceServer {
	return &AgentServiceServer{
		agentID: agentID,
		cfg:     cfg,
	}
}

// Ping handles a health-check RPC.
func (s *AgentServiceServer) Ping(_ context.Context, _ *agentservicepb.PingRequest) (*agentservicepb.PingResponse, error) {
	return &agentservicepb.PingResponse{
		AgentId: s.agentID,
		Version: agentVersion,
	}, nil
}

// RunTask streams RunTaskEvents to the caller while executing the agent loop.
func (s *AgentServiceServer) RunTask(req *agentservicepb.RunTaskRequest, stream agentservicepb.AgentService_RunTaskServer) error {
	cfg := s.resolveConfig(req)

	llmClient, err := newLLMClient(cfg)
	if err != nil {
		return status.Errorf(codes.Internal, "failed to create LLM client: %v", err)
	}

	agent := &builtin.BuiltinAgent{
		Client:      llmClient,
		Model:       cfg.Model,
		System:      cfg.SystemPrompt,
		Tools:       builtin.AllTools(),
		MaxTokens:   cfg.MaxTokens,
		Temperature: cfg.Temperature,
	}

	// Notify the caller that the run has started.
	if err := stream.Send(&agentservicepb.RunTaskEvent{
		Type:      agentservicepb.EventType_RUN_STARTED,
		Iteration: 0,
	}); err != nil {
		return err
	}

	initialMessages := []builtin.Message{
		{Role: builtin.RoleUser, Content: req.Task},
	}

	// Run the agent loop and collect streaming events via the EventCallback.
	// The agent writes to a channel; we forward events to the gRPC stream here.
	eventCh := make(chan *agentservicepb.RunTaskEvent, 32)
	errCh := make(chan error, 1)

	go func() {
		messages, loopErr := agent.RunWithCallback(stream.Context(), initialMessages, func(evt builtin.AgentEvent) {
			pb := agentEventToProto(evt)
			if pb != nil {
				eventCh <- pb
			}
		})
		_ = messages
		errCh <- loopErr
		close(eventCh)
	}()

	// Forward events from the channel to the gRPC stream until the goroutine
	// closes eventCh, then check the loop error.
	for evt := range eventCh {
		if sendErr := stream.Send(evt); sendErr != nil {
			return sendErr
		}
	}

	loopErr := <-errCh
	if loopErr != nil {
		if err := stream.Send(&agentservicepb.RunTaskEvent{
			Type:  agentservicepb.EventType_TASK_ERROR,
			Error: loopErr.Error(),
		}); err != nil {
			slog.Error("agentgrpc: failed to send TASK_ERROR event", "err", err)
		}
		return status.Errorf(codes.Internal, "agent loop error: %v", loopErr)
	}

	return nil
}

// DispatchToSubAgent delegates work to a sub-agent.
//
// When req.SubAgentAddress is empty the sub-agent runs in-process as a
// goroutine; the caller goroutine blocks on a result channel.  When
// req.SubAgentAddress is non-empty the request is forwarded to that remote
// gRPC endpoint and the response is returned.
func (s *AgentServiceServer) DispatchToSubAgent(ctx context.Context, req *agentservicepb.SubAgentRequest) (*agentservicepb.SubAgentResponse, error) {
	if req.SubAgentAddress == "" {
		return s.dispatchInProcess(ctx, req)
	}
	return s.dispatchRemote(ctx, req)
}

// dispatchInProcess runs the sub-agent as a goroutine and returns the result
// over a channel.  This avoids spawning a new process for every sub-task and
// takes full advantage of Go's concurrency model.
func (s *AgentServiceServer) dispatchInProcess(ctx context.Context, req *agentservicepb.SubAgentRequest) (*agentservicepb.SubAgentResponse, error) {
	type result struct {
		text string
		err  error
	}

	resultCh := make(chan result, 1)

	go func() {
		subReq := &agentservicepb.RunTaskRequest{
			Task:               req.Task,
			Model:              req.Model,
			LlmProvider:        req.LlmProvider,
			LlmEndpoint:        req.LlmEndpoint,
			SystemPrompt:       req.SystemPrompt,
			MaxTokens:          req.MaxTokens,
			Temperature:        req.Temperature,
			MaxContextMessages: req.MaxContextMessages,
			RuntimeConfig:      req.RuntimeConfig,
		}

		cfg := s.resolveConfig(subReq)

		llmClient, err := newLLMClient(cfg)
		if err != nil {
			resultCh <- result{err: fmt.Errorf("failed to create LLM client: %w", err)}
			return
		}

		agent := &builtin.BuiltinAgent{
			Client:      llmClient,
			Model:       cfg.Model,
			System:      cfg.SystemPrompt,
			Tools:       builtin.AllTools(),
			MaxTokens:   cfg.MaxTokens,
			Temperature: cfg.Temperature,
		}

		messages, err := agent.Run(ctx, []builtin.Message{
			{Role: builtin.RoleUser, Content: req.Task},
		})

		if err != nil {
			resultCh <- result{err: err}
			return
		}

		// Return the last assistant message as the result.
		var finalText string
		for i := len(messages) - 1; i >= 0; i-- {
			if messages[i].Role == builtin.RoleAssistant {
				finalText = messages[i].Content
				break
			}
		}

		resultCh <- result{text: finalText}
	}()

	select {
	case <-ctx.Done():
		return nil, status.FromContextError(ctx.Err()).Err()
	case res := <-resultCh:
		if res.err != nil {
			return &agentservicepb.SubAgentResponse{Error: res.err.Error()}, nil
		}
		return &agentservicepb.SubAgentResponse{Result: res.text}, nil
	}
}

// dispatchRemote forwards the sub-agent request to a remote gRPC endpoint.
func (s *AgentServiceServer) dispatchRemote(ctx context.Context, req *agentservicepb.SubAgentRequest) (*agentservicepb.SubAgentResponse, error) {
	conn, err := grpc.NewClient(req.SubAgentAddress,
		grpc.WithTransportCredentials(insecure.NewCredentials()),
	)
	if err != nil {
		return nil, status.Errorf(codes.Unavailable, "cannot connect to sub-agent at %s: %v", req.SubAgentAddress, err)
	}
	defer conn.Close()

	client := agentservicepb.NewAgentServiceClient(conn)

	// Collect all events from the streaming RunTask call.
	runReq := &agentservicepb.RunTaskRequest{
		Task:               req.Task,
		Model:              req.Model,
		LlmProvider:        req.LlmProvider,
		LlmEndpoint:        req.LlmEndpoint,
		SystemPrompt:       req.SystemPrompt,
		MaxTokens:          req.MaxTokens,
		Temperature:        req.Temperature,
		MaxContextMessages: req.MaxContextMessages,
		RuntimeConfig:      req.RuntimeConfig,
	}

	stream, err := client.RunTask(ctx, runReq)
	if err != nil {
		return nil, status.Errorf(codes.Internal, "RunTask RPC failed: %v", err)
	}

	var lastContent string
	var taskErr string

	for {
		evt, err := stream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			return nil, status.Errorf(codes.Internal, "stream receive error: %v", err)
		}
		switch evt.Type {
		case agentservicepb.EventType_TASK_COMPLETE:
			lastContent = evt.Content
		case agentservicepb.EventType_TASK_ERROR:
			taskErr = evt.Error
		}
	}

	if taskErr != "" {
		return &agentservicepb.SubAgentResponse{Error: taskErr}, nil
	}
	return &agentservicepb.SubAgentResponse{Result: lastContent}, nil
}

// resolveConfig merges the per-request parameters with the server's defaults.
func (s *AgentServiceServer) resolveConfig(req *agentservicepb.RunTaskRequest) AgentConfig {
	cfg := s.cfg

	// Runtime config proto field takes precedence over per-request legacy fields.
	if rc := req.RuntimeConfig; rc != nil {
		if rc.LlmProvider != "" {
			cfg.LLMProvider = rc.LlmProvider
		}
		if rc.Model != "" {
			cfg.Model = rc.Model
		}
		if rc.LlmEndpoint != "" {
			cfg.LLMEndpoint = rc.LlmEndpoint
		}
		if rc.SystemPrompt != "" {
			cfg.SystemPrompt = rc.SystemPrompt
		}
		if rc.MaxTokens > 0 {
			cfg.MaxTokens = int(rc.MaxTokens)
		}
		if rc.Temperature > 0 {
			cfg.Temperature = rc.Temperature
		}
		if rc.MaxIterations > 0 {
			cfg.MaxIterations = int(rc.MaxIterations)
		}
		if rc.MaxContextMessages > 0 {
			cfg.MaxContextMessages = int(rc.MaxContextMessages)
		}
	}

	// Fall back to per-request legacy fields.
	if req.LlmProvider != "" && cfg.LLMProvider == "" {
		cfg.LLMProvider = req.LlmProvider
	}
	if req.Model != "" && cfg.Model == "" {
		cfg.Model = req.Model
	}
	if req.LlmEndpoint != "" && cfg.LLMEndpoint == "" {
		cfg.LLMEndpoint = req.LlmEndpoint
	}
	if req.SystemPrompt != "" && cfg.SystemPrompt == "" {
		cfg.SystemPrompt = req.SystemPrompt
	}
	if req.MaxTokens > 0 && cfg.MaxTokens == 0 {
		cfg.MaxTokens = int(req.MaxTokens)
	}
	if req.Temperature > 0 && cfg.Temperature == 0 {
		cfg.Temperature = req.Temperature
	}
	if req.MaxContextMessages > 0 && cfg.MaxContextMessages == 0 {
		cfg.MaxContextMessages = int(req.MaxContextMessages)
	}

	// Apply sensible defaults.
	if cfg.MaxTokens == 0 {
		cfg.MaxTokens = 2048
	}
	if cfg.SystemPrompt == "" {
		cfg.SystemPrompt = builtin.GetSystemPrompt()
	}
	if cfg.LLMProvider == "" {
		cfg.LLMProvider = "ollama"
	}
	if cfg.Model == "" {
		cfg.Model = "llama3"
	}

	return cfg
}

// agentEventToProto converts an internal AgentEvent to a RunTaskEvent proto.
func agentEventToProto(evt builtin.AgentEvent) *agentservicepb.RunTaskEvent {
	switch evt.Type {
	case builtin.AgentEventTypeTextChunk:
		return &agentservicepb.RunTaskEvent{
			Type:    agentservicepb.EventType_TEXT_CHUNK,
			Content: evt.Content,
		}
	case builtin.AgentEventTypeToolCall:
		return &agentservicepb.RunTaskEvent{
			Type:         agentservicepb.EventType_TOOL_CALL,
			ToolName:     evt.ToolName,
			ToolArgsJson: evt.ToolArgsJSON,
			ToolResult:   evt.ToolResult,
		}
	case builtin.AgentEventTypeTaskComplete:
		return &agentservicepb.RunTaskEvent{
			Type:    agentservicepb.EventType_TASK_COMPLETE,
			Content: evt.Content,
		}
	case builtin.AgentEventTypeIterationStarted:
		return &agentservicepb.RunTaskEvent{
			Type:         agentservicepb.EventType_ITERATION_STARTED,
			Iteration:    int32(evt.Iteration),
			MessageCount: int32(evt.MessageCount),
		}
	default:
		return nil
	}
}
