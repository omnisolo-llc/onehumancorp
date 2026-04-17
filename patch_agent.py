with open('srcs/server/agents/builtin/agent.go', 'r') as f:
    content = f.read()

# Add Tracker to BuiltinAgent
content = content.replace(
"""type BuiltinAgent struct {
	Client      LLMClient
	Model       string
	System      string
	Tools       []Tool
	MaxTokens   int
	Temperature float32
	MaxTaskBudget int // Maximum output tokens permitted for an entire task
}""",
"""import_telemetry

type BuiltinAgent struct {
	AgentID     string
	Client      LLMClient
	Model       string
	System      string
	Tools       []Tool
	MaxTokens   int
	Temperature float32
	MaxTaskBudget int // Maximum output tokens permitted for an entire task
	Tracker     *telemetry.CostTracker
}""")

content = content.replace("import_telemetry", """
import "github.com/onehumancorp/mono/srcs/server/telemetry"
""")

# Initialize Tracker in SpawnTask
content = content.replace(
"""	agent := &BuiltinAgent{
		Client:    cfg.LLM,
		Model:     "claude-3-7-sonnet-20250219", // Default
		System:    cfg.SystemPrompt + cfg.SystemPromptSuffix,
		Tools:     cfg.Tools,
		MaxTokens: cfg.MaxTokensPerTurn,
	}""",
"""	agent := &BuiltinAgent{
		AgentID:   id,
		Client:    cfg.LLM,
		Model:     "claude-3-7-sonnet-20250219", // Default
		System:    cfg.SystemPrompt + cfg.SystemPromptSuffix,
		Tools:     cfg.Tools,
		MaxTokens: cfg.MaxTokensPerTurn,
		Tracker:   telemetry.NewCostTracker(nil),
	}""")

with open('srcs/server/agents/builtin/agent.go', 'w') as f:
    f.write(content)
