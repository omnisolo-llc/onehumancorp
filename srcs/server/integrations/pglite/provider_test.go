package pglite

import (
	"testing"
)

func TestPGLiteIntegration_Metadata(t *testing.T) {
	integration := &PGLiteIntegration{}
	metadata := integration.Metadata()
	if metadata == nil {
		t.Errorf("Expected metadata, got nil")
	}

	if metadata.GetId() != "pglite" {
		t.Errorf("Expected id to be pglite, got %s", metadata.GetId())
	}
}

func TestPGLiteIntegration_WizardSteps(t *testing.T) {
	integration := &PGLiteIntegration{}
	steps := integration.WizardSteps()
	if steps == nil {
		t.Errorf("Expected wizard steps, got nil")
	}

	if len(steps) != 1 {
		t.Errorf("Expected 1 wizard step, got %d", len(steps))
	}
}

func TestPGLiteIntegration_Tools(t *testing.T) {
	integration := &PGLiteIntegration{}

	queryRes := integration.PGLiteQuery("SELECT 1")
	if queryRes == "" {
		t.Errorf("Expected query result, got empty")
	}

	syncRes := integration.PGLiteSyncStatus()
	if syncRes == "" {
		t.Errorf("Expected sync result, got empty")
	}
}
