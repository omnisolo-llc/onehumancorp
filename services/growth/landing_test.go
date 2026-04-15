package growth

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/lib/analytics"
)

func TestLandingService_TrackVisit(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewLandingService(tracker)
	ctx := context.Background()

	err := service.TrackVisit(ctx, "page123", "visitor456")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	err = service.TrackVisit(ctx, "", "visitor456")
	if err == nil {
		t.Fatalf("expected error for empty pageID")
	}

	err = service.TrackVisit(ctx, "page123", "")
	if err == nil {
		t.Fatalf("expected error for empty visitorID")
	}
}

func TestLandingService_TrackConversion(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewLandingService(tracker)
	ctx := context.Background()

	err := service.TrackConversion(ctx, "page123", "visitor456")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	err = service.TrackConversion(ctx, "", "visitor456")
	if err == nil {
		t.Fatalf("expected error for empty pageID")
	}

	err = service.TrackConversion(ctx, "page123", "")
	if err == nil {
		t.Fatalf("expected error for empty visitorID")
	}
}
