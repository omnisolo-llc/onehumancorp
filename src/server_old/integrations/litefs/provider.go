package litefs

import (
	pb "github.com/onehumancorp/mono/src/proto"
)

type LiteFSIntegration struct{}

func (s *LiteFSIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "litefs",
		Name:        "LiteFS",
		Type:        "litefs",
		Category:    "Database",
		Description: "LiteFS MCP for Distributed Local-First SQLite Synchronization.",
		Publisher:   "Fly.io",
		Icon:        "https://fly.io/ui/images/litefs-logo.svg",
		Tags:        []string{"sqlite", "distributed", "local-first"}}
}

func (s *LiteFSIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title:       "Connection Data",
			Description: "Configure LiteFS MCP credentials",
			Fields: []*pb.WizardField{
				&pb.WizardField{
					Key:         "url",
					Label:       "LiteFS URL",
					Description: "The URL of the LiteFS endpoint",
					Type:        "url",
					Required:    true,
				},
			},
		},
	}
}
