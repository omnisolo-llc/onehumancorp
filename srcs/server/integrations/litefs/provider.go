package litefs

import (
	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/protobuf/proto"
)

type LiteFSIntegration struct{}

func (s *LiteFSIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("litefs"),
		Name:        proto.String("LiteFS"),
		Type:        proto.String("litefs"),
		Category:    proto.String("Database"),
		Description: proto.String("LiteFS MCP for Distributed Local-First SQLite Synchronization."),
		Publisher:   proto.String("Fly.io"),
		Icon:        proto.String("https://fly.io/ui/images/litefs-logo.svg"),
		Tags:        []string{"sqlite", "distributed", "local-first"}}.Build()
}

func (s *LiteFSIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Data"),
			Description: proto.String("Configure LiteFS MCP credentials"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("url"),
					Label:       proto.String("LiteFS URL"),
					Description: proto.String("The URL of the LiteFS endpoint"),
					Type:        proto.String("url"),
					Required:    proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}
