package nats

import (
	"testing"
)

func TestNatsIntegration_Metadata(t *testing.T) {
	integration := &NatsIntegration{}
	metadata := integration.Metadata()

	if metadata.GetId() != "nats" {
		t.Errorf("Expected ID to be 'nats', got '%s'", metadata.GetId())
	}
	if metadata.GetName() != "NATS" {
		t.Errorf("Expected Name to be 'NATS', got '%s'", metadata.GetName())
	}
}

func TestNatsIntegration_WizardSteps(t *testing.T) {
	integration := &NatsIntegration{}
	steps := integration.WizardSteps()

	if len(steps) != 1 {
		t.Errorf("Expected 1 wizard step, got %d", len(steps))
	}

	step := steps[0]
	if step.GetTitle() != "Connection Settings" {
		t.Errorf("Expected Title to be 'Connection Settings', got '%s'", step.GetTitle())
	}

	fields := step.GetFields()
	if len(fields) != 2 {
		t.Errorf("Expected 2 fields, got %d", len(fields))
	}

	field1 := fields[0]
	if field1.GetKey() != "url" {
		t.Errorf("Expected Field 1 Key to be 'url', got '%s'", field1.GetKey())
	}
	if field1.GetType() != "url" {
		t.Errorf("Expected Field 1 Type to be 'url', got '%v'", field1.GetType())
	}

	field2 := fields[1]
	if field2.GetKey() != "credentials" {
		t.Errorf("Expected Field 2 Key to be 'credentials', got '%s'", field2.GetKey())
	}
	if field2.GetType() != "password" {
		t.Errorf("Expected Field 2 Type to be 'password', got '%v'", field2.GetType())
	}
}
