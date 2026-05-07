package harness

import (
	"context"
	"fmt"
	"github.com/redis/go-redis/v9"
)

// CloudTransport wraps Redis Pub/Sub
type CloudTransport struct {
	client *redis.Client
	pubSub *redis.PubSub
	channelReq string
	channelRes string
}

func NewCloudTransport(client *redis.Client, channelID string) *CloudTransport {
	return &CloudTransport{
		client: client,
		pubSub: client.Subscribe(context.Background(), channelID+"_res"),
		channelReq: channelID+"_req",
		channelRes: channelID+"_res",
	}
}

func (t *CloudTransport) Send(ctx context.Context, message []byte) error {
	return t.client.Publish(ctx, t.channelReq, message).Err()
}

func (t *CloudTransport) Receive(ctx context.Context) ([]byte, error) {
	msg, err := t.pubSub.ReceiveMessage(ctx)
	if err != nil {
		return nil, fmt.Errorf("redis receive error: %w", err)
	}
	return []byte(msg.Payload), nil
}

func (t *CloudTransport) Close() error {
	t.pubSub.Close()
	return t.client.Close()
}
