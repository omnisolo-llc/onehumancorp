package powersync

import (
	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/protobuf/proto"
)

type PowerSyncIntegration struct{}

func (s *PowerSyncIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("powersync"),
		Name:        proto.String("PowerSync"),
		Type:        proto.String("powersync"),
		Category:    proto.String("Database"),
		Description: proto.String("PowerSync MCP for Hybrid SQLite-to-Postgres Synchronization"),
		Publisher:   proto.String("JourneyApps"),
		Icon:        proto.String("https://www.powersync.com/favicon.ico"),
		Tags:        []string{"sqlite", "postgres", "sync", "hybrid"},
	}.Build()
}

func (s *PowerSyncIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Data"),
			Description: proto.String("Configure PowerSync MCP credentials"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("url"),
					Label:       proto.String("PowerSync URL"),
					Description: proto.String("The URL of the PowerSync instance"),
					Type:        proto.String("url"),
					Required:    proto.Bool(true),
				}.Build(),
				pb.WizardField_builder{
					Key:         proto.String("token"),
					Label:       proto.String("Authentication Token"),
					Description: proto.String("The token for authentication"),
					Type:        proto.String("password"),
					Required:    proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}
