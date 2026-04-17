package growth

import (
	"context"
	"testing"
)

func TestReferralTracker(t *testing.T) {
	tracker := NewReferralTracker()
	if tracker.GetTotalReferrals() != 0 {
		t.Fatalf("Expected 0 referrals initially, got %d", tracker.GetTotalReferrals())
	}

	userID := "user123"
	code := tracker.GenerateReferralCode(userID)
	if code == "" {
		t.Fatalf("Expected non-empty referral code")
	}

	// Test recording referral
	ctx := context.Background()
	success := tracker.RecordReferral(ctx, code)
	if !success {
		t.Fatalf("Expected referral to be recorded successfully")
	}

	if tracker.GetTotalReferrals() != 1 {
		t.Fatalf("Expected 1 total referral after record, got %d", tracker.GetTotalReferrals())
	}

	if tracker.GetUserReferrals(userID) != 1 {
		t.Fatalf("Expected 1 referral for user, got %d", tracker.GetUserReferrals(userID))
	}

	// Test invalid code
	success = tracker.RecordReferral(ctx, "invalid_code")
	if success {
		t.Fatalf("Expected invalid code referral to fail")
	}
}

func TestReferralTrackerWithChannel(t *testing.T) {
	tracker := NewReferralTracker()
	userID := "user456"
	code := tracker.GenerateReferralCode(userID)

	ctx := context.Background()
	success := tracker.RecordReferralWithChannel(ctx, code, "twitter")
	if !success {
		t.Fatalf("Expected referral to be recorded successfully")
	}

	stats := tracker.GetChannelStats()
	if stats["twitter"] != 1 {
		t.Fatalf("Expected 1 referral from twitter, got %d", stats["twitter"])
	}
}

func TestCalculateReferralTier(t *testing.T) {
	tests := []struct {
		referrals int
		expected  string
	}{
		{0, "Bronze"},
		{4, "Bronze"},
		{5, "Silver"},
		{19, "Silver"},
		{20, "Gold"},
		{49, "Gold"},
		{50, "Platinum"},
		{100, "Platinum"},
	}

	for _, tt := range tests {
		got := CalculateReferralTier(tt.referrals)
		if got != tt.expected {
			t.Errorf("CalculateReferralTier(%d): expected %s, got %s", tt.referrals, tt.expected, got)
		}
	}
}

func TestCalculateTierDiscount(t *testing.T) {
	tests := []struct {
		tier     string
		expected float64
	}{
		{"Bronze", 0.00},
		{"Silver", 0.05},
		{"Gold", 0.10},
		{"Platinum", 0.20},
		{"Unknown", 0.00},
	}

	for _, tt := range tests {
		got := CalculateTierDiscount(tt.tier)
		if got != tt.expected {
			t.Errorf("CalculateTierDiscount(%q): expected %f, got %f", tt.tier, tt.expected, got)
		}
	}
}
