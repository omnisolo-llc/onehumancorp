package growth

import (
	"context"
	"github.com/onehumancorp/mono/src/server/lib/analytics"
	"testing"
)

func TestProcessInvite(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewReferralService(tracker)

	err := service.ProcessInvite(context.Background(), "user-123", "test@example.com")
	if err != nil {
		t.Errorf("ProcessInvite failed: %v", err)
	}

	err = service.ProcessInvite(context.Background(), "", "")
	if err == nil {
		t.Errorf("Expected error for empty parameters")
	}
}

func TestAcceptInvite(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewReferralService(tracker)

	err := service.AcceptInvite(context.Background(), "invite-123")
	if err != nil {
		t.Errorf("AcceptInvite failed: %v", err)
	}

	err = service.AcceptInvite(context.Background(), "")
	if err == nil {
		t.Errorf("Expected error for empty invite ID")
	}
}

func TestProcessBulkInvites(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewReferralService(tracker)

	err := service.ProcessBulkInvites(context.Background(), "user-123", []string{"test1@example.com", "test2@example.com"})
	if err != nil {
		t.Errorf("ProcessBulkInvites failed: %v", err)
	}

	err = service.ProcessBulkInvites(context.Background(), "", []string{})
	if err == nil {
		t.Errorf("Expected error for empty parameters")
	}
}
