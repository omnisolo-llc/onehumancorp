package agents

import (
	"context"
	"encoding/json"
	"github.com/redis/go-redis/v9"
)

type RedisPubSubTransport struct {
	client *redis.Client
	ctx    context.Context
	cancel context.CancelFunc
}

func NewRedisPubSubTransport(client *redis.Client) *RedisPubSubTransport {
	ctx, cancel := context.WithCancel(context.Background())
	return &RedisPubSubTransport{
		client: client,
		ctx:    ctx,
		cancel: cancel,
	}
}

func (t *RedisPubSubTransport) Send(channel string, msg *Message) error {
	b, err := json.Marshal(msg)
	if err != nil {
		return err
	}
	return t.client.Publish(t.ctx, channel, b).Err()
}

func (t *RedisPubSubTransport) Receive(channel string) (<-chan *Message, error) {
	pubsub := t.client.Subscribe(t.ctx, channel)
	ch := make(chan *Message)

	go func() {
		defer close(ch)
		defer pubsub.Close()

		for {
			select {
			case <-t.ctx.Done():
				return
			case msg, ok := <-pubsub.Channel():
				if !ok || msg == nil {
					return
				}
				var m Message
				if err := json.Unmarshal([]byte(msg.Payload), &m); err == nil {
					select {
					case ch <- &m:
					case <-t.ctx.Done():
						return
					}
				}
			}
		}
	}()

	return ch, nil
}

func (t *RedisPubSubTransport) Close() error {
	t.cancel()
	return nil
}
