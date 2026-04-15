package growth

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/lib/analytics"
)

func TestCheckQuota(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewQuotaService(tracker)

	err := service.CheckQuota(context.Background(), "free", 50)
	if err != nil {
		t.Errorf("CheckQuota failed on passing value: %v", err)
	}

	err = service.CheckQuota(context.Background(), "free", 150)
	if err == nil {
		t.Errorf("Expected error for exceeded quota")
	}

	err = service.CheckQuota(context.Background(), "unknown", 10)
	if err == nil {
		t.Errorf("Expected error for unknown tier")
	}
}
