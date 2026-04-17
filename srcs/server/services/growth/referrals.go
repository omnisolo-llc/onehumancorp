package growth

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
	"sync"
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
	UserCodes      map[string][]string
	CodeToUser     map[string]string
	ChannelStats   map[string]int // Track referral source channels
}

func NewReferralTracker() *ReferralTracker {
	return &ReferralTracker{
		UserReferrals: make(map[string]int),
		UserCodes:     make(map[string][]string),
		CodeToUser:    make(map[string]string),
		ChannelStats:  make(map[string]int),
	}
}

func (rt *ReferralTracker) GenerateReferralCode(userID string) string {
	rt.mu.Lock()
	defer rt.mu.Unlock()
	if codes, exists := rt.UserCodes[userID]; exists && len(codes) > 0 {
		return codes[0]
	}
	bytes := make([]byte, 4)
	if _, err := rand.Read(bytes); err != nil {
		// Log error or panic, here returning empty or handle properly.
		// Changing signature of GenerateReferralCode is bad without changing test.
		// Since we can't change signature, we fallback to something safe or panic.
		panic("failed to read random bytes: " + err.Error())
	}
	code := hex.EncodeToString(bytes)
	rt.UserCodes[userID] = append(rt.UserCodes[userID], code)
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

func (rt *ReferralTracker) RecordReferralWithChannel(ctx context.Context, code string, channel string) bool {
	rt.mu.Lock()
	defer rt.mu.Unlock()
	userID, exists := rt.CodeToUser[code]
	if !exists {
		return false
	}
	rt.UserReferrals[userID]++
	rt.TotalReferrals++

	if channel != "" {
		rt.ChannelStats[channel]++
	}

	if referralsCounter != nil {
		referralsCounter.Add(ctx, 1)
	}
	return true
}

func (rt *ReferralTracker) GetChannelStats() map[string]int {
	rt.mu.RLock()
	defer rt.mu.RUnlock()

	stats := make(map[string]int)
	for k, v := range rt.ChannelStats {
		stats[k] = v
	}
	return stats
}

func CalculateReferralTier(referrals int) string {
	if referrals >= 50 {
		return "Platinum"
	} else if referrals >= 20 {
		return "Gold"
	} else if referrals >= 5 {
		return "Silver"
	}
	return "Bronze"
}

func CalculateTierDiscount(tier string) float64 {
	switch tier {
	case "Platinum":
		return 0.20
	case "Gold":
		return 0.10
	case "Silver":
		return 0.05
	case "Bronze":
		return 0.00
	default:
		return 0.00
	}
}

func (rt *ReferralTracker) GenerateBulkReferralCodes(userID string, count int, maxCount int) ([]string, error) {
	rt.mu.Lock()
	defer rt.mu.Unlock()

	if count > maxCount {
		return nil, fmt.Errorf("requested count %d exceeds maximum allowed %d", count, maxCount)
	}

	if len(rt.UserCodes[userID])+count > maxCount {
		return nil, fmt.Errorf("user %s will exceed max code limit %d", userID, maxCount)
	}

	var codes []string
	for i := 0; i < count; i++ {
		bytes := make([]byte, 4)
		if _, err := rand.Read(bytes); err != nil {
			panic("failed to read random bytes: " + err.Error())
		}
		code := hex.EncodeToString(bytes)

		for {
			if _, exists := rt.CodeToUser[code]; !exists {
				break
			}
			if _, err := rand.Read(bytes); err != nil {
				panic("failed to read random bytes: " + err.Error())
			}
			code = hex.EncodeToString(bytes)
		}

		rt.UserCodes[userID] = append(rt.UserCodes[userID], code)
		rt.CodeToUser[code] = userID
		codes = append(codes, code)
	}
	return codes, nil
}

func (rt *ReferralTracker) RecordBulkReferrals(ctx context.Context, codes []string) int {
	rt.mu.Lock()
	defer rt.mu.Unlock()

	successCount := 0
	for _, code := range codes {
		userID, exists := rt.CodeToUser[code]
		if !exists {
			continue
		}
		rt.UserReferrals[userID]++
		rt.TotalReferrals++
		successCount++
	}

	if successCount > 0 && referralsCounter != nil {
		referralsCounter.Add(ctx, int64(successCount))
	}
	return successCount
}
