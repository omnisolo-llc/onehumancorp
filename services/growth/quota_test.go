package growth

import "testing"

func TestQuotaManager(t *testing.T) {
    manager := NewQuotaManager()
    manager.InitializeQuota("team-1", 5)

    q, ok := manager.GetQuota("team-1")
    if !ok || q.MaxMembers != 5 {
        t.Errorf("Expected initial quota 5")
    }

    manager.ExpandQuota("team-1", 3)
    q, _ = manager.GetQuota("team-1")
    if q.MaxMembers != 8 {
        t.Errorf("Expected expanded quota 8")
    }
}
