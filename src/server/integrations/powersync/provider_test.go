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
	if len(fields) != 1 {
		t.Errorf("Expected 1 field, got %d", len(fields))
	}

	field1 := fields[0]
	if field1.GetKey() != "url" {
		t.Errorf("Expected Field 1 Key to be 'url', got '%s'", field1.GetKey())
	}
	if field1.GetType() != "url" {
		t.Errorf("Expected Field 1 Type to be 'url', got '%v'", field1.GetType())
	}
}
