package orchestration

// TaskClaimedEvent is emitted when a task is claimed by an agent.
type TaskClaimedEvent struct {
	TaskID  string `json:"task_id"`
	AgentID string `json:"agent_id"`
}

// TaskCompletedEvent is emitted when a task is completed by an agent.
type TaskCompletedEvent struct {
	TaskID  string `json:"task_id"`
	AgentID string `json:"agent_id"`
	Status  string `json:"status"`
}

// AgentStatusUpdateEvent is emitted to broadcast the current status of an agent.
type AgentStatusUpdateEvent struct {
	AgentID string `json:"agent_id"`
	Status  string `json:"status"`
}
