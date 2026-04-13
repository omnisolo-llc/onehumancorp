package growth

import (
	"time"

	"github.com/onehumancorp/mono/lib/analytics"
)

// ReferralEvent tracks a single referral instance.
type ReferralEvent struct {
	ID             string                   `json:"id"`
	InviterID      string                   `json:"inviterId"`
	Source         analytics.ReferralSource `json:"source"`
	Converted      bool                     `json:"converted"`
	ConversionTime *time.Time               `json:"conversionTime,omitempty"`
	CreatedAt      time.Time                `json:"createdAt"`
}

type ReferralTracker struct {
	Events []ReferralEvent
}

func NewReferralTracker() *ReferralTracker {
	return &ReferralTracker{
		Events: make([]ReferralEvent, 0),
	}
}

func (rt *ReferralTracker) TrackReferral(inviterID string, source analytics.ReferralSource) {
	rt.Events = append(rt.Events, ReferralEvent{
		ID:        time.Now().UTC().Format("20060102150405"),
		InviterID: inviterID,
		Source:    source,
		Converted: false,
		CreatedAt: time.Now().UTC(),
	})
}

func (rt *ReferralTracker) MarkConverted(inviterID string) {
	for i := range rt.Events {
		if rt.Events[i].InviterID == inviterID && !rt.Events[i].Converted {
			now := time.Now().UTC()
			rt.Events[i].Converted = true
			rt.Events[i].ConversionTime = &now
			return
		}
	}
}

func (rt *ReferralTracker) GetMetrics() analytics.ViralMetrics {
	totalReferrals := len(rt.Events)
	totalConversions := 0
	inviters := make(map[string]bool)

	for _, event := range rt.Events {
		if event.Converted {
			totalConversions++
		}
		inviters[event.InviterID] = true
	}

	uniqueInviters := len(inviters)
	kFactor := analytics.ComputeViralCoefficient(totalConversions, uniqueInviters)

	return analytics.ViralMetrics{
		TotalReferrals:   totalReferrals,
		TotalConversions: totalConversions,
		UniqueInviters:   uniqueInviters,
		KFactor:          kFactor,
		LastUpdated:      time.Now().UTC(),
	}
}
