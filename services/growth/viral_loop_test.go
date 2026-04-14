package growth

import (
	"context"
	"testing"
)

func TestViralLoopManager(t *testing.T) {
	vlm := NewInMemViralLoopManager()
	ctx := context.Background()

	// Test IssueReward
	reward1, err := vlm.IssueReward(ctx, "user-1", "referral", 100)
	if err != nil {
		t.Fatalf("Failed to issue reward: %v", err)
	}
	if reward1.UserID != "user-1" {
		t.Errorf("Expected UserID to be 'user-1', got '%s'", reward1.UserID)
	}
	if reward1.RewardType != "referral" {
		t.Errorf("Expected RewardType to be 'referral', got '%s'", reward1.RewardType)
	}
	if reward1.Amount != 100 {
		t.Errorf("Expected Amount to be 100, got %d", reward1.Amount)
	}

	// Test GetRewards
	rewards, err := vlm.GetRewards(ctx, "user-1")
	if err != nil {
		t.Fatalf("Failed to get rewards: %v", err)
	}
	if len(rewards) != 1 {
		t.Errorf("Expected 1 reward, got %d", len(rewards))
	}
	if rewards[0].Amount != 100 {
		t.Errorf("Expected reward amount to be 100, got %d", rewards[0].Amount)
	}

	// Issue another reward
	_, err = vlm.IssueReward(ctx, "user-1", "signup", 50)
	if err != nil {
		t.Fatalf("Failed to issue second reward: %v", err)
	}

	rewards, err = vlm.GetRewards(ctx, "user-1")
	if err != nil {
		t.Fatalf("Failed to get rewards: %v", err)
	}
	if len(rewards) != 2 {
		t.Errorf("Expected 2 rewards, got %d", len(rewards))
	}
	// Verify order is descending
	if rewards[0].Amount != 50 {
		t.Errorf("Expected newest reward amount to be 50, got %d", rewards[0].Amount)
	}

	// Test empty rewards
	emptyRewards, err := vlm.GetRewards(ctx, "user-2")
	if err != nil {
		t.Fatalf("Failed to get empty rewards: %v", err)
	}
	if len(emptyRewards) != 0 {
		t.Errorf("Expected 0 rewards, got %d", len(emptyRewards))
	}
}
