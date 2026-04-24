package agents

import (
	"context"
	"fmt"

	"github.com/nats-io/nats.go"
)

// NatsTransport implements Transport interface using NATS.
type NatsTransport struct {
	nc             *nats.Conn
	sub            *nats.Subscription
	subChan        chan *nats.Msg
	publishSubject string
}

// NewNatsTransport creates a new NatsTransport.
func NewNatsTransport(url, publishSubject, subscribeSubject string) (*NatsTransport, error) {
	nc, err := nats.Connect(url)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to NATS: %w", err)
	}

	subChan := make(chan *nats.Msg, 64)
	sub, err := nc.ChanSubscribe(subscribeSubject, subChan)
	if err != nil {
		nc.Close()
		return nil, fmt.Errorf("failed to subscribe to NATS subject %s: %w", subscribeSubject, err)
	}

	return &NatsTransport{
		nc:             nc,
		sub:            sub,
		subChan:        subChan,
		publishSubject: publishSubject,
	}, nil
}

// Send publishes a message to the NATS subject.
func (t *NatsTransport) Send(ctx context.Context, message []byte) error {
	select {
	case <-ctx.Done():
		return ctx.Err()
	default:
		return t.nc.Publish(t.publishSubject, message)
	}
}

// Receive waits for a message from the subscribed NATS subject.
func (t *NatsTransport) Receive(ctx context.Context) ([]byte, error) {
	select {
	case <-ctx.Done():
		return nil, ctx.Err()
	case msg, ok := <-t.subChan:
		if !ok {
			return nil, context.Canceled
		}
		return msg.Data, nil
	}
}

// Close closes the NATS connection and subscription.
func (t *NatsTransport) Close() error {
	t.sub.Unsubscribe() //nolint:errcheck
	t.nc.Close()
	return nil
}
