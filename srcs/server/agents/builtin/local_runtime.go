package builtin

import (
	"context"

	"github.com/onehumancorp/mono/srcs/server/agents/local"
)

// AgentConfig configures a builtin local-agent task run.
type AgentConfig = local.AgentConfig

// TaskState tracks one running builtin local-agent task.
type TaskState = local.TaskState

// TaskStatus is the lifecycle state of a task.
type TaskStatus = local.TaskStatus

// AgentProgress is the public progress snapshot.
type AgentProgress = local.AgentProgress

// Hub is the builtin runner hub abstraction.
type Hub = local.Hub

// HubAgent is the builtin runner hub agent descriptor.
type HubAgent = local.HubAgent

// HubMessage is the builtin runner hub message descriptor.
type HubMessage = local.HubMessage

// Runner listens for TaskAssignment and runs builtin local tasks.
type Runner = local.Runner

// NewRunner creates a builtin local runner.
func NewRunner(hub Hub, agentID, agentName, role string, cfg AgentConfig) *Runner {
	return local.NewRunner(hub, agentID, agentName, role, cfg)
}

// SpawnTask launches a builtin local task and returns state for polling.
func SpawnTask(ctx context.Context, description, prompt, workDir string, cfg AgentConfig) (*TaskState, error) {
	return local.SpawnTask(ctx, description, prompt, workDir, cfg)
}

// LocalLLMClient is the interface for interacting with a language model using the local runner.
// It is distinct from the builtin package's LLMClient (which uses Chat/ChatRequest).
// Use LocalLLMClient when you want to pass a client to AgentConfig.LLM.
type LocalLLMClient = local.LLMClient

// NewLocalAnthropicClient creates an Anthropic LLM client compatible with AgentConfig.LLM.
func NewLocalAnthropicClient(apiKey, model, endpoint string) LocalLLMClient {
	return local.NewAnthropicClient(apiKey, model, endpoint)
}

// NewLocalOpenAICompatClient creates an OpenAI-compatible LLM client compatible with AgentConfig.LLM.
func NewLocalOpenAICompatClient(endpoint, apiKey, model string) LocalLLMClient {
	return local.NewOpenAICompatClient(endpoint, apiKey, model)
}
