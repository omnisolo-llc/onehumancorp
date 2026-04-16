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
	referrals     map[string]*GrowthReferral
}

func NewReferralRepository(rdb *redis.Client) *ReferralRepository {
	return &ReferralRepository{
		rdb:       rdb,
		referrals:     make(map[string]*GrowthReferral),
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
		if err := r.rdb.Set(ctx, fmt.Sprintf("growth:referral_index:%s", referral.ID), referral.InviterID, 0).Err(); err != nil {
			return err
		}
		return r.rdb.HSet(ctx, key, referral.ID, data).Err()
	}

	r.mu.Lock()
	defer r.mu.Unlock()
	r.referrals[referral.ID] = referral
	return nil
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

func (r *ReferralRepository) GetReferralByID(ctx context.Context, id string) (*GrowthReferral, error) {
	var inviterID string
	var err error

	if r.rdb != nil {
		inviterID, err = r.rdb.Get(ctx, fmt.Sprintf("growth:referral_index:%s", id)).Result()
		if err == redis.Nil || inviterID == "" {
			// Fallback for pre-existing referrals: scan all keys. This is slow but needed for backward compatibility.
			keys, err := r.rdb.Keys(ctx, "growth:referrals:*").Result()
			if err != nil {
				return nil, fmt.Errorf("referral not found")
			}
			found := false
			for _, k := range keys {
				dataStr, err := r.rdb.HGet(ctx, k, id).Result()
				if err == nil && dataStr != "" {
					var ref GrowthReferral
					if err := json.Unmarshal([]byte(dataStr), &ref); err == nil {
						return &ref, nil
					}
				}
			}
			if !found {
				return nil, fmt.Errorf("referral not found")
			}
		} else if err != nil {
			return nil, fmt.Errorf("referral not found")
		} else {
			key := fmt.Sprintf("growth:referrals:%s", inviterID)
			dataStr, err := r.rdb.HGet(ctx, key, id).Result()
			if err != nil {
				return nil, fmt.Errorf("referral data not found")
			}

			var ref GrowthReferral
			if err := json.Unmarshal([]byte(dataStr), &ref); err != nil {
				return nil, err
			}
			return &ref, nil
		}
	}

	r.mu.RLock()
	defer r.mu.RUnlock()

	// Fallback for memory missing index
	ref, ok := r.referrals[id]
	if !ok {
		return nil, fmt.Errorf("referral data not found")
	}

	// Return a copy so the memory fallback is safe from mutations
	refCopy := *ref
	return &refCopy, nil
}

func (r *ReferralRepository) AcceptReferral(ctx context.Context, id string) error {
	ref, err := r.GetReferralByID(ctx, id)
	if err != nil {
		return err
	}

	ref.Status = "SIGNED_UP"
	ref.UpdatedAt = time.Now()

	return r.SaveReferral(ctx, ref)
}
