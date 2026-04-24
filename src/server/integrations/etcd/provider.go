package etcd

import (
	pb "github.com/onehumancorp/mono/src/proto"
	"google.golang.org/protobuf/proto"
)

type EtcdIntegration struct{}

func (s *EtcdIntegration) Metadata() *pb.IntegrationMetadata {
	return pb.IntegrationMetadata_builder{
		Id:          proto.String("etcd"),
		Name:        proto.String("etcd"),
		Type:        proto.String("etcd"),
		Category:    proto.String("database"),
		Description: proto.String("etcd MCP for Distributed Swarm State orchestration and configuration sync."),
		Publisher:   proto.String("CNCF"),
		Icon:        proto.String("https://etcd.io/img/etcd-horizontal-color.svg"),
		Tags:        []string{"etcd", "distributed", "kv", "swarm", "state"}}.Build()
}

func (s *EtcdIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		pb.WizardStep_builder{
			Title:       proto.String("Connection Configuration"),
			Description: proto.String("Configure etcd MCP credentials"),
			Fields: []*pb.WizardField{
				pb.WizardField_builder{
					Key:         proto.String("endpoints"),
					Label:       proto.String("Endpoints"),
					Description: proto.String("Comma-separated list of etcd cluster endpoints (e.g. localhost:2379)"),
					Type:        proto.String("text"),
					Required:    proto.Bool(true),
				}.Build(),
				pb.WizardField_builder{
					Key:         proto.String("username"),
					Label:       proto.String("Username"),
					Description: proto.String("The Username for etcd authentication (optional)"),
					Type:        proto.String("text"),
					Required:    proto.Bool(false),
				}.Build(),
				pb.WizardField_builder{
					Key:         proto.String("password"),
					Label:       proto.String("Password"),
					Description: proto.String("The Password for etcd authentication (optional)"),
					Type:        proto.String("password"),
					Required:    proto.Bool(false),
				}.Build(),
			},
		}.Build(),
	}
}
