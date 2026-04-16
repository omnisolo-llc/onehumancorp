package growth

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/lib/analytics"
)

func TestProcessBulkInvites(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewTeamInviteService(tracker)

	err := service.ProcessBulkInvites(context.Background(), "user-123", "a@example.com, b@example.com, c@example.com")
	if err != nil {
		t.Errorf("ProcessBulkInvites failed: %v", err)
	}

	err = service.ProcessBulkInvites(context.Background(), "", "")
	if err == nil {
		t.Errorf("Expected error for empty parameters")
	}

	err = service.ProcessBulkInvites(context.Background(), "user-123", "   ,  ,,")
	if err == nil {
		t.Errorf("Expected error for no valid emails")
	}

	err = service.ProcessBulkInvites(context.Background(), "user-123", "invalid1, invalid2")
	if err == nil {
		t.Errorf("Expected error for no valid emails")
	}
}
