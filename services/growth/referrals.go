package growth

import (
	"github.com/prometheus/client_golang/prometheus"
)

var (
	referralsCounter = prometheus.NewCounter(
		prometheus.CounterOpts{
			Name: "ohc_growth_referrals_total",
			Help: "Total number of referrals added",
		},
	)
)

func init() {
	prometheus.MustRegister(referralsCounter)
}


type ReferralTracker struct {
	TotalReferrals int
}

func NewReferralTracker() *ReferralTracker {
	return &ReferralTracker{}
}

func (rt *ReferralTracker) AddReferral() {
	referralsCounter.Inc()
	rt.TotalReferrals++
}

func (rt *ReferralTracker) GetTotalReferrals() int {
	return rt.TotalReferrals
}
