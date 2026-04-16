package powersync

import (
	"testing"
)

func TestPowerSyncIntegration_Metadata(t *testing.T) {
	integration := &PowerSyncIntegration{}
	metadata := integration.Metadata()

	if metadata.GetId() != "powersync" {
		t.Errorf("Expected ID to be 'powersync', got '%s'", metadata.GetId())
	}
	if metadata.GetName() != "PowerSync" {
		t.Errorf("Expected Name to be 'PowerSync', got '%s'", metadata.GetName())
	}
}

func TestPowerSyncIntegration_WizardSteps(t *testing.T) {
	integration := &PowerSyncIntegration{}
	steps := integration.WizardSteps()

	if len(steps) != 1 {
		t.Errorf("Expected 1 wizard step, got %d", len(steps))
	}

	step := steps[0]
	if step.GetTitle() != "Connection Data" {
		t.Errorf("Expected Title to be 'Connection Data', got '%s'", step.GetTitle())
	}

	fields := step.GetFields()
	if len(fields) != 2 {
		t.Errorf("Expected 2 fields, got %d", len(fields))
	}

	field := fields[0]
	if field.GetKey() != "url" {
		t.Errorf("Expected first Field Key to be 'url', got '%s'", field.GetKey())
	}
	if field.GetType() != "url" {
		t.Errorf("Expected first Field Type to be 'url', got '%v'", field.GetType())
	}

	field2 := fields[1]
	if field2.GetKey() != "token" {
		t.Errorf("Expected second Field Key to be 'token', got '%s'", field2.GetKey())
	}
	if field2.GetType() != "password" {
		t.Errorf("Expected second Field Type to be 'password', got '%v'", field2.GetType())
	}
}
