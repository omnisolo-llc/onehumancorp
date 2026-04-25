package nats

import (
	pb "github.com/onehumancorp/mono/src/proto"
	"google.golang.org/protobuf/proto"
)

type NatsIntegration struct{}

func (s *NatsIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("nats"),
		Name:        proto.String("NATS"),
		Type:        proto.String("nats"),
		Category:    proto.String("Event Mesh"),
		Description: proto.String("NATS Hybrid Event Mesh for Standalone to Cloud coordination."),
		Publisher:   proto.String("Synadia"),
		Icon:        proto.String("https://nats.io/img/logo.svg"),
		Tags:        []string{"messaging", "event-mesh", "pubsub", "hybrid"}}.Build()
}

func (s *NatsIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Configuration"),
			Description: proto.String("Configure NATS connection credentials"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("url"),
					Label:       proto.String("NATS URL"),
					Description: proto.String("The remote NATS URL. Leave empty to use embedded local server."),
					Type:        proto.String("url"),
					Required:    proto.Bool(false),
				}.Build(),
				pb.WizardField_builder{
					Key:         proto.String("credentialsPath"),
					Label:       proto.String("Credentials File Path"),
					Description: proto.String("Path to the NATS user credentials file (.creds)"),
					Type:        proto.String("text"),
					Required:    proto.Bool(false),
				}.Build(),
			},
		}.Build(),
	}
}
