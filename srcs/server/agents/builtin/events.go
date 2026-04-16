package builtin

// AgentEventType identifies the kind of event emitted during agent execution.
type AgentEventType int

const (
	AgentEventTypeIterationStarted AgentEventType = iota
	AgentEventTypeTaskComplete
	AgentEventTypeToolCall
	AgentEventTypeStreamChunk // Added for streaming support
)

// AgentEvent is a structured event emitted by the agent loop.
type AgentEvent struct {
	Type         AgentEventType
	Iteration    int
	MessageCount int
	Content      string
	ToolName     string
	ToolArgsJSON string
	ToolResult   string
}

// EventCallback is called for each AgentEvent during RunWithCallback.
type EventCallback func(AgentEvent)
