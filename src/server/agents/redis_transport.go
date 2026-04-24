package agents

import (
	"context"

	"github.com/redis/rueidis"
)

// RedisPubSubTransport uses Redis Pub/Sub for scalable, multi-tenant execution in Cloud Mode.
// It implements the Transport interface.
type RedisPubSubTransport struct {
	client      rueidis.Client
	publishChan string
	cancel      context.CancelFunc
	msgChan     chan []byte
	errChan     chan error
	ctx         context.Context
}

// NewRedisPubSubTransport creates a new RedisPubSubTransport.
func NewRedisPubSubTransport(client rueidis.Client, publishChan string, subscribeChan string) *RedisPubSubTransport {
	ctx, cancel := context.WithCancel(context.Background())
	t := &RedisPubSubTransport{
		client:      client,
		publishChan: publishChan,
		cancel:      cancel,
		msgChan:     make(chan []byte),
		errChan:     make(chan error, 1),
		ctx:         ctx,
	}

	go func() {
		// Use a dedicated client for Pub/Sub from the main client to avoid multiplexing issues in some environments.
		dedic, cancelDedic := client.Dedicate()
		defer cancelDedic()

		err := dedic.Receive(ctx, dedic.B().Subscribe().Channel(subscribeChan).Build(), func(msg rueidis.PubSubMessage) {
			select {
			case <-ctx.Done():
				return
			case t.msgChan <- []byte(msg.Message):
			}
		})
		if err != nil {
			select {
			case <-ctx.Done():
			case t.errChan <- err:
			}
		}
	}()

	return t
}

// Send publishes a message to the target Redis channel.
func (t *RedisPubSubTransport) Send(ctx context.Context, message []byte) error {
	cmd := t.client.B().Publish().Channel(t.publishChan).Message(string(message)).Build()
	return t.client.Do(ctx, cmd).Error()
}

// Receive waits for a message from the subscribed Redis channel.
func (t *RedisPubSubTransport) Receive(ctx context.Context) ([]byte, error) {
	select {
	case <-ctx.Done():
		return nil, ctx.Err()
	case <-t.ctx.Done():
		return nil, context.Canceled
	case err := <-t.errChan:
		return nil, err
	case msg := <-t.msgChan:
		return msg, nil
	}
}

// Close closes the underlying Redis Pub/Sub subscription.
func (t *RedisPubSubTransport) Close() error {
	t.cancel()
	return nil
}
