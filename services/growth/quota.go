package growth

import "sync"

type Quota struct {
    TeamID     string
    MaxMembers int
}

type QuotaManager struct {
    mu     sync.RWMutex
    quotas map[string]Quota
}

func NewQuotaManager() *QuotaManager {
    return &QuotaManager{
        quotas: make(map[string]Quota),
    }
}

func (qm *QuotaManager) InitializeQuota(teamID string, initialMax int) {
    qm.mu.Lock()
    defer qm.mu.Unlock()
    qm.quotas[teamID] = Quota{
        TeamID:     teamID,
        MaxMembers: initialMax,
    }
}

func (qm *QuotaManager) ExpandQuota(teamID string, additional int) {
    qm.mu.Lock()
    defer qm.mu.Unlock()
    if q, ok := qm.quotas[teamID]; ok {
        q.MaxMembers += additional
        qm.quotas[teamID] = q
    }
}

func (qm *QuotaManager) GetQuota(teamID string) (Quota, bool) {
    qm.mu.RLock()
    defer qm.mu.RUnlock()
    q, ok := qm.quotas[teamID]
    return q, ok
}
