package mesh

import "context"

// TeammateMeshEvent defines the message format for the mesh
type TeammateMeshEvent struct {
	EventType string                 `json:"event_type"`
	TaskID    string                 `json:"task_id"`
	Payload   map[string]interface{} `json:"payload"`
}

// MeshPubSub is the interface for the Pub/Sub backplane
type MeshPubSub interface {
	Publish(ctx context.Context, topic string, message TeammateMeshEvent) error
	Subscribe(ctx context.Context, topic string) (<-chan TeammateMeshEvent, error)
	Close() error
}
