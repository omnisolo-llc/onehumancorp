// Command agentd is the standalone builtin agent gRPC daemon.
//
// It exposes an AgentService gRPC server (RunTask, Ping, DispatchToSubAgent)
// and executes the full builtin agent ReAct loop.  The main OHC server
// connects as a client; the agent connects to sub-agent processes as a client
// in turn.
//
// When DispatchToSubAgent is called without a remote address, the sub-agent
// runs in-process as a goroutine and communicates over a Go channel, taking
// full advantage of Go's concurrency model.
//
// Configuration is via environment variables:
//
//	OHC_AGENT_ADDRESS       gRPC listen address (default: 127.0.0.1:50051)
//	OHC_AGENT_ID            agent identifier (default: auto-generated UUID)
//	ANTHROPIC_API_KEY       enables the Anthropic Claude LLM backend
//	OPENAI_API_KEY          enables the OpenAI LLM backend
//	OHC_LOCAL_LLM_ENDPOINT  Ollama endpoint (default: http://localhost:11434/api/chat)
//	OHC_LLM_PROVIDER        explicit provider override: "anthropic"|"openai"|"ollama"
//	OHC_LLM_MODEL           LLM model name
//	OHC_MAX_TOKENS          maximum tokens per LLM response (default: 2048)
package main

import (
	"context"
	"fmt"
	"log/slog"
	"net"
	"os"
	"os/signal"
	"syscall"

	"github.com/google/uuid"
	agentservicepb "github.com/onehumancorp/mono/srcs/proto/agentservice"
	agentgrpc "github.com/onehumancorp/mono/srcs/server/agents/builtin/grpc"
	"google.golang.org/grpc"
)

func main() {
	if err := run(); err != nil {
		fmt.Fprintln(os.Stderr, "agentd:", err)
		os.Exit(1)
	}
}

func run() error {
	address := getEnv("OHC_AGENT_ADDRESS", agentgrpc.DefaultAddress)
	agentID := getEnv("OHC_AGENT_ID", uuid.New().String())

	cfg := agentgrpc.AgentConfig{
		LLMProvider:  getEnv("OHC_LLM_PROVIDER", ""),
		Model:        getEnv("OHC_LLM_MODEL", ""),
		LLMEndpoint:  getEnv("OHC_LOCAL_LLM_ENDPOINT", ""),
		MaxTokens:    getEnvInt("OHC_MAX_TOKENS", 2048),
		MaxIterations: getEnvInt("OHC_MAX_ITERATIONS", 0),
		MaxContextMessages: getEnvInt("OHC_MAX_CONTEXT_MESSAGES", 0),
	}

	lis, err := net.Listen("tcp", address)
	if err != nil {
		return fmt.Errorf("listen %s: %w", address, err)
	}

	srv := grpc.NewServer()
	svc := agentgrpc.NewAgentServiceServer(agentID, cfg, nil)
	agentservicepb.RegisterAgentServiceServer(srv, svc)
	slog.Info("agentd: starting", "address", address, "agent_id", agentID)

	// Handle SIGTERM / SIGINT for graceful shutdown.
	ctx, stop := signal.NotifyContext(context.Background(), syscall.SIGTERM, syscall.SIGINT)
	defer stop()

	errCh := make(chan error, 1)
	go func() {
		if err := srv.Serve(lis); err != nil {
			errCh <- err
		}
		close(errCh)
	}()

	select {
	case <-ctx.Done():
		slog.Info("agentd: shutting down gracefully")
		srv.GracefulStop()
		return nil
	case err := <-errCh:
		return fmt.Errorf("grpc serve: %w", err)
	}
}

func getEnv(key, fallback string) string {
	if v, ok := os.LookupEnv(key); ok && v != "" {
		return v
	}
	return fallback
}

func getEnvInt(key string, fallback int) int {
	v := os.Getenv(key)
	if v == "" {
		return fallback
	}
	var n int
	if _, err := fmt.Sscanf(v, "%d", &n); err != nil {
		return fallback
	}
	return n
}
