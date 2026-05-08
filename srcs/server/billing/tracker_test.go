package billing

import (
	"context"
	"testing"
)

func TestRecordMissionCost(t *testing.T) {
	tracker := NewTracker()
	ctx := context.Background()

	err := tracker.RecordMissionCost(ctx, "tenant-1", "mission-1", "agent-1", "role-1", 50.0)
	if err != nil {
		t.Errorf("Expected no error, got %v", err)
	}
}
