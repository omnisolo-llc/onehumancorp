package growth

import "testing"

func TestReferralTracker(t *testing.T) {
    tracker := NewReferralTracker()
    if tracker.GetTotalReferrals() != 0 {
        t.Errorf("Expected 0 referrals initially, got %d", tracker.GetTotalReferrals())
    }
    tracker.AddReferral()
    if tracker.GetTotalReferrals() != 1 {
        t.Errorf("Expected 1 referral after add, got %d", tracker.GetTotalReferrals())
    }

    // Test SovereignInvite
    invite := tracker.CreateSovereignInvite("inv-1", "user-1", "guest@example.com", "asset-1")
    if invite.Status != "PENDING" {
        t.Errorf("Expected PENDING status, got %s", invite.Status)
    }

    invites := tracker.GetSovereignInvites()
    if len(invites) != 1 {
        t.Errorf("Expected 1 invite, got %d", len(invites))
    }

    acceptedInvite, ok := tracker.AcceptSovereignInvite("inv-1")
    if !ok {
        t.Errorf("Failed to accept invite")
    }
    if acceptedInvite.Status != "ACCEPTED" {
        t.Errorf("Expected ACCEPTED status, got %s", acceptedInvite.Status)
    }
}
