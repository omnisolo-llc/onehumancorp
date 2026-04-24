package weaviate

import (
	pb "github.com/onehumancorp/mono/src/proto"
	"google.golang.org/protobuf/proto"
)

type WeaviateIntegration struct{}

func (s *WeaviateIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("weaviate"),
		Name:        proto.String("Weaviate"),
		Type:        proto.String("weaviate"),
		Category:    proto.String("Vector Database"),
		Description: proto.String("Advanced hybrid vector search via Weaviate MCP."),
		Publisher:   proto.String("Weaviate"),
		Icon:        proto.String("https://weaviate.io/img/site/weaviate-nav-logo.svg"),
		Tags:        []string{"vector", "database", "mcp", "search"}}.Build()
}

func (s *WeaviateIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Data"),
			Description: proto.String("Configure Weaviate MCP connection"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("url"),
					Label:       proto.String("Weaviate URL"),
					Description: proto.String("The URL of the Weaviate instance"),
					Type:        proto.String("url"),
					Required:    proto.Bool(true),
				}.Build(),
				pb.WizardField_builder{
					Key:         proto.String("api_key"),
					Label:       proto.String("API Key"),
					Description: proto.String("Optional API Key for authentication"),
					Type:        proto.String("password"),
					Required:    proto.Bool(false),
				}.Build(),
			},
		}.Build(),
	}
}
