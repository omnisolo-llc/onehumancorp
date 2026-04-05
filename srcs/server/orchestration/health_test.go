package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestHybridHealthProbe(t *testing.T) {
	// A simple test ensuring the struct exists and fields map correctly.
	probe := HybridHealthProbe{
		Mode:        "cloud",
		Status:      "healthy",
		DBPing:      10 * time.Millisecond,
		SyncBacklog: 5,
		MeshActive:  true,
	}

	if probe.Mode != "cloud" {
		t.Errorf("Expected mode 'cloud', got '%s'", probe.Mode)
	}
	if probe.Status != "healthy" {
		t.Errorf("Expected status 'healthy', got '%s'", probe.Status)
	}
}

func TestHubCheckHealth_Degraded_NoDB(t *testing.T) {
	h := &Hub{}
	probe, err := h.CheckHealth(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if probe.Status != "degraded" {
		t.Errorf("Expected status 'degraded', got '%s'", probe.Status)
	}
	if probe.Mode != "standalone" {
		t.Errorf("Expected mode 'standalone', got '%s'", probe.Mode)
	}
	if probe.MeshActive != false {
		t.Errorf("Expected mesh active false, got true")
	}
}

// We could add more advanced mock tests here for checking full coverage of health.go
