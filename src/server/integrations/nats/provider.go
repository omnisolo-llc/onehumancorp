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
		Description: proto.String("NATS Hybrid Event Mesh Integration."),
		Publisher:   proto.String("Synadia"),
		Icon:        proto.String("https://nats.io/img/logo.svg"),
		Tags:        []string{"nats", "event mesh", "pubsub", "hybrid"}}.Build()
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
