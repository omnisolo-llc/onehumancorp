package restic

import (
	pb "github.com/onehumancorp/mono/src/proto"
)

type ResticIntegration struct{}

func (s *ResticIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "restic",
		Name:        "Restic",
		Type:        "restic",
		Category:    "Database",
		Description: "Restic MCP for secure, deduplicated, local-first snapshots.",
		Publisher:   "Restic",
		Icon:        "https://restic.net/restic.png",
		Tags:        []string{"backup", "snapshot", "local", "encrypted"}}
}

func (s *ResticIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title:       "Repository Configuration",
			Description: "Configure the Restic backup repository",
			Fields: []*pb.WizardField{
				&pb.WizardField{
					Key:         "repository",
					Label:       "Repository Path/URL",
					Description: "The path to the local directory or S3 bucket URL",
					Type:        "text",
					Required:    true,
				},
				&pb.WizardField{
					Key:         "password",
					Label:       "Repository Password",
					Description: "The password to encrypt/decrypt the Restic repository",
					Type:        "password",
					Required:    true,
				},
			},
		},
	}
}
