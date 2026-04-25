package libsql

import (
	pb "github.com/onehumancorp/mono/src/proto"
	"google.golang.org/protobuf/proto"
)

type LibSQLIntegration struct{}

func (s *LibSQLIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("libsql"),
		Name:        proto.String("LibSQL"),
		Type:        proto.String("libsql"),
		Category:    proto.String("Database"),
		Description: proto.String("LibSQL MCP for Distributed Edge SQLite Synchronization."),
		Publisher:   proto.String("Turso"),
		Icon:        proto.String("https://turso.tech/logomark.svg"),
		Tags:        []string{"sqlite", "distributed", "edge"}}.Build()
}

func (s *LibSQLIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Data"),
			Description: proto.String("Configure LibSQL MCP credentials"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("url"),
					Label:       proto.String("LibSQL URL"),
					Description: proto.String("The URL of the LibSQL endpoint"),
					Type:        proto.String("url"),
					Required:    proto.Bool(true),
				}.Build(),
				pb.WizardField_builder{
					Key:         proto.String("authToken"),
					Label:       proto.String("Auth Token"),
					Description: proto.String("The Authentication Token for LibSQL"),
					Type:        proto.String("password"),
					Required:    proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}
