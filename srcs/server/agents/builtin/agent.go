package builtin

// BuiltinAgent handles the core loop for the builtin agent.
type BuiltinAgent struct {
	Client      LLMClient
	Model       string
	System      string
	Tools       []Tool
	MaxTokens   int
	Temperature float32
}

// Event types for the agent's run loop.
type EventType string

const (
	EventTurnStart       EventType = "turn_start"
	EventLLMCallStart    EventType = "llm_call_start"
	EventLLMCallComplete EventType = "llm_call_complete"
	EventToolStart       EventType = "tool_start"
	EventToolComplete    EventType = "tool_complete"
	EventToolError       EventType = "tool_error"
	EventError           EventType = "error"
	EventSuccess         EventType = "success"
)

// AgentEvent represents an observable event during the agent execution loop.
type AgentEvent struct {
	Type EventType
	Data any
}

// LLMClient is the interface for talking to the LLM backend.
type LLMClient interface {
	Chat(req ChatRequest) (ChatResponse, error)
}
