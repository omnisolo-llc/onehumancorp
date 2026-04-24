package nats

import (
	pb "github.com/onehumancorp/mono/srcs/proto"
)

// NatsIntegration implements the IntegrationProvider interface for NATS.
type NatsIntegration struct{}

// Metadata returns the metadata for the NATS integration.
func (n *NatsIntegration) Metadata() *pb.IntegrationMetadata {
	id := "nats"
	name := "NATS Event Mesh"
	typ := "event_mesh"
	category := "infrastructure"
	description := "Provides low-latency, scalable, and decentralized event routing for real-time communication between Cloud-Native and Standalone Desktop nodes."
	publisher := "OneHumanCorp"
	icon := "nats"
	builder := pb.IntegrationMetadata_builder{
		Id:          &id,
		Name:        &name,
		Type:        &typ,
		Category:    &category,
		Description: &description,
		Publisher:   &publisher,
		Icon:        &icon,
		Tags:        []string{"event", "mesh", "pubsub", "jetstream", "hybrid"},
	}
	res := builder.Build()
	return res
}

// WizardSteps returns the configuration steps for the NATS integration.
func (n *NatsIntegration) WizardSteps() []*pb.WizardStep {
	title := "Connection Configuration"
	desc := "Configure the connection to the NATS cluster. Leave blank to use an embedded local instance."

	key1 := "nats_url"
	label1 := "NATS Server URL"
	type1 := "url"
	req1 := false
	desc1 := "The URL of the remote NATS cluster (e.g., nats://cloud-node:4222)."

	key2 := "credentials_file"
	label2 := "Credentials File"
	type2 := "text"
	req2 := false
	desc2 := "Path to the NATS credentials file for authentication."

	field1 := pb.WizardField_builder{
		Key:         &key1,
		Label:       &label1,
		Type:        &type1,
		Required:    &req1,
		Description: &desc1,
	}.Build()

	field2 := pb.WizardField_builder{
		Key:         &key2,
		Label:       &label2,
		Type:        &type2,
		Required:    &req2,
		Description: &desc2,
	}.Build()

	step1 := pb.WizardStep_builder{
		Title:       &title,
		Description: &desc,
		Fields:      []*pb.WizardField{field1, field2},
	}.Build()

	return []*pb.WizardStep{step1}
}
