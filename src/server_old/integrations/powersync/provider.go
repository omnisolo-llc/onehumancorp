package powersync

import (
	pb "github.com/onehumancorp/mono/src/proto"
)

type PowerSyncIntegration struct{}

func (s *PowerSyncIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "powersync",
		Name:        "PowerSync",
		Type:        "powersync",
		Category:    "Database",
		Description: "PowerSync MCP for Hybrid SQLite-to-PostgreSQL Synchronization.",
		Publisher:   "JourneyApps",
		Icon:        "https://www.powersync.com/favicon.ico",
		Tags:        []string{"sqlite", "postgres", "sync", "offline"},
	}
}

func (s *PowerSyncIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title:       "Connection Data",
			Description: "Configure PowerSync MCP credentials",
			Fields: []*pb.WizardField{
				&pb.WizardField{
					Key:         "url",
					Label:       "PowerSync URL",
					Description: "The URL of the PowerSync instance",
					Type:        "url",
					Required:    true,
				},
			},
		},
	}
}
