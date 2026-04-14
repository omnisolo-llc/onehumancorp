package growth

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"sync"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var referralsCounter metric.Int64Counter

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/ohc")
	referralsCounter, _ = meter.Int64Counter("growth_referrals_total")
}

type ReferralTracker struct {
	mu             sync.RWMutex
	TotalReferrals int
	UserReferrals  map[string]int
	UserCodes      map[string]string
	CodeToUser     map[string]string
}

func NewReferralTracker() *ReferralTracker {
	return &ReferralTracker{
		UserReferrals: make(map[string]int),
		UserCodes:     make(map[string]string),
		CodeToUser:    make(map[string]string),
	}
}

func (rt *ReferralTracker) GenerateReferralCode(userID string) string {
	rt.mu.Lock()
	defer rt.mu.Unlock()
	if code, exists := rt.UserCodes[userID]; exists {
		return code
	}
	bytes := make([]byte, 4)
	rand.Read(bytes)
	code := hex.EncodeToString(bytes)
	rt.UserCodes[userID] = code
	rt.CodeToUser[code] = userID
	return code
}

func (rt *ReferralTracker) RecordReferral(ctx context.Context, code string) bool {
	rt.mu.Lock()
	defer rt.mu.Unlock()
	userID, exists := rt.CodeToUser[code]
	if !exists {
		return false
	}
	rt.UserReferrals[userID]++
	rt.TotalReferrals++
	if referralsCounter != nil {
		referralsCounter.Add(ctx, 1)
	}
	return true
}

func (rt *ReferralTracker) GetUserReferrals(userID string) int {
	rt.mu.RLock()
	defer rt.mu.RUnlock()
	return rt.UserReferrals[userID]
}

func (rt *ReferralTracker) GetTotalReferrals() int {
	rt.mu.RLock()
	defer rt.mu.RUnlock()
	return rt.TotalReferrals
}
