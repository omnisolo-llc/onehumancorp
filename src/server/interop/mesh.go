package interop

import (
	"context"

	"github.com/go-redis/redis/v8"
)

type TeammateMesh interface {
	Publish(ctx context.Context, channel string, data []byte) error
	Subscribe(ctx context.Context, channel string) (<-chan []byte, func(), error)
}

type RedisTeammateMesh struct {
	client *redis.Client
}

func NewRedisTeammateMesh(client *redis.Client) *RedisTeammateMesh {
	return &RedisTeammateMesh{
		client: client,
	}
}

func (m *RedisTeammateMesh) Publish(ctx context.Context, channel string, data []byte) error {
	return m.client.Publish(ctx, channel, data).Err()
}

func (m *RedisTeammateMesh) Subscribe(ctx context.Context, channel string) (<-chan []byte, func(), error) {
	pubsub := m.client.Subscribe(ctx, channel)

	// Wait for confirmation that subscription is created before returning
	_, err := pubsub.Receive(ctx)
	if err != nil {
		return nil, nil, err
	}

	ch := pubsub.Channel()
	outCh := make(chan []byte)

	ctxCancel, cancel := context.WithCancel(ctx)

	go func() {
		defer pubsub.Close()
		defer close(outCh)
		for {
			select {
			case <-ctxCancel.Done():
				return
			case msg, ok := <-ch:
				if !ok {
					return
				}
				outCh <- []byte(msg.Payload)
			}
		}
	}()

	cancelFunc := func() {
		cancel()
	}

	return outCh, cancelFunc, nil
}
