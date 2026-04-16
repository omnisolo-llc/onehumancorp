package growth

import (
	"context"
	"testing"
)

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

func TestReferralRepository_AcceptReferral(t *testing.T) {
	repo := NewReferralRepository(nil)
	ctx := context.Background()

	referral := &GrowthReferral{
		ID:           "ref-2",
		InviterID:    "user-2",
		InviteeEmail: "newuser@example.com",
		Status:       "PENDING",
	}

	err := repo.SaveReferral(ctx, referral)
	if err != nil {
		t.Fatalf("Failed to save referral: %v", err)
	}

	err = repo.AcceptReferral(ctx, "ref-2")
	if err != nil {
		t.Fatalf("Failed to accept referral: %v", err)
	}

	stats, err := repo.GetStats(ctx, "user-2")
	if err != nil {
		t.Fatalf("Failed to get stats: %v", err)
	}

	if stats.Signups != 1 {
		t.Errorf("Expected 1 signup after accepting referral, got %d", stats.Signups)
	}
}
