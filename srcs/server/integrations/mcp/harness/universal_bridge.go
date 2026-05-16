package harness

import (
	"context"
	"io"
	"os"

	"github.com/redis/go-redis/v9"
)

// TransportBridge interface abstracts communication for agents.
type TransportBridge interface {
	Send(ctx context.Context, message []byte) error
	Receive(ctx context.Context) ([]byte, error)
	Close() error
}

// NewUniversalBridge dynamically wires InProcessTransport (Local) or CloudTransport based on execution mode.
func NewUniversalBridge(stdin io.Reader, stdout io.Writer, channelID string) TransportBridge {
	mode := os.Getenv("OHC_EXECUTION_MODE")
	if mode == "cloud" {
		redisURL := os.Getenv("REDIS_URL")
		if redisURL == "" {
			redisURL = "redis://localhost:6379"
		}
		opts, err := redis.ParseURL(redisURL)
		if err != nil {
			// Fallback if bad URL
			if stdin == nil || stdout == nil {
				return nil
			}
			return NewLocalTransport(stdin, stdout)
		}
		client := redis.NewClient(opts)
		return NewCloudTransport(client, channelID)
	}
	if stdin == nil || stdout == nil {
				return nil
			}
			return NewLocalTransport(stdin, stdout)
}
