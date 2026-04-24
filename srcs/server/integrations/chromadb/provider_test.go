package chromadb

import (
	"testing"
)

func TestChromaDBIntegration_Metadata(t *testing.T) {
	integration := &ChromaDBIntegration{}
	metadata := integration.Metadata()

	if metadata.GetId() != "chromadb" {
		t.Errorf("Expected ID to be 'chromadb', got '%s'", metadata.GetId())
	}
	if metadata.GetName() != "ChromaDB" {
		t.Errorf("Expected Name to be 'ChromaDB', got '%s'", metadata.GetName())
	}
	if metadata.GetType() != "chromadb" {
		t.Errorf("Expected Type to be 'chromadb', got '%s'", metadata.GetType())
	}
	if metadata.GetCategory() != "Database" {
		t.Errorf("Expected Category to be 'Database', got '%s'", metadata.GetCategory())
	}
}

func TestChromaDBIntegration_WizardSteps(t *testing.T) {
	integration := &ChromaDBIntegration{}
	steps := integration.WizardSteps()

	if len(steps) != 1 {
		t.Fatalf("Expected 1 wizard step, got %d", len(steps))
	}

	step := steps[0]
	if step.GetTitle() != "Connection Data" {
		t.Errorf("Expected title 'Connection Data', got '%s'", step.GetTitle())
	}

	if len(step.GetFields()) != 1 {
		t.Fatalf("Expected 1 field, got %d", len(step.GetFields()))
	}

	field := step.GetFields()[0]
	if field.GetKey() != "url" {
		t.Errorf("Expected field key 'url', got '%s'", field.GetKey())
	}
	if field.GetType() != "url" {
		t.Errorf("Expected field type 'url', got '%s'", field.GetType())
	}
	if !field.GetRequired() {
		t.Errorf("Expected field 'url' to be required")
	}
}
