package nats

import (
	"testing"
)

func TestNatsIntegration_Metadata(t *testing.T) {
	provider := &NatsIntegration{}
	meta := provider.Metadata()

	if meta.GetId() != "nats" {
		t.Errorf("Expected Id to be 'nats', got %s", meta.GetId())
	}
	if meta.GetName() != "NATS" {
		t.Errorf("Expected Name to be 'NATS', got %s", meta.GetName())
	}
	if len(meta.GetTags()) != 4 {
		t.Errorf("Expected 4 tags, got %d", len(meta.GetTags()))
	}
}

func TestNatsIntegration_WizardSteps(t *testing.T) {
	provider := &NatsIntegration{}
	steps := provider.WizardSteps()

	if len(steps) != 1 {
		t.Fatalf("Expected 1 wizard step, got %d", len(steps))
	}

	fields := steps[0].GetFields()
	if len(fields) != 2 {
		t.Fatalf("Expected 2 fields in step, got %d", len(fields))
	}

	if fields[0].GetKey() != "url" {
		t.Errorf("Expected first field key to be 'url', got %s", fields[0].GetKey())
	}
}
