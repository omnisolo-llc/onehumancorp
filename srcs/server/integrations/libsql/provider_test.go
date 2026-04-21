package libsql

import (
	"testing"
)

func TestLibSQLIntegration_Metadata(t *testing.T) {
	integration := &LibSQLIntegration{}
	metadata := integration.Metadata()

	if metadata.GetId() != "libsql" {
		t.Errorf("Expected ID to be 'libsql', got '%s'", metadata.GetId())
	}
	if metadata.GetName() != "LibSQL" {
		t.Errorf("Expected Name to be 'LibSQL', got '%s'", metadata.GetName())
	}
}

func TestLibSQLIntegration_WizardSteps(t *testing.T) {
	integration := &LibSQLIntegration{}
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

	field1 := fields[0]
	if field1.GetKey() != "url" {
		t.Errorf("Expected Field 1 Key to be 'url', got '%s'", field1.GetKey())
	}
	if field1.GetType() != "url" {
		t.Errorf("Expected Field 1 Type to be 'url', got '%v'", field1.GetType())
	}

	field2 := fields[1]
	if field2.GetKey() != "authToken" {
		t.Errorf("Expected Field 2 Key to be 'authToken', got '%s'", field2.GetKey())
	}
	if field2.GetType() != "password" {
		t.Errorf("Expected Field 2 Type to be 'password', got '%v'", field2.GetType())
	}
}
