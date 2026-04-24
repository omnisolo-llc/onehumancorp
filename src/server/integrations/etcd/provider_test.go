package etcd

import (
	"testing"
)

func TestEtcdIntegration_Metadata(t *testing.T) {
	integration := &EtcdIntegration{}
	metadata := integration.Metadata()

	if metadata.GetId() != "etcd" {
		t.Errorf("Expected ID to be 'etcd', got '%s'", metadata.GetId())
	}
	if metadata.GetName() != "etcd" {
		t.Errorf("Expected Name to be 'etcd', got '%s'", metadata.GetName())
	}
	if metadata.GetType() != "etcd" {
		t.Errorf("Expected Type to be 'etcd', got '%s'", metadata.GetType())
	}
	if metadata.GetCategory() != "database" {
		t.Errorf("Expected Category to be 'database', got '%s'", metadata.GetCategory())
	}
}

func TestEtcdIntegration_WizardSteps(t *testing.T) {
	integration := &EtcdIntegration{}
	steps := integration.WizardSteps()

	if len(steps) != 1 {
		t.Errorf("Expected 1 wizard step, got %d", len(steps))
	}

	step := steps[0]
	if step.GetTitle() != "Connection Configuration" {
		t.Errorf("Expected Title to be 'Connection Configuration', got '%s'", step.GetTitle())
	}

	fields := step.GetFields()
	if len(fields) != 3 {
		t.Errorf("Expected 3 fields, got %d", len(fields))
	}

	field1 := fields[0]
	if field1.GetKey() != "endpoints" {
		t.Errorf("Expected Field 1 Key to be 'endpoints', got '%s'", field1.GetKey())
	}
	if field1.GetType() != "text" {
		t.Errorf("Expected Field 1 Type to be 'text', got '%v'", field1.GetType())
	}

	field2 := fields[1]
	if field2.GetKey() != "username" {
		t.Errorf("Expected Field 2 Key to be 'username', got '%s'", field2.GetKey())
	}
	if field2.GetType() != "text" {
		t.Errorf("Expected Field 2 Type to be 'text', got '%v'", field2.GetType())
	}

	field3 := fields[2]
	if field3.GetKey() != "password" {
		t.Errorf("Expected Field 3 Key to be 'password', got '%s'", field3.GetKey())
	}
	if field3.GetType() != "password" {
		t.Errorf("Expected Field 3 Type to be 'password', got '%v'", field3.GetType())
	}
}
