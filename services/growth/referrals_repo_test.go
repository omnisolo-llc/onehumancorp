package growth

import (
	"context"
	"testing"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

func TestReferralRepository_GetReferralByID_Redis(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer mr.Close()

	rdb := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	repo := NewReferralRepository(rdb)
	ctx := context.Background()

	referral := &GrowthReferral{
		ID:           "ref-redis-1",
		InviterID:    "user-redis",
		InviteeEmail: "redis@example.com",
		Status:       "PENDING",
	}

	err = repo.SaveReferral(ctx, referral)
	if err != nil {
		t.Fatalf("Failed to save referral: %v", err)
	}

	retrieved, err := repo.GetReferralByID(ctx, "ref-redis-1")
	if err != nil {
		t.Fatalf("Failed to get referral by ID: %v", err)
	}
	if retrieved.InviterID != "user-redis" {
		t.Errorf("Expected inviter ID user-redis, got %s", retrieved.InviterID)
	}
}

func TestReferralRepository_GetReferralByID_InMemory(t *testing.T) {
	repo := NewReferralRepository(nil)
	ctx := context.Background()

	referral := &GrowthReferral{
		ID:           "ref-mem-1",
		InviterID:    "user-mem",
		InviteeEmail: "mem@example.com",
		Status:       "PENDING",
	}

	err := repo.SaveReferral(ctx, referral)
	if err != nil {
		t.Fatalf("Failed to save referral: %v", err)
	}

	retrieved, err := repo.GetReferralByID(ctx, "ref-mem-1")
	if err != nil {
		t.Fatalf("Failed to get referral by ID: %v", err)
	}
	if retrieved.InviterID != "user-mem" {
		t.Errorf("Expected inviter ID user-mem, got %s", retrieved.InviterID)
	}

	_, err = repo.GetReferralByID(ctx, "non-existent")
	if err == nil {
		t.Errorf("Expected error for non-existent referral, got nil")
	}
}

func TestReferralRepository_InMemoryFallback(t *testing.T) {
	repo := NewReferralRepository(nil)
	ctx := context.Background()

	referral := &GrowthReferral{
		ID:           "ref-1",
		InviterID:    "user-1",
		InviteeEmail: "test@example.com",
		Status:       "SIGNED_UP",
	}

	err := repo.SaveReferral(ctx, referral)
	if err != nil {
		t.Fatalf("Failed to save referral: %v", err)
	}

	stats, err := repo.GetStats(ctx, "user-1")
	if err != nil {
		t.Fatalf("Failed to get stats: %v", err)
	}

	if stats.InvitesSent != 1 {
		t.Errorf("Expected 1 invite sent, got %d", stats.InvitesSent)
	}
	if stats.Signups != 1 {
		t.Errorf("Expected 1 signup, got %d", stats.Signups)
	}
	if stats.RewardTier != "Bronze" {
		t.Errorf("Expected Bronze tier, got %s", stats.RewardTier)
	}
}

func TestReferralRepository_RewardTiers(t *testing.T) {
	repo := NewReferralRepository(nil)
	ctx := context.Background()

	for i := 0; i < 25; i++ {
		referral := &GrowthReferral{
			ID:           string(rune(i)),
			InviterID:    "user-gold",
			InviteeEmail: "test@example.com",
			Status:       "SIGNED_UP",
		}
		_ = repo.SaveReferral(ctx, referral)
	}

	stats, err := repo.GetStats(ctx, "user-gold")
	if err != nil {
		t.Fatalf("Failed to get stats: %v", err)
	}

	if stats.RewardTier != "Gold" {
		t.Errorf("Expected Gold tier, got %s", stats.RewardTier)
	}
}
