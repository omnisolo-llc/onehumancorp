package mesh

import (
	"context"

	"github.com/redis/rueidis"
)

type RedisMeshBroker struct {
	client rueidis.Client
}

func NewRedisMeshBroker(client rueidis.Client) *RedisMeshBroker {
	return &RedisMeshBroker{
		client: client,
	}
}

func (b *RedisMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	cmd := b.client.B().Publish().Channel(channel).Message(string(payload)).Build()
	return b.client.Do(ctx, cmd).Error()
}
