package atlas

import (
	"context"
	"os/exec"

	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/protobuf/proto"
)

type AtlasIntegration struct{}

func (a *AtlasIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("atlas"),
		Name:        proto.String("Atlas MCP"),
		Type:        proto.String("atlas"),
		Category:    proto.String("Database"),
		Description: proto.String("Atlas MCP integration for Declarative Database Migrations."),
		Tags:        []string{"database", "migrations", "declarative"}}.Build()
}

func (a *AtlasIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Data"),
			Description: proto.String("Configure Atlas MCP credentials"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("database_url"),
					Label:       proto.String("Database URL"),
					Description: proto.String("The URL of the database to migrate"),
					Type:        proto.String("url"),
					Required:    proto.Bool(true),
				}.Build(),
			},
		}.Build(),
	}
}

// AtlasStatus executes atlas migrate status.
func (a *AtlasIntegration) AtlasStatus(ctx context.Context, url string) (string, error) {
	cmd := exec.CommandContext(ctx, "atlas", "migrate", "status", "--url", url)
	out, err := cmd.CombinedOutput()
	return string(out), err
}

// AtlasApply executes atlas migrate apply.
func (a *AtlasIntegration) AtlasApply(ctx context.Context, url string) (string, error) {
	cmd := exec.CommandContext(ctx, "atlas", "migrate", "apply", "--url", url)
	out, err := cmd.CombinedOutput()
	return string(out), err
}

// AtlasInspect executes atlas schema inspect.
func (a *AtlasIntegration) AtlasInspect(ctx context.Context, url string) (string, error) {
	cmd := exec.CommandContext(ctx, "atlas", "schema", "inspect", "--url", url)
	out, err := cmd.CombinedOutput()
	return string(out), err
}
