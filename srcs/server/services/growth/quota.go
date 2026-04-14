package growth

type QuotaTracker struct {
	BaseQuota int
	BonusPerReferral int
}

func NewQuotaTracker(base int, bonus int) *QuotaTracker {
	return &QuotaTracker{
		BaseQuota: base,
		BonusPerReferral: bonus,
	}
}

func (q *QuotaTracker) CalculateQuota(successfulReferrals int) int {
	return q.BaseQuota + (successfulReferrals * q.BonusPerReferral)
}

func (q *QuotaTracker) CheckLimit(used int, successfulReferrals int) bool {
	limit := q.CalculateQuota(successfulReferrals)
	return used < limit
}
