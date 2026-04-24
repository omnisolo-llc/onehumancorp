package nats

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/nats-io/nats-server/v2/server"
	"github.com/nats-io/nats.go"
	pb "github.com/onehumancorp/mono/src/proto"
	"google.golang.org/protobuf/proto"
	"github.com/onehumancorp/mono/src/server/msgbus"
		"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	meter          = otel.Meter("github.com/onehumancorp/mono/src/server/integrations/nats")
	messagesPub    metric.Int64Counter
	messagesRecv   metric.Int64Counter
	metricsInitErr error
	metricsOnce    sync.Once
)

func initMetrics() {
	metricsOnce.Do(func() {
		messagesPub, metricsInitErr = meter.Int64Counter("ohc.nats.messages_published",
			metric.WithDescription("Number of messages published to NATS"),
		)
		if metricsInitErr != nil {
			fmt.Printf("Failed to initialize messagesPub metric: %v", metricsInitErr)
			return
		}
		messagesRecv, metricsInitErr = meter.Int64Counter("ohc.nats.messages_received",
			metric.WithDescription("Number of messages received from NATS"),
		)
		if metricsInitErr != nil {
			fmt.Printf("Failed to initialize messagesRecv metric: %v", metricsInitErr)
			return
		}
	})
}

// Config for NATS connection
type Config struct {
	URL      string
	IsLocal  bool // Run embedded NATS if local
}

// NatsIntegration implements IntegrationProvider and an event mesh using NATS.
type NatsIntegration struct {
	cfg        Config
	conn       *nats.Conn
	js         nats.JetStreamContext
	embedSrv   *server.Server
	subs       map[string]*nats.Subscription
	mu         sync.Mutex
}

// NewNatsIntegration initializes a NATS connection. If config.IsLocal is true,
// it embeds a NATS server.
func NewNatsIntegration(cfg Config) (*NatsIntegration, error) {
	initMetrics()
	n := &NatsIntegration{
		cfg:  cfg,
		subs: make(map[string]*nats.Subscription),
	}

	if cfg.IsLocal {
		opts := &server.Options{
			Host: "127.0.0.1",
			Port: server.RANDOM_PORT,
			NoLog: true,
			NoSigs: true,
		}
		srv, err := server.NewServer(opts)
		if err != nil {
			return nil, fmt.Errorf("failed to create embedded NATS server: %w", err)
		}
		go srv.Start()
		if !srv.ReadyForConnections(5 * time.Second) {
			return nil, fmt.Errorf("embedded NATS server failed to start")
		}
		n.embedSrv = srv
		cfg.URL = srv.ClientURL()
	}

	if cfg.URL == "" {
		cfg.URL = nats.DefaultURL
	}

	nc, err := nats.Connect(cfg.URL)
	if err != nil {
		if n.embedSrv != nil {
			n.embedSrv.Shutdown()
		}
		return nil, fmt.Errorf("nats connect %q: %w", cfg.URL, err)
	}
	n.conn = nc

	js, err := nc.JetStream()
	if err != nil {
		nc.Close()
		if n.embedSrv != nil {
			n.embedSrv.Shutdown()
		}
		return nil, fmt.Errorf("nats jetstream: %w", err)
	}
	n.js = js

	return n, nil
}

func (n *NatsIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("nats"),
		Name:        proto.String("NATS Event Mesh"),
		Description: proto.String("Hybrid event mesh using NATS and JetStream."),
		Category:    proto.String("messaging"),
		Icon:        proto.String("https://nats.io/img/logo.png"),
	}.Build()
}

func (n *NatsIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connect to NATS"),
			Description: proto.String("Configure NATS event mesh connection parameters."),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("url"),
					Label:       proto.String("NATS Server URL"),
					Type:        proto.String("text"),
					Required:    proto.Bool(false),
					Description: proto.String("Leave empty to use embedded local NATS server."),
				}.Build(),
			},
		}.Build(),
	}
}

// Publish publishes a message to a NATS subject.
func (n *NatsIntegration) Publish(ctx context.Context, subject string, data []byte) error {
	if n.conn == nil {
		return fmt.Errorf("nats integration not connected")
	}
	err := n.conn.Publish(subject, data)
	if err == nil && metricsInitErr == nil {
		messagesPub.Add(ctx, 1)
	}
	return err
}

// Subscribe subscribes to a NATS subject and executes the handler on message receipt.
func (n *NatsIntegration) Subscribe(subject string, handler func(msg *msgbus.Message)) error {
	if n.conn == nil {
		return fmt.Errorf("nats integration not connected")
	}
	n.mu.Lock()
	defer n.mu.Unlock()

	if _, exists := n.subs[subject]; exists {
		return fmt.Errorf("already subscribed to %q", subject)
	}

	sub, err := n.conn.Subscribe(subject, func(m *nats.Msg) {
		if metricsInitErr == nil {
			messagesRecv.Add(context.Background(), 1)
		}
		handler(&msgbus.Message{
			Topic:   m.Subject,
			Payload: m.Data,
		})
	})
	if err != nil {
		return err
	}
	n.subs[subject] = sub
	return nil
}

// Close gracefully closes the NATS connection and the embedded server if running.
func (n *NatsIntegration) Close() {
	n.mu.Lock()
	for _, sub := range n.subs {
		sub.Unsubscribe()
	}
	n.subs = make(map[string]*nats.Subscription)
	n.mu.Unlock()

	if n.conn != nil {
		n.conn.Close()
	}
	if n.embedSrv != nil {
		n.embedSrv.Shutdown()
		n.embedSrv.WaitForShutdown()
	}
}
