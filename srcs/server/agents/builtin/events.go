package builtin

// AgentEventType classifies an event emitted by the agent loop.
type AgentEventType int

const (
	// AgentEventTypeTextChunk is a partial text response from the LLM.
	AgentEventTypeTextChunk AgentEventType = iota
	// AgentEventTypeToolCall signals that a tool was invoked and its result collected.
	AgentEventTypeToolCall
	// AgentEventTypeTaskComplete signals the loop finished with a final message.
	AgentEventTypeTaskComplete
	// AgentEventTypeIterationStarted signals the start of a new ReAct iteration.
	AgentEventTypeIterationStarted
)

// AgentEvent is emitted by RunWithCallback during the agent loop.
type AgentEvent struct {
	Type AgentEventType

	// Content holds the assistant text (TEXT_CHUNK, TASK_COMPLETE).
	Content string

	// Tool-call fields (TOOL_CALL).
	ToolName     string
	ToolArgsJSON string
	ToolResult   string

	// Iteration fields (ITERATION_STARTED).
	Iteration    int
	MessageCount int
}

// EventCallback is called synchronously during RunWithCallback for each event.
// Implementations must not block; use a buffered channel or fire-and-forget
// goroutine if further processing is needed.
type EventCallback func(AgentEvent)
