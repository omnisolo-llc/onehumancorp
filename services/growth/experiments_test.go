package growth

import (
	"testing"
)

func TestExperimentTracker(t *testing.T) {
	tracker := NewExperimentTracker()

	tracker.TrackAssignment("user-1", "local_first")
	tracker.TrackAssignment("user-2", "local_first")
	tracker.TrackAssignment("user-3", "cloud_convenience")

	tracker.MarkConverted("user-1")

	metrics := tracker.GetMetrics()

	if len(metrics) != 2 {
		t.Fatalf("expected 2 variants, got %d", len(metrics))
	}

	localMetrics := metrics["local_first"]
	if localMetrics.TotalAssigned != 2 {
		t.Errorf("expected 2 assignments for local_first, got %d", localMetrics.TotalAssigned)
	}
	if localMetrics.TotalConverted != 1 {
		t.Errorf("expected 1 conversion for local_first, got %d", localMetrics.TotalConverted)
	}
	if localMetrics.ConversionRate != 50.0 {
		t.Errorf("expected 50%% conversion rate for local_first, got %f", localMetrics.ConversionRate)
	}

	cloudMetrics := metrics["cloud_convenience"]
	if cloudMetrics.TotalAssigned != 1 {
		t.Errorf("expected 1 assignment for cloud_convenience, got %d", cloudMetrics.TotalAssigned)
	}
	if cloudMetrics.TotalConverted != 0 {
		t.Errorf("expected 0 conversions for cloud_convenience, got %d", cloudMetrics.TotalConverted)
	}
	if cloudMetrics.ConversionRate != 0.0 {
		t.Errorf("expected 0%% conversion rate for cloud_convenience, got %f", cloudMetrics.ConversionRate)
	}
}
