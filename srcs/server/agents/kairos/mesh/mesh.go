package mesh

import "context"

// TeammateMesh defines the realtime asynchronous coordination layer for agents.
type TeammateMesh interface {
	Publish(ctx context.Context, channel, message string) error
	Subscribe(ctx context.Context, channel string) (<-chan string, error)
	AcquireLock(ctx context.Context, key string) (func(), error)
}

const (
	ChannelTaskCreated   = "mesh:events:task_created"
	ChannelStatusUpdate  = "mesh:events:status_update"
	ChannelMailAgentPrefix = "mesh:mail:agent_"
)

// AgentMailChannel returns the channel name for an agent.
func AgentMailChannel(agentID string) string {
	return ChannelMailAgentPrefix + agentID
}
