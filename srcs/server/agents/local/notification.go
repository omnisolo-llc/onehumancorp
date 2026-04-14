package local

// TaskNotificationPayload is the typed notification sent when a local agent task completes.
//
// It is serialized to JSON and delivered via the Hub's pub/sub mechanism.
// This replaces the legacy XML <task-notification> format.
//
// Field numbers are annotated to map 1:1 with the ohc.agent.service.TaskNotification
// protobuf message defined in agent_service.proto; migration to proto.Marshal is
// mechanical once protoc integration is complete.
type TaskNotificationPayload struct {
	TaskID     string `json:"task_id"`      // proto field 1
	ToolUseID  string `json:"tool_use_id"`  // proto field 2
	OutputFile string `json:"output_file"`  // proto field 3
	Status     string `json:"status"`       // proto field 4
	Summary    string `json:"summary"`      // proto field 5
	Result     string `json:"result"`       // proto field 6
	TokenCount int64  `json:"token_count"`  // proto field 7
	ToolUses   int64  `json:"tool_uses"`    // proto field 8
	DurationMs int64  `json:"duration_ms"`  // proto field 9
}
