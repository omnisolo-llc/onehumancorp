package growth

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/lib/analytics"
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

	emails := make([]string, 101)
	for i := 0; i < 101; i++ {
		emails[i] = "test@example.com"
	}
	err = service.ProcessBulkInvites(context.Background(), "user-123", emails)
	if err == nil {
		t.Errorf("Expected error for too many emails")
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
