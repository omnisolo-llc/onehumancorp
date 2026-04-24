package nats

import (
	"context"
	"fmt"
	"sync"

	"github.com/nats-io/nats.go"
	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"google.golang.org/protobuf/proto"
)

type NatsIntegration struct {
	nc *nats.Conn
	js nats.JetStreamContext
	mu sync.RWMutex
}

func (n *NatsIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("nats"),
		Name:        proto.String("NATS Event Mesh"),
		Description: proto.String("Hybrid Event Mesh integration using NATS and JetStream"),
		Category:    proto.String("Event Bus"),
		Type:        proto.String("nats"),
	}.Build()
}

func (n *NatsIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connect to NATS"),
			Description: proto.String("Provide the NATS server URL"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:      proto.String("url"),
					Label:    proto.String("NATS URL"),
					Type:     proto.String("text"),
					Required: proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}

func (n *NatsIntegration) Connect(url string) error {
	n.mu.Lock()
	defer n.mu.Unlock()

	nc, err := nats.Connect(url)
	if err != nil {
		return fmt.Errorf("failed to connect to NATS: %w", err)
	}

	js, err := nc.JetStream()
	if err != nil {
		nc.Close()
		return fmt.Errorf("failed to initialize JetStream: %w", err)
	}

	n.nc = nc
	n.js = js
	return nil
}

func (n *NatsIntegration) Close() {
	n.mu.Lock()
	defer n.mu.Unlock()
	if n.nc != nil {
		n.nc.Close()
	}
}

func (n *NatsIntegration) Publish(ctx context.Context, subject string, data []byte) error {
	n.mu.RLock()
	defer n.mu.RUnlock()

	if n.nc == nil {
		return fmt.Errorf("not connected to NATS")
	}

	err := n.nc.Publish(subject, data)
	if err == nil {
		telemetry.RecordNatsMessagesPublished(ctx, 1)
	}
	return err
}

func (n *NatsIntegration) Subscribe(ctx context.Context, subject string, handler func(msg []byte)) (*nats.Subscription, error) {
	n.mu.RLock()
	defer n.mu.RUnlock()

	if n.nc == nil {
		return nil, fmt.Errorf("not connected to NATS")
	}

	sub, err := n.nc.Subscribe(subject, func(m *nats.Msg) {
		telemetry.RecordNatsMessagesReceived(context.Background(), 1)
		handler(m.Data)
	})
	if err != nil {
		return nil, err
	}

	return sub, nil
}
