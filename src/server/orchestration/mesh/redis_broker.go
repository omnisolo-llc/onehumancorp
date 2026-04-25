package mesh

import (
    "context"
    "github.com/redis/rueidis"
)

type redisMeshSubscription struct {
    client  rueidis.Client
    channel string
    cancel  context.CancelFunc
}

func (s *redisMeshSubscription) Close() error {
    s.cancel()
    return nil
}

type RedisMeshBroker struct {
    client rueidis.Client
}

func NewRedisMeshBroker(client rueidis.Client) *RedisMeshBroker {
    return &RedisMeshBroker{client: client}
}

func (b *RedisMeshBroker) Broadcast(ctx context.Context, channel string, payload []byte) error {
    cmd := b.client.B().Publish().Channel(channel).Message(string(payload)).Build()
    return b.client.Do(ctx, cmd).Error()
}

func (b *RedisMeshBroker) Subscribe(ctx context.Context, channel string, handler func(msg []byte)) (Subscription, error) {
    subCtx, cancel := context.WithCancel(ctx)

    go func() {
        err := b.client.Receive(subCtx, b.client.B().Subscribe().Channel(channel).Build(), func(msg rueidis.PubSubMessage) {
            handler([]byte(msg.Message))
        })
        if err != nil {
            // Check if context canceled, else handle err...
        }
    }()

    return &redisMeshSubscription{
        client:  b.client,
        channel: channel,
        cancel:  cancel,
    }, nil
}
