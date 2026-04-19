package restic

import (
	pb "github.com/onehumancorp/mono/srcs/proto"
	"github.com/onehumancorp/mono/srcs/server/tools/resticmcp"
	"google.golang.org/protobuf/proto"
)

type ResticIntegration struct{}

func (s *ResticIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("restic"),
		Name:        proto.String("Restic"),
		Type:        proto.String("restic"),
		Category:    proto.String("Database"),
		Description: proto.String("Restic MCP for secure, deduplicated, local-first snapshots of OHC configuration, SQLite, and ChromaDB."),
		Publisher:   proto.String("Restic"),
		Icon:        proto.String("https://restic.net/restic-logo-500.png"),
		Tags:        []string{"backup", "snapshot", "local-first", "mcp"}}.Build()
}

func (s *ResticIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Repository Configuration"),
			Description: proto.String("Configure Restic repository credentials"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("repository"),
					Label:       proto.String("Repository Path/URL"),
					Description: proto.String("Local path or remote URL to the Restic repository"),
					Type:        proto.String("text"),
					Required:    proto.Bool(true),
				}.Build(),
				pb.WizardField_builder{
					Key:         proto.String("password"),
					Label:       proto.String("Repository Password"),
					Description: proto.String("Password to decrypt the Restic repository"),
					Type:        proto.String("password"),
					Required:    proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}

// GetProvider returns the underlying ResticMCP implementation
func (s *ResticIntegration) GetProvider(repo, pwd string) *resticmcp.ResticMCP {
	return resticmcp.NewResticMCP(repo, pwd)
}
