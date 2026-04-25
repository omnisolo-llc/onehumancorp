package nats

import (
	pb "github.com/onehumancorp/mono/src/proto"
	"github.com/onehumancorp/mono/src/server/integrations"
	"google.golang.org/protobuf/proto"
)

// NatsIntegration provides a NATS hybrid event mesh implementation.
type NatsIntegration struct{}

// Metadata returns the integration metadata for NATS.
func (s *NatsIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("nats"),
		Name:        proto.String("NATS"),
		Type:        proto.String("nats"),
		Category:    proto.String(string(integrations.CategoryEventMesh)),
		Description: proto.String("High-performance hybrid event mesh for real-time messaging and JetStream KV."),
		Publisher:   proto.String("Synadia Communications Inc."),
		Icon:        proto.String("https://upload.wikimedia.org/wikipedia/commons/e/e4/NATS_Logo.svg"),
		Tags:        []string{"event-mesh", "pubsub", "hybrid"},
	}.Build()
}

// WizardSteps returns the configuration steps for the NATS integration.
func (s *NatsIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title: proto.String("Cluster Connection"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{Key: proto.String("server_url"), Label: proto.String("Server URL"), Type: proto.String("url"), Required: proto.Bool(true)}.Build(),
				pb.WizardField_builder{Key: proto.String("credentials_file"), Label: proto.String("Credentials File Path"), Type: proto.String("text"), Required: proto.Bool(false)}.Build(),
			},
		}.Build(),
		pb.WizardStep_builder{
			Title: proto.String("Local Leaf Node"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{Key: proto.String("enable_leaf"), Label: proto.String("Enable embedded leaf node for offline mode"), Type: proto.String("boolean"), Required: proto.Bool(false)}.Build(),
			},
		}.Build(),
	}
}
