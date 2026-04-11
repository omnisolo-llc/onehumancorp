package orchestration

import (
	"testing"
)

func TestHybridHealthProbe(t *testing.T) {
	probe := HybridHealthProbe{
		Status: "healthy",
	}
	if probe.Status != "healthy" {
		t.Errorf("expected healthy, got %v", probe.Status)
	}
}
