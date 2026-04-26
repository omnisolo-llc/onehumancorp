package msgbus

import (
	"context"
	"fmt"

	nats "github.com/nats-io/nats.go"
)

// NATSBus is a Bus backed by NATS.  It supports both embedded (standalone
// desktop) and external NATS servers (for multi-process deployments).
type NATSBus struct {
	conn    *nats.Conn
	timeout nats.Option
}

func newNATSBus(cfg Config) (*NATSBus, error) {
	urls := nats.DefaultURL
	if len(cfg.NATSURLs) > 0 {
		urls = cfg.NATSURLs[0]
		for _, u := range cfg.NATSURLs[1:] {
			urls += "," + u
		}
	}

	opts := []nats.Option{
		nats.Timeout(cfg.PublishTimeout),
		nats.MaxReconnects(-1), // reconnect indefinitely
		nats.Name("ohc-agent"),
	}

	conn, err := nats.Connect(urls, opts...)
	if err != nil {
		return nil, fmt.Errorf("msgbus/nats: connect %q: %w", urls, err)
	}
	return &NATSBus{conn: conn}, nil
}

// Publish publishes msg.Payload to msg.Topic.
func (b *NATSBus) Publish(_ context.Context, msg Message) error {
	return b.conn.Publish(msg.Topic, msg.Payload)
}

// Subscribe registers handler for all messages on topic.
func (b *NATSBus) Subscribe(topic string, handler Handler) (func(), error) {
	sub, err := b.conn.Subscribe(topic, func(m *nats.Msg) {
		handler(Message{Topic: m.Subject, Payload: m.Data})
	})
	if err != nil {
		return nil, fmt.Errorf("msgbus/nats: subscribe %q: %w", topic, err)
	}
	return func() { _ = sub.Unsubscribe() }, nil
}

// Close drains and closes the NATS connection.
func (b *NATSBus) Close() error {
	b.conn.Close()
	return nil
}
