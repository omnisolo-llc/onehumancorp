package atlas

import (
	"testing"
)

func TestAtlasIntegration_Metadata(t *testing.T) {
	integration := &AtlasIntegration{}
	metadata := integration.Metadata()

	if metadata.GetId() != "atlas" {
		t.Errorf("Expected ID to be 'atlas', got '%s'", metadata.GetId())
	}
	if metadata.GetName() != "Atlas MCP" {
		t.Errorf("Expected Name to be 'Atlas MCP', got '%s'", metadata.GetName())
	}
}

func TestAtlasIntegration_WizardSteps(t *testing.T) {
	integration := &AtlasIntegration{}
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
	if field.GetKey() != "database_url" {
		t.Errorf("Expected Field Key to be 'database_url', got '%s'", field.GetKey())
	}
	if field.GetType() != "url" {
		t.Errorf("Expected Field Type to be 'url', got '%v'", field.GetType())
	}
}
