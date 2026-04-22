package obsidian

import (
	"testing"
)

func TestObsidianIntegration_Metadata(t *testing.T) {
	integration := &ObsidianIntegration{}
	meta := integration.Metadata()

	if meta.GetId() != "obsidian" {
		t.Errorf("expected id obsidian, got %s", meta.GetId())
	}
	if meta.GetName() != "Obsidian" {
		t.Errorf("expected name Obsidian, got %s", meta.GetName())
	}
}

func TestObsidianIntegration_WizardSteps(t *testing.T) {
	integration := &ObsidianIntegration{}
	steps := integration.WizardSteps()

	if len(steps) != 1 {
		t.Fatalf("expected 1 wizard step, got %d", len(steps))
	}
	if steps[0].GetTitle() != "Obsidian Configuration" {
		t.Errorf("expected title Obsidian Configuration, got %s", steps[0].GetTitle())
	}
}
