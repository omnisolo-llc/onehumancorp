package nats

import (
	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/protobuf/proto"
	"github.com/nats-io/nats-server/v2/server"
	"fmt"
	"time"
	"os"
)

type NATSIntegration struct{}

func (s *NATSIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("nats"),
		Name:        proto.String("NATS JetStream"),
		Type:        proto.String("nats"),
		Category:    proto.String("Messaging"),
		Description: proto.String("NATS JetStream MCP for Hybrid Swarm Messaging."),
		Publisher:   proto.String("Synadia"),
		Icon:        proto.String("https://nats.io/img/logo.png"),
		Tags:        []string{"messaging", "pubsub", "jetstream", "mesh"}}.Build()
}

func (s *NATSIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Data"),
			Description: proto.String("Configure NATS Server credentials"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("url"),
					Label:       proto.String("NATS URL"),
					Description: proto.String("The URL of the NATS server"),
					Type:        proto.String("url"),
					Required:    proto.Bool(true),
				}.Build(),
				pb.WizardField_builder{
					Key:         proto.String("creds"),
					Label:       proto.String("Credentials File"),
					Description: proto.String("Path or contents of the NATS user credentials file"),
					Type:        proto.String("password"),
					Required:    proto.Bool(false),
				}.Build(),
			},
		}.Build(),
	}
}

func StartEmbeddedServerIfNeeded() (*server.Server, error) {
	mode := os.Getenv("OHC_STANDALONE")
	if mode == "true" {
		opts := &server.Options{}
		opts.Host = "127.0.0.1"
		opts.Port = server.RANDOM_PORT
		opts.JetStream = true

		ns, err := server.NewServer(opts)
		if err != nil {
			return nil, fmt.Errorf("error creating server: %v", err)
		}

		go ns.Start()
		if !ns.ReadyForConnections(10 * time.Second) {
			return nil, fmt.Errorf("server not ready")
		}

		return ns, nil
	}

	// Not in standalone mode, assume cloud mode and external NATS connection
	return nil, nil
}
