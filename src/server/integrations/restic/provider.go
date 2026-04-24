package restic

import (
	pb "github.com/onehumancorp/mono/src/proto"
	"google.golang.org/protobuf/proto"
)

type ResticIntegration struct{}

func (s *ResticIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("restic"),
		Name:        proto.String("Restic"),
		Type:        proto.String("restic"),
		Category:    proto.String("Database"),
		Description: proto.String("Restic MCP for secure, deduplicated, local-first snapshots."),
		Publisher:   proto.String("Restic"),
		Icon:        proto.String("https://restic.net/restic.png"),
		Tags:        []string{"backup", "snapshot", "local", "encrypted"}}.Build()
}

func (s *ResticIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Repository Configuration"),
			Description: proto.String("Configure the Restic backup repository"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("repository"),
					Label:       proto.String("Repository Path/URL"),
					Description: proto.String("The path to the local directory or S3 bucket URL"),
					Type:        proto.String("text"),
					Required:    proto.Bool(true),
				}.Build(),
				pb.WizardField_builder{
					Key:         proto.String("password"),
					Label:       proto.String("Repository Password"),
					Description: proto.String("The password to encrypt/decrypt the Restic repository"),
					Type:        proto.String("password"),
					Required:    proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}
