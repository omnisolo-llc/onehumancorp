package agentgrpc

import (
	"context"
	"fmt"
	"io"
	"log/slog"
	"time"

	"google.golang.org/genai"

	"google.golang.org/adk/agent"
	"google.golang.org/adk/agent/llmagent"
	"google.golang.org/adk/model"
	"google.golang.org/adk/runner"
	"google.golang.org/adk/session"

	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	"google.golang.org/grpc"
	"google.golang.org/grpc/codes"
	"google.golang.org/grpc/credentials/insecure"
	"google.golang.org/grpc/status"
)

const agentVersion = "1.0.0"

// AgentServiceServer implements agentservicepb.AgentServiceServer.
// It uses google.golang.org/adk (llmagent + runner) for the agent loop,
// custom model.LLM adapters for Anthropic / OpenAI / Ollama, and
// adk tool.Toolset wrappers loaded from the ToolsetConfig proto.
type AgentServiceServer struct {
	agentservicepb.UnimplementedAgentServiceServer

	agentID string
	cfg     AgentConfig

	// defaultToolsetCfg is the process-level toolset config.
	// Individual RunTaskRequests can override it via toolset_config.
	defaultToolsetCfg *agentservicepb.ToolsetConfig

	// llmOverride bypasses newADKModel for unit tests.
	llmOverride model.LLM
}

// AgentConfig is the process-level configuration for all agent instances.
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
func NewAgentServiceServer(agentID string, cfg AgentConfig, defaultToolsetCfg *agentservicepb.ToolsetConfig) *AgentServiceServer {
	return &AgentServiceServer{
		agentID:           agentID,
		cfg:               cfg,
		defaultToolsetCfg: defaultToolsetCfg,
	}
}

// SetLLMClientOverride installs a test model.LLM.  Only for unit tests.
func (s *AgentServiceServer) SetLLMClientOverride(m model.LLM) {
	s.llmOverride = m
}

// Ping implements AgentService.Ping.
func (s *AgentServiceServer) Ping(_ context.Context, _ *agentservicepb.PingRequest) (*agentservicepb.PingResponse, error) {
	return &agentservicepb.PingResponse{
		AgentId: s.agentID,
		Version: agentVersion,
	}, nil
}

// RunTask implements AgentService.RunTask.
// It streams RunTaskEvents back to the caller while running the adk agent loop.
func (s *AgentServiceServer) RunTask(req *agentservicepb.RunTaskRequest, stream agentservicepb.AgentService_RunTaskServer) error {
	ctx := stream.Context()
	cfg := s.resolveAgentConfig(req)

	// Inject past successful memories into system prompt for self-learning.
	cfg.SystemPrompt = InjectMemoriesIntoPrompt(cfg.SystemPrompt)

	// Build model.LLM.
	llm, err := s.resolveLLM(cfg)
	if err != nil {
		return status.Errorf(codes.Internal, "failed to create LLM: %v", err)
	}

	// Resolve toolset config (per-request overrides process default).
	toolsetCfg := s.defaultToolsetCfg
	if req.ToolsetConfig != nil {
		toolsetCfg = req.ToolsetConfig
	}

	// Build toolsets (built-in wrappers + MCP servers).
	toolsets, err := BuildToolsets(ctx, toolsetCfg)
	if err != nil {
		return status.Errorf(codes.Internal, "failed to build toolsets: %v", err)
	}

	// Build the adk llmagent.
	a, err := llmagent.New(llmagent.Config{
		Name:        "builtin-agent",
		Description: "Builtin OHC coding agent",
		Model:       llm,
		Instruction: cfg.SystemPrompt,
		Toolsets:    toolsets,
	})
	if err != nil {
		return status.Errorf(codes.Internal, "failed to create agent: %v", err)
	}

	// Create in-memory session service and runner.
	sessionSvc := session.InMemoryService()
	r, err := runner.New(runner.Config{
		AppName:           "ohc-builtin",
		Agent:             a,
		SessionService:    sessionSvc,
		AutoCreateSession: true,
	})
	if err != nil {
		return status.Errorf(codes.Internal, "failed to create runner: %v", err)
	}

	// Progress tracking + task output file (Claude Code harness pattern).
	progress := NewTaskProgress()
	outWriter := NewTaskOutputWriter(req.TaskId)
	defer outWriter.Close()

	// Notify the caller that the run has started.
	if sendErr := stream.Send(&agentservicepb.RunTaskEvent{
		Type: agentservicepb.EventType_RUN_STARTED,
	}); sendErr != nil {
		return sendErr
	}

	start := time.Now()
	task := req.Task

	// Retry loop: on failure, offer the LLM a self-reflection prompt.
	const maxRetries = 1
	var lastErr string
	var finalContent string

	for attempt := 0; attempt <= maxRetries; attempt++ {
		if attempt > 0 {
			task = SelfReflectionPrompt(req.Task, lastErr)
		}

		userContent := genai.NewContentFromText(task, "user")
		eventSeq := r.Run(ctx, "user-1", "", userContent, agent.RunConfig{})

		iteration := 0
		runFailed := false
		for evt, evtErr := range eventSeq {
			if evtErr != nil {
				lastErr = evtErr.Error()
				runFailed = true
				sendErr := stream.Send(&agentservicepb.RunTaskEvent{
					Type:  agentservicepb.EventType_TASK_ERROR,
					Error: evtErr.Error(),
				})
				if sendErr != nil {
					slog.Error("agentgrpc: send TASK_ERROR failed", "err", sendErr)
				}
				break
			}
			if evt == nil {
				continue
			}
			pb := sessionEventToProto(evt, &iteration)
			if pb != nil {
				// Track progress.
				if pb.ToolName != "" {
					progress.RecordToolUse(pb.ToolName)
				}
				if pb.Type == agentservicepb.EventType_TASK_COMPLETE {
					finalContent = pb.Content
					outWriter.Write("[COMPLETE] " + pb.Content)
				}
				if pb.Type == agentservicepb.EventType_TOOL_CALL {
					outWriter.Write(fmt.Sprintf("[TOOL] %s %s", pb.ToolName, pb.ToolArgsJson))
				}
				if sendErr := stream.Send(pb); sendErr != nil {
					return sendErr
				}
			}
		}
		if !runFailed {
			break // success
		}
	}

	// Write AutoDream memory entry so the agent can learn from this task.
	snap := progress.Snapshot()
	outcome := "success"
	if lastErr != "" && finalContent == "" {
		outcome = "failure"
	}
	lessons := ""
	if lastErr != "" {
		lessons = "Error encountered: " + lastErr
	}
	RecordTaskMemory(MemoryEntry{
		TaskID:      req.TaskId,
		Summary:     truncateStr(req.Task, 200),
		Outcome:     outcome,
		Duration:    time.Since(start).Seconds(),
		ToolsUsed:   []string{snap.LastActivity},
		Lessons:     lessons,
		CompletedAt: time.Now(),
	})

	return nil
}

// sessionEventToProto converts an adk session.Event to a RunTaskEvent proto.
func sessionEventToProto(evt *session.Event, iteration *int) *agentservicepb.RunTaskEvent {
	if evt == nil || evt.Content == nil {
		return nil
	}

	hasFunctionCall := false
	hasText := false
	for _, p := range evt.Content.Parts {
		if p.FunctionCall != nil {
			hasFunctionCall = true
		}
		if p.Text != "" {
			hasText = true
		}
	}

	if evt.IsFinalResponse() && hasText && !hasFunctionCall {
		*iteration++
		return &agentservicepb.RunTaskEvent{
			Type:      agentservicepb.EventType_TASK_COMPLETE,
			Content:   extractFinalText(evt.Content),
			Iteration: int32(*iteration),
		}
	}

	if hasFunctionCall {
		for _, p := range evt.Content.Parts {
			if p.FunctionCall != nil {
				*iteration++
				return &agentservicepb.RunTaskEvent{
					Type:         agentservicepb.EventType_TOOL_CALL,
					ToolName:     p.FunctionCall.Name,
					ToolArgsJson: jsonStringify(p.FunctionCall.Args),
					Iteration:    int32(*iteration),
				}
			}
		}
	}

	if hasText {
		return &agentservicepb.RunTaskEvent{
			Type:    agentservicepb.EventType_TEXT_CHUNK,
			Content: extractFinalText(evt.Content),
		}
	}

	return nil
}

// DispatchToSubAgent implements AgentService.DispatchToSubAgent.
// When req.SubAgentAddress is empty the sub-agent runs in-process as a
// goroutine communicating over a channel.  Otherwise the request is
// forwarded to the given gRPC address.
func (s *AgentServiceServer) DispatchToSubAgent(ctx context.Context, req *agentservicepb.SubAgentRequest) (*agentservicepb.SubAgentResponse, error) {
	if req.SubAgentAddress == "" {
		return s.dispatchInProcess(ctx, req)
	}
	return s.dispatchRemote(ctx, req)
}

// dispatchInProcess runs the sub-agent inside a goroutine and returns the
// result over a channel.  This avoids spawning a new OS process for every
// sub-task and leverages Go's lightweight concurrency.
func (s *AgentServiceServer) dispatchInProcess(ctx context.Context, req *agentservicepb.SubAgentRequest) (*agentservicepb.SubAgentResponse, error) {
	type result struct {
		text string
		err  error
	}
	resultCh := make(chan result, 1)

	go func() {
		runReq := subAgentToRunRequest(req)
		cfg := s.resolveAgentConfig(runReq)

		llm, err := s.resolveLLM(cfg)
		if err != nil {
			resultCh <- result{err: fmt.Errorf("create LLM: %w", err)}
			return
		}

		toolsetCfg := s.defaultToolsetCfg
		if runReq.ToolsetConfig != nil {
			toolsetCfg = runReq.ToolsetConfig
		}
		toolsets, err := BuildToolsets(ctx, toolsetCfg)
		if err != nil {
			resultCh <- result{err: fmt.Errorf("build toolsets: %w", err)}
			return
		}

		a, err := llmagent.New(llmagent.Config{
			Name:        "sub-agent",
			Description: "Builtin OHC sub-agent",
			Model:       llm,
			Instruction: cfg.SystemPrompt,
			Toolsets:    toolsets,
		})
		if err != nil {
			resultCh <- result{err: fmt.Errorf("create agent: %w", err)}
			return
		}

		sessionSvc := session.InMemoryService()
		r, err := runner.New(runner.Config{
			AppName:           "ohc-subagent",
			Agent:             a,
			SessionService:    sessionSvc,
			AutoCreateSession: true,
		})
		if err != nil {
			resultCh <- result{err: fmt.Errorf("create runner: %w", err)}
			return
		}

		userContent := genai.NewContentFromText(req.Task, "user")
		var lastText string
		for evt, err := range r.Run(ctx, "user-1", "", userContent, agent.RunConfig{}) {
			if err != nil {
				resultCh <- result{err: err}
				return
			}
			if evt != nil && evt.IsFinalResponse() {
				lastText = extractFinalText(evt.Content)
			}
		}
		resultCh <- result{text: lastText}
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
		return nil, status.Errorf(codes.Unavailable, "connect to sub-agent at %s: %v", req.SubAgentAddress, err)
	}
	defer conn.Close()

	client := agentservicepb.NewAgentServiceClient(conn)
	stream, err := client.RunTask(ctx, subAgentToRunRequest(req))
	if err != nil {
		return nil, status.Errorf(codes.Internal, "RunTask RPC: %v", err)
	}

	var lastContent, taskErr string
	for {
		evt, err := stream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			return nil, status.Errorf(codes.Internal, "stream recv: %v", err)
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

// resolveAgentConfig merges per-request fields with the server defaults.
func (s *AgentServiceServer) resolveAgentConfig(req *agentservicepb.RunTaskRequest) AgentConfig {
	cfg := s.cfg
	if rc := req.GetRuntimeConfig(); rc != nil {
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
	}
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
	if cfg.MaxTokens == 0 {
		cfg.MaxTokens = 2048
	}
	if cfg.SystemPrompt == "" {
		cfg.SystemPrompt = defaultSystemPrompt()
	}
	if cfg.LLMProvider == "" {
		cfg.LLMProvider = "ollama"
	}
	if cfg.Model == "" {
		cfg.Model = "llama3"
	}
	return cfg
}

// resolveLLM returns a model.LLM, using the test override when set.
func (s *AgentServiceServer) resolveLLM(cfg AgentConfig) (model.LLM, error) {
	if s.llmOverride != nil {
		return s.llmOverride, nil
	}
	return newADKModel(cfg)
}

// defaultSystemPrompt returns the built-in system prompt.
func defaultSystemPrompt() string {
	// Delegate to the builtin package's system prompt for consistency.
	return "You are a helpful AI coding assistant. You have access to tools to help you complete tasks. Use them as needed to provide accurate and complete responses."
}

// subAgentToRunRequest converts a SubAgentRequest to a RunTaskRequest.
func subAgentToRunRequest(req *agentservicepb.SubAgentRequest) *agentservicepb.RunTaskRequest {
	return &agentservicepb.RunTaskRequest{
		Task:               req.Task,
		Model:              req.Model,
		LlmProvider:        req.LlmProvider,
		LlmEndpoint:        req.LlmEndpoint,
		SystemPrompt:       req.SystemPrompt,
		MaxTokens:          req.MaxTokens,
		Temperature:        req.Temperature,
		RuntimeConfig:      req.RuntimeConfig,
		ToolsetConfig:      req.ToolsetConfig,
	}
}
