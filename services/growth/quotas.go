package growth

type FreeTierQuota struct {
	MaxAgents    int
	MaxStorageGB int
}

func NewFreeTierQuota() *FreeTierQuota {
	return &FreeTierQuota{
		MaxAgents:    3,
		MaxStorageGB: 5,
	}
}

func (q *FreeTierQuota) CanDeployAgent(currentAgents int) bool {
	return currentAgents < q.MaxAgents
}

func (q *FreeTierQuota) CanAllocateStorage(currentStorageGB int) bool {
	return currentStorageGB < q.MaxStorageGB
}
