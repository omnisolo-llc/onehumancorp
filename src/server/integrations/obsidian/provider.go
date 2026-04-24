package obsidian

import (
	pb "github.com/onehumancorp/mono/src/proto"
	"google.golang.org/protobuf/proto"
)

type ObsidianIntegration struct{}

func (s *ObsidianIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("obsidian"),
		Name:        proto.String("Obsidian"),
		Type:        proto.String("obsidian"),
		Category:    proto.String("Knowledge Base"),
		Description: proto.String("Local Markdown-based knowledge management via Obsidian MCP."),
		Publisher:   proto.String("Obsidian"),
		Icon:        proto.String("https://obsidian.md/favicon.ico"),
		Tags:        []string{"knowledge", "local", "mcp", "markdown"}}.Build()
}

func (s *ObsidianIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Obsidian Configuration"),
			Description: proto.String("Configure Obsidian local vault path"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("vault_path"),
					Label:       proto.String("Vault Path"),
					Description: proto.String("The local filesystem path to your Obsidian vault"),
					Type:        proto.String("text"),
					Required:    proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}
