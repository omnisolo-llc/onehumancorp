package nats

import (
	"testing"
)

func TestNatsIntegration_Metadata(t *testing.T) {
	integration := &NatsIntegration{}
	meta := integration.Metadata()

	if meta.GetId() != "nats" {
		t.Errorf("expected Id 'nats', got '%s'", meta.GetId())
	}
	if meta.GetType() != "event_mesh" {
		t.Errorf("expected Type 'event_mesh', got '%s'", meta.GetType())
	}
	if len(meta.GetTags()) == 0 {
		t.Errorf("expected Tags to be non-empty")
	}
}

func TestNatsIntegration_WizardSteps(t *testing.T) {
	integration := &NatsIntegration{}
	steps := integration.WizardSteps()

	if len(steps) != 1 {
		t.Fatalf("expected 1 wizard step, got %d", len(steps))
	}

	step := steps[0]
	if step.GetTitle() != "Connection Configuration" {
		t.Errorf("expected Title 'Connection Configuration', got '%s'", step.GetTitle())
	}

	if len(step.GetFields()) != 2 {
		t.Fatalf("expected 2 fields, got %d", len(step.GetFields()))
	}

	if step.GetFields()[0].GetKey() != "nats_url" {
		t.Errorf("expected field 1 Key 'nats_url', got '%s'", step.GetFields()[0].GetKey())
	}
	if step.GetFields()[1].GetKey() != "credentials_file" {
		t.Errorf("expected field 2 Key 'credentials_file', got '%s'", step.GetFields()[1].GetKey())
	}
}
