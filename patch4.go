<<<<<<< SEARCH
// PublishAgentNotification sends a lightweight inbox-notification to a specific
// agent's Centrifuge channel.
func (cn *CentrifugeNode) PublishAgentNotification(agentID string, msg Message) {
	channel := "agent:" + agentID
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal agent notification", "error", err)
		return
	}
	if _, err := cn.node.Publish(channel, data); err != nil {
		slog.Debug("[centrifuge] publish agent notification", "channel", channel, "error", err)
	}
}
=======
// PublishAgentNotification sends a lightweight inbox-notification to a specific
// agent's Centrifuge channel.
func (cn *CentrifugeNode) PublishAgentNotification(agentID string, msg Message) {
	channel := "agent:" + agentID
	data, err := json.Marshal(msg)
	if err != nil {
		slog.Error("[centrifuge] marshal agent notification", "error", err)
		return
	}
	if _, err := cn.node.Publish(channel, data); err != nil {
		slog.Debug("[centrifuge] publish agent notification", "channel", channel, "error", err)
	}
}

// PublishTaskBroadcast sends task updates to the swarm mesh.
func (cn *CentrifugeNode) PublishTaskBroadcast(taskID string, payload interface{}) {
	channel := "mesh:tasks"
	data, err := json.Marshal(map[string]interface{}{
		"type":    "TASK_BROADCAST",
		"task_id": taskID,
		"payload": payload,
	})
	if err != nil {
		slog.Error("[centrifuge] marshal task broadcast", "error", err)
		return
	}
	if _, err := cn.node.Publish(channel, data); err != nil {
		slog.Debug("[centrifuge] publish task broadcast", "channel", channel, "error", err)
	}
}
>>>>>>> REPLACE
