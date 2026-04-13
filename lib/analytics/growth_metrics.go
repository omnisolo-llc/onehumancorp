package analytics

import (
	"time"
)

// ViralMetrics contains aggregated growth data for the viral referral loop.
type ViralMetrics struct {
	TotalReferrals   int       `json:"totalReferrals"`
	TotalConversions int       `json:"totalConversions"`
	UniqueInviters   int       `json:"uniqueInviters"`
	KFactor          float64   `json:"kFactor"`
	LastUpdated      time.Time `json:"lastUpdated"`
}

// ReferralSource indicates the origin of a referral.
type ReferralSource string

const (
	SourceDirect     ReferralSource = "direct"
	SourceStandalone ReferralSource = "standalone"
	SourceCloud      ReferralSource = "cloud"
	SourceSocial     ReferralSource = "social"
)

// ComputeViralCoefficient calculates the K-factor based on conversions and inviters.
func ComputeViralCoefficient(conversions, inviters int) float64 {
	if inviters <= 0 {
		return 0.0
	}
	return float64(conversions) / float64(inviters)
}

// CalculateConversionRate returns the percentage of referrals that converted.
func CalculateConversionRate(referrals, conversions int) float64 {
	if referrals <= 0 {
		return 0.0
	}
	return (float64(conversions) / float64(referrals)) * 100.0
}
