package litefs

import (
	"testing"
)

func TestLiteFSIntegration_Metadata(t *testing.T) {
	integration := &LiteFSIntegration{}
	metadata := integration.Metadata()

	if metadata.GetId() != "litefs" {
		t.Errorf("Expected ID to be 'litefs', got '%s'", metadata.GetId())
	}
	if metadata.GetName() != "LiteFS" {
		t.Errorf("Expected Name to be 'LiteFS', got '%s'", metadata.GetName())
	}
}

func TestLiteFSIntegration_WizardSteps(t *testing.T) {
	integration := &LiteFSIntegration{}
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

	field := fields[0]
	if field.GetKey() != "url" {
		t.Errorf("Expected Field Key to be 'url', got '%s'", field.GetKey())
	}
	if field.GetType() != "url" {
		t.Errorf("Expected Field Type to be 'url', got '%v'", field.GetType())
	}
}
