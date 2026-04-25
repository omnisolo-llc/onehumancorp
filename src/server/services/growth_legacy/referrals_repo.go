package growth

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
	"time"

	"github.com/redis/go-redis/v9"
)

type GrowthReferral struct {
	ID           string    `json:"id"`
	InviterID    string    `json:"inviter_id"`
	InviteeEmail string    `json:"invitee_email"`
	Status       string    `json:"status"`
	CreatedAt    time.Time `json:"created_at"`
	UpdatedAt    time.Time `json:"updated_at"`
}

type ReferralStats struct {
	InvitesSent int    `json:"invites_sent"`
	Signups     int    `json:"signups"`
	RewardTier  string `json:"reward_tier"`
}

type ReferralRepository struct {
	rdb *redis.Client

	// In-memory fallback
	mu        sync.RWMutex
	referrals map[string]*GrowthReferral
}

func NewReferralRepository(rdb *redis.Client) *ReferralRepository {
	return &ReferralRepository{
		rdb:       rdb,
		referrals: make(map[string]*GrowthReferral),
	}
}

func (r *ReferralRepository) SaveReferral(ctx context.Context, referral *GrowthReferral) error {
	now := time.Now()
	if referral.CreatedAt.IsZero() {
		referral.CreatedAt = now
	}
	referral.UpdatedAt = now

	if r.rdb != nil {
		key := fmt.Sprintf("growth:referrals:%s", referral.InviterID)
		data, err := json.Marshal(referral)
		if err != nil {
			return err
		}
		err = r.rdb.HSet(ctx, key, referral.ID, data).Err()
		if err != nil {
			return err
		}
		indexKey := fmt.Sprintf("growth:referral_index:%s", referral.ID)
		return r.rdb.Set(ctx, indexKey, referral.InviterID, 0).Err()
	}

	r.mu.Lock()
	defer r.mu.Unlock()
	r.referrals[referral.ID] = referral
	return nil
}

func (r *ReferralRepository) GetReferralByID(ctx context.Context, referralID string) (*GrowthReferral, error) {
	if r.rdb != nil {
		indexKey := fmt.Sprintf("growth:referral_index:%s", referralID)
		inviterID, err := r.rdb.Get(ctx, indexKey).Result()
		if err != nil {
			return nil, err
		}

		key := fmt.Sprintf("growth:referrals:%s", inviterID)
		dataStr, err := r.rdb.HGet(ctx, key, referralID).Result()
		if err != nil {
			return nil, err
		}

		var ref GrowthReferral
		if err := json.Unmarshal([]byte(dataStr), &ref); err != nil {
			return nil, err
		}
		return &ref, nil
	}

	r.mu.RLock()
	defer r.mu.RUnlock()
	ref, exists := r.referrals[referralID]
	if !exists {
		return nil, fmt.Errorf("referral not found")
	}
	return ref, nil
}

func (r *ReferralRepository) GetReferralsByInviter(ctx context.Context, inviterID string) ([]*GrowthReferral, error) {
	if r.rdb != nil {
		key := fmt.Sprintf("growth:referrals:%s", inviterID)

		resultsMap, err := r.rdb.HGetAll(ctx, key).Result()
		if err != nil {
			return nil, err
		}

		var results []*GrowthReferral
		for _, dataStr := range resultsMap {
			var ref GrowthReferral
			if err := json.Unmarshal([]byte(dataStr), &ref); err == nil {
				results = append(results, &ref)
			}
		}
		return results, nil
	}

	r.mu.RLock()
	defer r.mu.RUnlock()
	var results []*GrowthReferral
	for _, ref := range r.referrals {
		if ref.InviterID == inviterID {
			results = append(results, ref)
		}
	}
	return results, nil
}

func (r *ReferralRepository) GetStats(ctx context.Context, inviterID string) (*ReferralStats, error) {
	referrals, err := r.GetReferralsByInviter(ctx, inviterID)
	if err != nil {
		return nil, err
	}

	stats := &ReferralStats{
		InvitesSent: len(referrals),
		Signups:     0,
		RewardTier:  "Bronze",
	}

	for _, ref := range referrals {
		if ref.Status == "SIGNED_UP" {
			stats.Signups++
		}
	}

	if stats.Signups >= 50 {
		stats.RewardTier = "Platinum"
	} else if stats.Signups >= 20 {
		stats.RewardTier = "Gold"
	} else if stats.Signups >= 5 {
		stats.RewardTier = "Silver"
	}

	return stats, nil
}


func (r *ReferralRepository) GetAllReferrals(ctx context.Context) ([]*GrowthReferral, error) {
	if r.rdb != nil {
		var results []*GrowthReferral
		var keys []string

		// Use SCAN to find all growth:referrals:* keys
		iter := r.rdb.Scan(ctx, 0, "growth:referrals:*", 0).Iterator()
		for iter.Next(ctx) {
			keys = append(keys, iter.Val())
		}
		if err := iter.Err(); err != nil {
			return nil, err
		}

		if len(keys) == 0 {
			return results, nil
		}

		// Use a pipeline to fetch all data in one go
		pipe := r.rdb.Pipeline()
		var cmds []*redis.MapStringStringCmd
		for _, key := range keys {
			cmds = append(cmds, pipe.HGetAll(ctx, key))
		}

		_, err := pipe.Exec(ctx)
		if err != nil && err != redis.Nil {
			return nil, err
		}

		for _, cmd := range cmds {
			resultsMap := cmd.Val()
			for _, dataStr := range resultsMap {
				var ref GrowthReferral
				if err := json.Unmarshal([]byte(dataStr), &ref); err == nil {
					results = append(results, &ref)
				}
			}
		}

		return results, nil
	}

	r.mu.RLock()
	defer r.mu.RUnlock()
	var results []*GrowthReferral
	for _, ref := range r.referrals {
		results = append(results, ref)
	}
	return results, nil
}


func (r *ReferralRepository) GetViralCoefficient(ctx context.Context) (float64, error) {
	referrals, err := r.GetAllReferrals(ctx)
	if err != nil {
		return 0, err
	}

	if len(referrals) == 0 {
		return 0, nil
	}

	uniqueInviters := make(map[string]bool)
	signedUpCount := 0

	for _, ref := range referrals {
		uniqueInviters[ref.InviterID] = true
		if ref.Status == "SIGNED_UP" {
			signedUpCount++
		}
	}

	if len(uniqueInviters) == 0 {
		return 0, nil
	}

	return float64(signedUpCount) / float64(len(uniqueInviters)), nil
}
