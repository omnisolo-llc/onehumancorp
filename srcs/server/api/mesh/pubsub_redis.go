package mesh

import (
	"context"

	"github.com/redis/rueidis"
)

type RedisPubSub struct {
	client rueidis.Client
}

func NewRedisPubSub(client rueidis.Client) *RedisPubSub {
	return &RedisPubSub{
		client: client,
	}
}

func (p *RedisPubSub) Publish(ctx context.Context, topic string, message []byte) error {
	cmd := p.client.B().Publish().Channel(topic).Message(string(message)).Build()
	return p.client.Do(ctx, cmd).Error()
}

func (p *RedisPubSub) Subscribe(ctx context.Context, topic string) (<-chan []byte, error) {
	out := make(chan []byte, 100)

	go func() {
		defer close(out)

		err := p.client.Receive(ctx, p.client.B().Subscribe().Channel(topic).Build(), func(msg rueidis.PubSubMessage) {
			select {
			case out <- []byte(msg.Message):
			case <-ctx.Done():
			}
		})
		if err != nil && err != context.Canceled {
			// In a real application, handle error logging
		}
	}()

	return out, nil
}
