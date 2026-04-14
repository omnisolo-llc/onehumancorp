package analytics

import (
	"sync"
)

// ViralLoopTracker tracks referrals and calculates the viral coefficient (K-factor).
type ViralLoopTracker struct {
	mu          sync.RWMutex
	referrals   map[string]int
	conversions map[string]int
}

// NewViralLoopTracker initializes a new ViralLoopTracker.
func NewViralLoopTracker() *ViralLoopTracker {
	return &ViralLoopTracker{
		referrals:   make(map[string]int),
		conversions: make(map[string]int),
	}
}

// RecordReferral increments the referral count for an inviter.
func (v *ViralLoopTracker) RecordReferral(inviterID string) {
	v.mu.Lock()
	defer v.mu.Unlock()
	v.referrals[inviterID]++
}

// RecordConversion increments the conversion count for an inviter.
func (v *ViralLoopTracker) RecordConversion(inviterID string) {
	v.mu.Lock()
	defer v.mu.Unlock()
	v.conversions[inviterID]++
}

// GetKFactor calculates the K-factor.
func (v *ViralLoopTracker) GetKFactor() float64 {
	v.mu.RLock()
	defer v.mu.RUnlock()

	totalConversions := 0
	for _, count := range v.conversions {
		totalConversions += count
	}

	uniqueInviters := len(v.referrals)
	if uniqueInviters == 0 {
		return 0.0
	}

	return float64(totalConversions) / float64(uniqueInviters)
}
