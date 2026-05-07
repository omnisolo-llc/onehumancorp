package libsql

import (
	"context"
	"testing"
)

func TestLibSQLIntegration_Metadata(t *testing.T) {
	l := NewLibSQLIntegration(nil)
	meta := l.Metadata()
	if meta.ID != "libsql" {
		t.Errorf("Expected ID libsql, got %s", meta.ID)
	}
}

func TestLibSQLIntegration_WizardSteps(t *testing.T) {
	l := NewLibSQLIntegration(nil)
	steps := l.WizardSteps()
	if len(steps) != 2 {
		t.Errorf("Expected 2 steps, got %d", len(steps))
	}
}

func TestLibSQLIntegration_CheckReplicationLag(t *testing.T) {
	l := NewLibSQLIntegration(nil)
	ctx := context.Background()
	_, err := l.CheckReplicationLag(ctx)
	if err == nil {
		t.Errorf("Expected error for unconfigured URL")
	}

	l.ConfigureReplication("http://example.com", "token")
	lag, err := l.CheckReplicationLag(ctx)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
	if lag != 100 {
		t.Errorf("Expected lag 100, got %d", lag)
	}
}

func TestLibSQLIntegration_ValidateEdgeSync(t *testing.T) {
	l := NewLibSQLIntegration(nil)
	ctx := context.Background()
	err := l.ValidateEdgeSync(ctx)
	if err == nil {
		t.Errorf("Expected error for unconfigured URL")
	}

	l.ConfigureReplication("http://example.com", "token")
	err = l.ValidateEdgeSync(ctx)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}
