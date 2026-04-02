package orchestration

import (
	"log/slog"
	"encoding/json"
)

// PublishTaskBroadcast broadcasts a SwarmTask change to the `swarm:tasks:updates` channel via Centrifuge
func (cn *CentrifugeNode) PublishTaskBroadcast(task *Task) {
	channel := "swarm:tasks:updates"
	data, err := json.Marshal(task)
	if err != nil {
		slog.Error("[centrifuge] marshal task broadcast", "error", err)
		return
	}
	if _, err := cn.node.Publish(channel, data); err != nil {
		slog.Debug("[centrifuge] publish task broadcast", "channel", channel, "error", err)
	}
}

// BroadcastTaskUpdate is a convenience method on Hub to fan out the update
func (h *Hub) BroadcastTaskUpdate(task *Task) {
	cn := h.CentrifugeNode()
	if cn != nil {
		cn.PublishTaskBroadcast(task)
	}
}
