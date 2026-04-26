package etcd

import (
	pb "github.com/onehumancorp/mono/src/proto"
)

type EtcdIntegration struct{}

func (s *EtcdIntegration) Metadata() *pb.IntegrationMetadata {
	return &pb.IntegrationMetadata{
		Id:          "etcd",
		Name:        "etcd",
		Type:        "etcd",
		Category:    "database",
		Description: "etcd MCP for Distributed Swarm State orchestration and configuration sync.",
		Publisher:   "CNCF",
		Icon:        "https://etcd.io/img/etcd-horizontal-color.svg",
		Tags:        []string{"etcd", "distributed", "kv", "swarm", "state"}}
}

func (s *EtcdIntegration) WizardSteps() []*pb.WizardStep {
	return []*pb.WizardStep{
		&pb.WizardStep{
			Title:       "Connection Configuration",
			Description: "Configure etcd MCP credentials",
			Fields: []*pb.WizardField{
				&pb.WizardField{
					Key:         "endpoints",
					Label:       "Endpoints",
					Description: "Comma-separated list of etcd cluster endpoints (e.g. localhost:2379)",
					Type:        "text",
					Required:    true,
				},
				&pb.WizardField{
					Key:         "username",
					Label:       "Username",
					Description: "The Username for etcd authentication (optional)",
					Type:        "text",
					Required:    false,
				},
				&pb.WizardField{
					Key:         "password",
					Label:       "Password",
					Description: "The Password for etcd authentication (optional)",
					Type:        "password",
					Required:    false,
				},
			},
		},
	}
}
