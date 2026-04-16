package pglite

import (
	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/protobuf/proto"
)

type PGLiteIntegration struct{}

func (s *PGLiteIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("pglite"),
		Name:        proto.String("PGLite"),
		Type:        proto.String("pglite"),
		Category:    proto.String("Database"),
		Description: proto.String("PGLite MCP for Distributed Local-First Postgres Synchronization."),
		Publisher:   proto.String("ElectricSQL"),
		Icon:        proto.String("https://electric-sql.com/img/pglite-logo.svg"),
		Tags:        []string{"postgres", "distributed", "local-first"}}.Build()
}

func (s *PGLiteIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Data"),
			Description: proto.String("Configure PGLite MCP credentials"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("data_dir"),
					Label:       proto.String("Data Directory"),
					Description: proto.String("Path to PGLite persistent data"),
					Type:        proto.String("text"),
					Required:    proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}

// PGLiteQuery simulates the MCP tool for executing queries against local PGLite
func (s *PGLiteIntegration) PGLiteQuery(query string) string {
	return "unsupported in standard integration layer, intended for standalone daemon"
}

// PGLiteSyncStatus simulates the MCP tool for checking sync status
func (s *PGLiteIntegration) PGLiteSyncStatus() string {
	return "unsupported in standard integration layer, intended for standalone daemon"
}
