package growth

type ReferralTracker struct {
	TotalReferrals int
}

func NewReferralTracker() *ReferralTracker {
	return &ReferralTracker{}
}

func (rt *ReferralTracker) AddReferral() {
	rt.TotalReferrals++
}

func (rt *ReferralTracker) GetTotalReferrals() int {
	return rt.TotalReferrals
}
