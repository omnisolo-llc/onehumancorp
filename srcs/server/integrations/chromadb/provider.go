package chromadb

import (
	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/protobuf/proto"
)

type ChromaDBIntegration struct{}

func (s *ChromaDBIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("chromadb"),
		Name:        proto.String("ChromaDB"),
		Type:        proto.String("chromadb"),
		Category:    proto.String("Database"),
		Description: proto.String("Local Vector Embedded Integrations via ChromaDB MCP."),
		Publisher:   proto.String("ChromaDB"),
		Icon:        proto.String("https://docs.trychroma.com/img/chroma.png"),
		Tags:        []string{"vector", "database", "local", "mcp"},
	}.Build()
}

func (s *ChromaDBIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Data"),
			Description: proto.String("Configure ChromaDB MCP connection"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("url"),
					Label:       proto.String("ChromaDB URL"),
					Description: proto.String("The URL of the local ChromaDB endpoint"),
					Type:        proto.String("url"),
					Required:    proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}
