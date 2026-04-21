package obsidian

import (
	"testing"
)

func TestObsidianIntegrationMetadata(t *testing.T) {
	integration := &ObsidianIntegration{}
	metadata := integration.Metadata()
	if metadata.GetId() != "obsidian" {
		t.Errorf("Expected id to be obsidian, got %s", metadata.GetId())
	}
	if metadata.GetName() != "Obsidian" {
		t.Errorf("Expected name to be Obsidian, got %s", metadata.GetName())
	}
}

func TestObsidianIntegrationWizardSteps(t *testing.T) {
	integration := &ObsidianIntegration{}
	steps := integration.WizardSteps()
	if len(steps) == 0 {
		t.Errorf("Expected wizard steps to not be empty")
	}
}
