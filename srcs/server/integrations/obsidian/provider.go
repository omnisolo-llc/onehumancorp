package obsidian

import (
	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/protobuf/proto"
)

type ObsidianIntegration struct{}

func (s *ObsidianIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("obsidian"),
		Name:        proto.String("Obsidian"),
		Type:        proto.String("obsidian"),
		Category:    proto.String("Knowledge Base"),
		Description: proto.String("Obsidian MCP for Local-First Knowledge Base Synchronization."),
		Publisher:   proto.String("Obsidian"),
		Icon:        proto.String("https://obsidian.md/images/obsidian-logo-gradient.svg"),
		Tags:        []string{"obsidian", "markdown", "knowledge base", "local-first"}}.Build()
}

func (s *ObsidianIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Data"),
			Description: proto.String("Configure Obsidian MCP folder path"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("path"),
					Label:       proto.String("Obsidian Vault Path"),
					Description: proto.String("The local path to your Obsidian vault"),
					Type:        proto.String("text"),
					Required:    proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}
