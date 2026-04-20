package orchestration

import (
	"context"
	"testing"
)

func TestResourceMonitor_Basic(t *testing.T) {
	rm := NewResourceMonitor()
	ctx := context.Background()

	// Non-linux fallback check
	_, err := rm.GetCPUUsage(ctx)
	if err != nil {
		t.Errorf("GetCPUUsage failed: %v", err)
	}

	_, err = rm.GetMemoryUsage(ctx)
	if err != nil {
		t.Errorf("GetMemoryUsage failed: %v", err)
	}
}
