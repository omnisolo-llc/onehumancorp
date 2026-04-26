package obsidian

import (
	pb "github.com/onehumancorp/mono/src/proto"
)

type ObsidianIntegration struct{}

func (s *ObsidianIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "obsidian",
		Name:        "Obsidian",
		Type:        "obsidian",
		Category:    "Knowledge Base",
		Description: "Local Markdown-based knowledge management via Obsidian MCP.",
		Publisher:   "Obsidian",
		Icon:        "https://obsidian.md/favicon.ico",
		Tags:        []string{"knowledge", "local", "mcp", "markdown"}}
}

func (s *ObsidianIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title:       "Obsidian Configuration",
			Description: "Configure Obsidian local vault path",
			Fields: []*pb.WizardField{
				&pb.WizardField{
					Key:         "vault_path",
					Label:       "Vault Path",
					Description: "The local filesystem path to your Obsidian vault",
					Type:        "text",
					Required:    true,
				},
			},
		},
	}
}
