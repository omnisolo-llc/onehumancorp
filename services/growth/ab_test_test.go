package growth

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/lib/analytics"
)

func TestRecordImpression(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewABTestService(tracker)

	err := service.RecordImpression(context.Background(), "exp-123", "variant-a")
	if err != nil {
		t.Errorf("RecordImpression failed: %v", err)
	}

	err = service.RecordImpression(context.Background(), "", "")
	if err == nil {
		t.Errorf("Expected error for empty parameters")
	}
}

func TestRecordConversion(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewABTestService(tracker)

	err := service.RecordConversion(context.Background(), "exp-123", "variant-a")
	if err != nil {
		t.Errorf("RecordConversion failed: %v", err)
	}

	err = service.RecordConversion(context.Background(), "", "")
	if err == nil {
		t.Errorf("Expected error for empty parameters")
	}
}
