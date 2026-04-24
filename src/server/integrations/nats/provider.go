package nats

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/nats-io/nats.go"
	"github.com/nats-io/nats-server/v2/server"
	pb "github.com/onehumancorp/mono/src/proto"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
	"google.golang.org/protobuf/proto"
)

type NatsIntegration struct {
	mu               sync.RWMutex
	Conn             *nats.Conn
	Server           *server.Server
	publishedCounter metric.Int64Counter
	receivedCounter  metric.Int64Counter
}

func NewNatsIntegration() *NatsIntegration {
	meter := otel.Meter("github.com/onehumancorp/mono/src/server/integrations/nats")

	pubCounter, _ := meter.Int64Counter(
		"ohc.nats.messages_published",
		metric.WithDescription("Number of messages published to NATS"),
	)

	recCounter, _ := meter.Int64Counter(
		"ohc.nats.messages_received",
		metric.WithDescription("Number of messages received from NATS"),
	)

	return &NatsIntegration{
		publishedCounter: pubCounter,
		receivedCounter:  recCounter,
	}
}

func (s *NatsIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("nats"),
		Name:        proto.String("NATS"),
		Type:        proto.String("nats"),
		Category:    proto.String("Event Mesh"),
		Description: proto.String("NATS Hybrid Event Mesh Integration."),
		Publisher:   proto.String("Synadia"),
		Icon:        proto.String("https://nats.io/img/logo.svg"),
		Tags:        []string{"nats", "event mesh", "pubsub", "hybrid"},
	}.Build()
}

func (s *NatsIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Settings"),
			Description: proto.String("Configure NATS connection details"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("url"),
					Label:       proto.String("NATS Server URL"),
					Description: proto.String("The URL of the NATS server cluster"),
					Type:        proto.String("url"),
					Required:    proto.Bool(true),
				}.Build(),
				pb.WizardField_builder{
					Key:         proto.String("credentials"),
					Label:       proto.String("Credentials"),
					Description: proto.String("NATS Credentials (NKEY/JWT)"),
					Type:        proto.String("password"),
					Required:    proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}

func (s *NatsIntegration) Connect(url string, credentials string, embedded bool, embeddedPort int) error {
	s.mu.Lock()
	defer s.mu.Unlock()

	var err error
	connectUrl := url

	if embedded {
		opts := &server.Options{
			Port: embeddedPort,
		}
		s.Server, err = server.NewServer(opts)
		if err != nil {
			return fmt.Errorf("failed to create embedded NATS server: %w", err)
		}

		go s.Server.Start()
		if !s.Server.ReadyForConnections(10 * time.Second) {
			return fmt.Errorf("embedded NATS server failed to start")
		}

		connectUrl = s.Server.ClientURL()
	}

	opts := []nats.Option{
		nats.Timeout(5 * time.Second),
	}
	if credentials != "" {
		opts = append(opts, nats.UserCredentials(credentials))
	}

	s.Conn, err = nats.Connect(connectUrl, opts...)
	if err != nil {
		if s.Server != nil {
			s.Server.Shutdown()
			s.Server = nil
		}
		return fmt.Errorf("failed to connect to NATS: %w", err)
	}

	return nil
}

func (s *NatsIntegration) Disconnect() error {
	s.mu.Lock()
	defer s.mu.Unlock()

	if s.Conn != nil {
		s.Conn.Close()
		s.Conn = nil
	}

	if s.Server != nil {
		s.Server.Shutdown()
		s.Server.WaitForShutdown()
		s.Server = nil
	}

	return nil
}

func (s *NatsIntegration) Publish(ctx context.Context, subject string, data []byte) error {
	s.mu.RLock()
	defer s.mu.RUnlock()

	if s.Conn == nil {
		return fmt.Errorf("NATS connection is not established")
	}

	err := s.Conn.Publish(subject, data)
	if err != nil {
		return fmt.Errorf("failed to publish message: %w", err)
	}

	if s.publishedCounter != nil {
		s.publishedCounter.Add(ctx, 1)
	}

	return nil
}

func (s *NatsIntegration) Subscribe(ctx context.Context, subject string, handler func(msg []byte)) (*nats.Subscription, error) {
	s.mu.RLock()
	defer s.mu.RUnlock()

	if s.Conn == nil {
		return nil, fmt.Errorf("NATS connection is not established")
	}

	sub, err := s.Conn.Subscribe(subject, func(m *nats.Msg) {
		if s.receivedCounter != nil {
			s.receivedCounter.Add(context.Background(), 1)
		}
		handler(m.Data)
	})

	if err != nil {
		return nil, fmt.Errorf("failed to subscribe to subject: %w", err)
	}

	return sub, nil
}
