package growth

import "testing"

func TestTeamInviteManager(t *testing.T) {
    manager := NewTeamInviteManager()
    invite := manager.CreateInvite("inv-1", "team-1", "user-1", "test@example.com")
    if invite.Status != "PENDING" {
        t.Errorf("Expected PENDING status")
    }

    invites := manager.GetInvites("team-1")
    if len(invites) != 1 {
        t.Errorf("Expected 1 invite")
    }

    accepted, ok := manager.AcceptInvite("inv-1")
    if !ok || accepted.Status != "ACCEPTED" {
        t.Errorf("Failed to accept invite")
    }
}
