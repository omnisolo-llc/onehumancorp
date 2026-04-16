package powersync

import (
	pb "github.com/onehumancorp/mono/srcs/proto"
	"google.golang.org/protobuf/proto"
)

// PowerSyncIntegration implements the Integration interface for PowerSync.
type PowerSyncIntegration struct{}

// Metadata returns the integration metadata for PowerSync.
func (p *PowerSyncIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("powersync"),
		Name:        proto.String("PowerSync"),
		Type:        proto.String("powersync"),
		Category:    proto.String("database"),
		Description: proto.String("Seamless, offline-first synchronization engine bridging SQLite and PostgreSQL."),
		Publisher:   proto.String("JourneyApps"),
		Icon:        proto.String("https://www.powersync.com/favicon.ico"),
		Tags:        []string{"database", "sync", "offline-first"}}.Build()
}

// WizardSteps returns the wizard steps for configuring PowerSync.
func (p *PowerSyncIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Data"),
			Description: proto.String("Configure PowerSync API credentials"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{Key: proto.String("endpoint"), Label: proto.String("PowerSync Endpoint"), Type: proto.String("url"), Required: proto.Bool(true)}.Build(),
				pb.WizardField_builder{Key: proto.String("token"), Label: proto.String("PowerSync Token"), Type: proto.String("password"), Required: proto.Bool(true)}.Build(),
			},
		}.Build(),
	}
}
