package agents

import (
	"context"
	"io"
	"os"

	"github.com/redis/go-redis/v9"
)

// UniversalTransportBridge implements Transport interface and dynamically routes traffic
// to either InProcessTransport (Standalone mode) or RedisPubSubTransport (Cloud mode).
type UniversalTransportBridge struct {
	transport Transport
}

// NewUniversalTransportBridge creates a new UniversalTransportBridge based on the current environment.
func NewUniversalTransportBridge(client redis.UniversalClient, publishChan, subscribeChan string, reader io.Reader, writer io.Writer) *UniversalTransportBridge {
	var t Transport
	if os.Getenv("OHC_STANDALONE") == "true" {
		t = NewInProcessTransport(reader, writer)
	} else {
		t = NewRedisPubSubTransport(client, publishChan, subscribeChan)
	}

	return &UniversalTransportBridge{
		transport: t,
	}
}

// Send delegates the message sending to the underlying transport.
func (b *UniversalTransportBridge) Send(ctx context.Context, message []byte) error {
	return b.transport.Send(ctx, message)
}

// Receive delegates the message receiving to the underlying transport.
func (b *UniversalTransportBridge) Receive(ctx context.Context) ([]byte, error) {
	return b.transport.Receive(ctx)
}

// Close delegates the closing to the underlying transport.
func (b *UniversalTransportBridge) Close() error {
	return b.transport.Close()
}
