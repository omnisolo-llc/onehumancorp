package kairos

import (
	"os"
	"testing"
)

func TestGetMode(t *testing.T) {
	os.Setenv("OHC_HEADLESS", "true")
	if GetMode() != "headless" {
		t.Errorf("Expected headless, got %s", GetMode())
	}

	os.Setenv("OHC_HEADLESS", "false")
	os.Setenv("OHC_MULTITENANT", "true")
	if GetMode() != "cloud" {
		t.Errorf("Expected cloud, got %s", GetMode())
	}

	os.Setenv("OHC_MULTITENANT", "false")
	if GetMode() != "standalone" {
		t.Errorf("Expected standalone, got %s", GetMode())
	}
}

func TestMetricsRegistered(t *testing.T) {
	// Simple test to ensure nothing panics during init
	AutoDreamWorkerTasksTotal.WithLabelValues("standalone", "success").Inc()
	AutoDreamStorageOpsTotal.WithLabelValues("standalone", "sqlite", "success").Inc()
	AutoDreamEmbeddingDuration.WithLabelValues("standalone").Observe(0.5)
}
