package growth

import "testing"

func TestFreeTierQuota(t *testing.T) {
	quota := NewFreeTierQuota()
	if quota.MaxAgents != 3 {
		t.Errorf("Expected MaxAgents to be 3, got %d", quota.MaxAgents)
	}
	if quota.MaxStorageGB != 5 {
		t.Errorf("Expected MaxStorageGB to be 5, got %d", quota.MaxStorageGB)
	}

	if !quota.CanDeployAgent(2) {
		t.Error("Expected to be able to deploy agent with 2 current agents")
	}
	if quota.CanDeployAgent(3) {
		t.Error("Expected to not be able to deploy agent with 3 current agents")
	}

	if !quota.CanAllocateStorage(4) {
		t.Error("Expected to be able to allocate storage with 4GB current storage")
	}
	if quota.CanAllocateStorage(5) {
		t.Error("Expected to not be able to allocate storage with 5GB current storage")
	}
}
