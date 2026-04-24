package restic

import (
	"testing"
)

func TestResticIntegration_Metadata(t *testing.T) {
	integration := &ResticIntegration{}
	metadata := integration.Metadata()

	if metadata.GetId() != "restic" {
		t.Errorf("Expected ID to be 'restic', got '%s'", metadata.GetId())
	}
	if metadata.GetName() != "Restic" {
		t.Errorf("Expected Name to be 'Restic', got '%s'", metadata.GetName())
	}
}

func TestResticIntegration_WizardSteps(t *testing.T) {
	integration := &ResticIntegration{}
	steps := integration.WizardSteps()

	if len(steps) != 1 {
		t.Errorf("Expected 1 wizard step, got %d", len(steps))
	}

	step := steps[0]
	if step.GetTitle() != "Repository Configuration" {
		t.Errorf("Expected Title to be 'Repository Configuration', got '%s'", step.GetTitle())
	}

	fields := step.GetFields()
	if len(fields) != 2 {
		t.Errorf("Expected 2 fields, got %d", len(fields))
	}

	field1 := fields[0]
	if field1.GetKey() != "repository" {
		t.Errorf("Expected Field 1 Key to be 'repository', got '%s'", field1.GetKey())
	}
	if field1.GetType() != "text" {
		t.Errorf("Expected Field 1 Type to be 'text', got '%v'", field1.GetType())
	}

	field2 := fields[1]
	if field2.GetKey() != "password" {
		t.Errorf("Expected Field 2 Key to be 'password', got '%s'", field2.GetKey())
	}
	if field2.GetType() != "password" {
		t.Errorf("Expected Field 2 Type to be 'password', got '%v'", field2.GetType())
	}
}
