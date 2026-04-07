package agents

import (
	"context"
	"encoding/json"
	"log/slog"
	"os"

	"github.com/onehumancorp/mono/srcs/server/agents/builtin"
	"github.com/onehumancorp/mono/srcs/server/integrations/plane"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// RunBuiltinAgentForTask spins up a builtin agent to handle a specific issue.
func RunBuiltinAgentForTask(ctx context.Context, hub *orchestration.Hub, issue plane.Issue, agentID string) {
	// 1. Setup client. For now use Anthropic if available, else OpenAI, else error.
	var client builtin.LLMClient
	if key := os.Getenv("ANTHROPIC_API_KEY"); key != "" {
		client = builtin.NewAnthropicClient(key)
	} else if key := os.Getenv("OPENAI_API_KEY"); key != "" {
		client = builtin.NewOpenAIClient(key)
	} else {
		slog.Error("No API key available for builtin agent runner", "agent_id", agentID)
		return
	}

	registry := builtin.NewToolRegistry()
	registry.Register(builtin.BashTool)
	registry.Register(builtin.FileReadTool)
	registry.Register(builtin.FileWriteTool)
	registry.Register(builtin.GlobTool)
	registry.Register(builtin.GrepTool)
	registry.Register(builtin.WebFetchTool)
	registry.Register(builtin.WebSearchTool)
	registry.Register(builtin.SendMessageTool)
	registry.Register(builtin.TodoWriteTool)
	registry.Register(builtin.ToolSearchTool)

	agent := &builtin.BuiltinAgent{
		Client:      client,
		Model:       "claude-3-7-sonnet-20250219", // Default if using Anthropic. TODO: select dynamically
		System:      builtin.GetSystemPrompt(),
		Tools:       registry.GetAll(),
		MaxTokens:   8192,
		Temperature: 0.1,
	}

	// 2. Prepare initial messages
	payload, _ := json.Marshal(map[string]string{
		"issue_id":   issue.ID,
		"issue_name": issue.Name,
		"directive":  "Please resolve the attached issue descriptor.",
	})

	initialMsg := builtin.Message{
		Role:    builtin.RoleUser,
		Content: string(payload),
	}

	slog.Info("Starting builtin agent loop", "agent_id", agentID, "issue_id", issue.ID)

	_, err := agent.Run(ctx, []builtin.Message{initialMsg})
	if err != nil {
		slog.Error("Builtin agent run failed", "agent_id", agentID, "error", err)
	} else {
		slog.Info("Builtin agent run finished successfully", "agent_id", agentID, "issue_id", issue.ID)
	}
}
