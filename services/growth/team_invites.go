package growth

import "sync"

type TeamInvite struct {
    ID           string
    TeamID       string
    InviterID    string
    InviteeEmail string
    Status       string
}

type TeamInviteManager struct {
    mu      sync.RWMutex
    invites []TeamInvite
}

func NewTeamInviteManager() *TeamInviteManager {
    return &TeamInviteManager{
        invites: make([]TeamInvite, 0),
    }
}

func (tm *TeamInviteManager) CreateInvite(id, teamID, inviterID, inviteeEmail string) TeamInvite {
    tm.mu.Lock()
    defer tm.mu.Unlock()
    invite := TeamInvite{
        ID:           id,
        TeamID:       teamID,
        InviterID:    inviterID,
        InviteeEmail: inviteeEmail,
        Status:       "PENDING",
    }
    tm.invites = append(tm.invites, invite)
    return invite
}

func (tm *TeamInviteManager) AcceptInvite(id string) (TeamInvite, bool) {
    tm.mu.Lock()
    defer tm.mu.Unlock()
    for i, inv := range tm.invites {
        if inv.ID == id {
            tm.invites[i].Status = "ACCEPTED"
            return tm.invites[i], true
        }
    }
    return TeamInvite{}, false
}

func (tm *TeamInviteManager) GetInvites(teamID string) []TeamInvite {
    tm.mu.RLock()
    defer tm.mu.RUnlock()
    var res []TeamInvite
    for _, inv := range tm.invites {
        if inv.TeamID == teamID {
            res = append(res, inv)
        }
    }
    return res
}
