package mesh

import (
	"context"

	"github.com/redis/rueidis"
)

// RedisMeshBroker implements MeshBroker using rueidis for Cloud-Native environments.
type RedisMeshBroker struct {
	client rueidis.Client
}

// NewRedisMeshBroker creates a new RedisMeshBroker.
func NewRedisMeshBroker(client rueidis.Client) *RedisMeshBroker {
	return &RedisMeshBroker{
		client: client,
	}
}

// Broadcast publishes the payload to the specified Redis channel.
func (b *RedisMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	cmd := b.client.B().Publish().Channel(channel).Message(string(payload)).Build()
	return b.client.Do(ctx, cmd).Error()
}
