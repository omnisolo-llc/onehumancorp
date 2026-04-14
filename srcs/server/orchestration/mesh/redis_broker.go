package mesh

import (
	"context"
	"github.com/redis/rueidis"
)

type RedisMeshBroker struct {
	client rueidis.Client
}

func NewRedisMeshBroker(client rueidis.Client) *RedisMeshBroker {
	return &RedisMeshBroker{client: client}
}

func (m *RedisMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
	cmd := m.client.B().Publish().Channel(channel).Message(string(payload)).Build()
	return m.client.Do(ctx, cmd).Error()
}
