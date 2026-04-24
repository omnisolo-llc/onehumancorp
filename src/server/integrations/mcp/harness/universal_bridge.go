package harness

import (
    "context"
    "io"

    "github.com/redis/go-redis/v9"
)

// TransportBridge abstracts the communication medium for agents.
type TransportBridge interface {
    Send(ctx context.Context, message []byte) error
    Receive(ctx context.Context) ([]byte, error)
    Close() error
}

// NewUniversalBridge creates a transport bridge.
// Mode can be "LOCAL" or "CLOUD".
func NewUniversalBridge(mode string, reader io.Reader, writer io.Writer, redisClient *redis.Client, pubChannel, subChannel string) TransportBridge {
    if mode == "CLOUD" && redisClient != nil {
        return NewCloudTransport(redisClient, pubChannel, subChannel)
    }
    return NewLocalTransport(reader, writer)
}
