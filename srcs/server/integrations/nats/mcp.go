package nats

import (
	"context"
	"encoding/json"
	"fmt"
	"github.com/nats-io/nats.go"
)

type MCPTool struct {
	nc *nats.Conn
	js nats.JetStreamContext
}

func NewMCPTool(url string) (*MCPTool, error) {
	nc, err := nats.Connect(url)
	if err != nil {
		return nil, fmt.Errorf("failed to connect to nats: %v", err)
	}
	js, err := nc.JetStream()
	if err != nil {
		nc.Close()
		return nil, fmt.Errorf("failed to get jetstream context: %v", err)
	}
	return &MCPTool{nc: nc, js: js}, nil
}

func (t *MCPTool) Close() {
	if t.nc != nil {
		t.nc.Close()
	}
}

func (t *MCPTool) Publish(ctx context.Context, subject string, data []byte) error {
	_, err := t.js.Publish(subject, data)
	return err
}

func (t *MCPTool) SubscribeSync(ctx context.Context, subject string) (*nats.Subscription, error) {
	return t.js.SubscribeSync(subject)
}

func (t *MCPTool) PublishJSON(ctx context.Context, subject string, payload interface{}) error {
	data, err := json.Marshal(payload)
	if err != nil {
		return err
	}
	return t.Publish(ctx, subject, data)
}
