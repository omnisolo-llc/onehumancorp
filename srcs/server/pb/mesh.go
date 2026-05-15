package pb

// MeshEvent definition for Teammate Mesh APIs
type MeshEvent struct {
	EventID   string `json:"event_id"`
	Topic     string `json:"topic"`
	Payload   []byte `json:"payload"`
	Timestamp int64  `json:"timestamp"`
}

// TeammateMeshEvent for rich payload task broadcasting
type TeammateMeshEvent struct {
	AgentID string `json:"agent_id"`
	Action  string `json:"action"`
	Status  string `json:"status"`
	Payload []byte `json:"payload"`
	MsgID   string `json:"msg_id"`
}
