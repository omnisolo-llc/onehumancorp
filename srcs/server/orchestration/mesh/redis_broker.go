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

func (r *RedisMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	cmd := r.client.B().Publish().Channel(channel).Message(string(payload)).Build()
	return r.client.Do(ctx, cmd).Error()
}
