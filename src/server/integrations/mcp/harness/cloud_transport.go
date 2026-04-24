package harness

import (
    "context"

    "github.com/redis/go-redis/v9"
)

type CloudTransport struct {
    client     *redis.Client
    pubChannel string
    sub        *redis.PubSub
    msgChan    <-chan *redis.Message
}

func NewCloudTransport(client *redis.Client, pubChannel, subChannel string) *CloudTransport {
    sub := client.Subscribe(context.Background(), subChannel)
    return &CloudTransport{
        client:     client,
        pubChannel: pubChannel,
        sub:        sub,
        msgChan:    sub.Channel(),
    }
}

func (c *CloudTransport) Send(ctx context.Context, message []byte) error {
    return c.client.Publish(ctx, c.pubChannel, message).Err()
}

func (c *CloudTransport) Receive(ctx context.Context) ([]byte, error) {
    select {
    case <-ctx.Done():
        return nil, ctx.Err()
    case msg, ok := <-c.msgChan:
        if !ok {
            return nil, context.Canceled
        }
        return []byte(msg.Payload), nil
    }
}

func (c *CloudTransport) Close() error {
    return c.sub.Close()
}
