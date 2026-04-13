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
	referralsByExperimentCounter = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_growth_referrals_by_experiment_total",
			Help: "Total number of referrals added per experiment variant",
		},
		[]string{"experiment_id", "variant"},
	)
)

func init() {
	prometheus.MustRegister(referralsCounter)
	prometheus.MustRegister(referralsByExperimentCounter)
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

func (rt *ReferralTracker) TrackExperimentReferral(experimentID, variant string) {
	referralsCounter.Inc()
	referralsByExperimentCounter.WithLabelValues(experimentID, variant).Inc()
	rt.TotalReferrals++
}
