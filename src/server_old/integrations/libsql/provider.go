package libsql

import (
	pb "github.com/onehumancorp/mono/src/proto"
)

type LibSQLIntegration struct{}

func (s *LibSQLIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "libsql",
		Name:        "LibSQL",
		Type:        "libsql",
		Category:    "Database",
		Description: "LibSQL MCP for Distributed Edge SQLite Synchronization.",
		Publisher:   "Turso",
		Icon:        "https://turso.tech/logomark.svg",
		Tags:        []string{"sqlite", "distributed", "edge"}}
}

func (s *LibSQLIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title:       "Connection Data",
			Description: "Configure LibSQL MCP credentials",
			Fields: []*pb.WizardField{
				&pb.WizardField{
					Key:         "url",
					Label:       "LibSQL URL",
					Description: "The URL of the LibSQL endpoint",
					Type:        "url",
					Required:    true,
				},
				&pb.WizardField{
					Key:         "authToken",
					Label:       "Auth Token",
					Description: "The Authentication Token for LibSQL",
					Type:        "password",
					Required:    true,
				},
			},
		},
	}
}
