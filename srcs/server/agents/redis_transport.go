package agents

import (
	"context"

	"github.com/redis/go-redis/v9"
)

// RedisPubSubTransport uses Redis Pub/Sub for scalable, multi-tenant execution in Cloud Mode.
// It implements the Transport interface.
type RedisPubSubTransport struct {
	client       redis.UniversalClient
	publishChan  string
	subscribeCh  *redis.PubSub
	msgChan      <-chan *redis.Message
}

// NewRedisPubSubTransport creates a new RedisPubSubTransport.
func NewRedisPubSubTransport(client redis.UniversalClient, publishChan string, subscribeChan string) *RedisPubSubTransport {
	pubsub := client.Subscribe(context.Background(), subscribeChan)
	return &RedisPubSubTransport{
		client:      client,
		publishChan: publishChan,
		subscribeCh: pubsub,
		msgChan:     pubsub.Channel(),
	}
}

// Send publishes a message to the target Redis channel.
func (t *RedisPubSubTransport) Send(ctx context.Context, message []byte) error {
	return t.client.Publish(ctx, t.publishChan, message).Err()
}

// Receive waits for a message from the subscribed Redis channel.
func (t *RedisPubSubTransport) Receive(ctx context.Context) ([]byte, error) {
	select {
	case <-ctx.Done():
		return nil, ctx.Err()
	case msg, ok := <-t.msgChan:
		if !ok {
			return nil, context.Canceled
		}
		return []byte(msg.Payload), nil
	}
}

// Close closes the underlying Redis Pub/Sub subscription.
func (t *RedisPubSubTransport) Close() error {
	return t.subscribeCh.Close()
}
