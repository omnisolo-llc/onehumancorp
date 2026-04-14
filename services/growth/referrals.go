package growth

import "sync"

type SovereignInvite struct {
    ID           string
    InviterID    string
    InviteeEmail string
    AssetID      string
    Status       string
}

type ReferralTracker struct {
    mu             sync.RWMutex
    TotalReferrals int
    invites        []SovereignInvite
}

func NewReferralTracker() *ReferralTracker {
    return &ReferralTracker{
        invites: make([]SovereignInvite, 0),
    }
}

func (rt *ReferralTracker) AddReferral() {
    rt.mu.Lock()
    defer rt.mu.Unlock()
    rt.TotalReferrals++
}

func (rt *ReferralTracker) GetTotalReferrals() int {
    rt.mu.RLock()
    defer rt.mu.RUnlock()
    return rt.TotalReferrals
}

func (rt *ReferralTracker) CreateSovereignInvite(id, inviterID, inviteeEmail, assetID string) SovereignInvite {
    rt.mu.Lock()
    defer rt.mu.Unlock()
    invite := SovereignInvite{
        ID:           id,
        InviterID:    inviterID,
        InviteeEmail: inviteeEmail,
        AssetID:      assetID,
        Status:       "PENDING",
    }
    rt.invites = append(rt.invites, invite)
    return invite
}

func (rt *ReferralTracker) AcceptSovereignInvite(id string) (SovereignInvite, bool) {
    rt.mu.Lock()
    defer rt.mu.Unlock()
    for i, inv := range rt.invites {
        if inv.ID == id {
            rt.invites[i].Status = "ACCEPTED"
            return rt.invites[i], true
        }
    }
    return SovereignInvite{}, false
}

func (rt *ReferralTracker) GetSovereignInvites() []SovereignInvite {
    rt.mu.RLock()
    defer rt.mu.RUnlock()
    res := make([]SovereignInvite, len(rt.invites))
    copy(res, rt.invites)
    return res
}
