package powersync

import (
	"testing"
)

func TestPowerSyncIntegration_Metadata(t *testing.T) {
	p := &PowerSyncIntegration{}
	meta := p.Metadata()

	if meta.GetId() != "powersync" {
		t.Errorf("Expected id to be 'powersync', got '%s'", meta.GetId())
	}
	if meta.GetName() != "PowerSync" {
		t.Errorf("Expected name to be 'PowerSync', got '%s'", meta.GetName())
	}
	if meta.GetType() != "powersync" {
		t.Errorf("Expected type to be 'powersync', got '%s'", meta.GetType())
	}
	if meta.GetCategory() != "database" {
		t.Errorf("Expected category to be 'database', got '%s'", meta.GetCategory())
	}
}

func TestPowerSyncIntegration_WizardSteps(t *testing.T) {
	p := &PowerSyncIntegration{}
	steps := p.WizardSteps()

	if len(steps) != 1 {
		t.Fatalf("Expected 1 wizard step, got %d", len(steps))
	}
	if steps[0].GetTitle() != "Connection Data" {
		t.Errorf("Expected title to be 'Connection Data', got '%s'", steps[0].GetTitle())
	}
	if len(steps[0].GetFields()) != 2 {
		t.Fatalf("Expected 2 fields, got %d", len(steps[0].GetFields()))
	}
}
