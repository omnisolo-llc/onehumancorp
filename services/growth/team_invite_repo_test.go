package growth

import (
	"context"
	"testing"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

func TestTeamInviteRepository_InMemory(t *testing.T) {
	repo := NewTeamInviteRepository(nil)
	ctx := context.Background()

	inv := &TeamInvite{
		ID:           "inv-1",
		TenantID:     "t-1",
		InviterID:    "u-1",
		InviteeEmail: "test@example.com",
		Status:       "PENDING",
	}

	err := repo.SaveInvite(ctx, inv)
	if err != nil {
		t.Fatalf("Failed to save invite: %v", err)
	}

	fetched, err := repo.GetInviteByID(ctx, "inv-1")
	if err != nil {
		t.Fatalf("Failed to fetch invite: %v", err)
	}
	if fetched.InviteeEmail != "test@example.com" {
		t.Errorf("Expected email test@example.com, got %s", fetched.InviteeEmail)
	}

	invites, err := repo.GetInvitesByTenant(ctx, "t-1")
	if err != nil {
		t.Fatalf("Failed to fetch invites by tenant: %v", err)
	}
	if len(invites) != 1 {
		t.Errorf("Expected 1 invite, got %d", len(invites))
	}

	_, err = repo.GetInviteByID(ctx, "non-existent")
	if err == nil {
		t.Error("Expected error fetching non-existent invite")
	}
}

func TestTeamInviteRepository_Redis(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer mr.Close()

	rdb := redis.NewClient(&redis.Options{Addr: mr.Addr()})
	repo := NewTeamInviteRepository(rdb)
	ctx := context.Background()

	inv := &TeamInvite{
		ID:           "inv-2",
		TenantID:     "t-2",
		InviterID:    "u-2",
		InviteeEmail: "test2@example.com",
		Status:       "PENDING",
	}

	err = repo.SaveInvite(ctx, inv)
	if err != nil {
		t.Fatalf("Failed to save invite to redis: %v", err)
	}

	fetched, err := repo.GetInviteByID(ctx, "inv-2")
	if err != nil {
		t.Fatalf("Failed to fetch invite from redis: %v", err)
	}
	if fetched.InviteeEmail != "test2@example.com" {
		t.Errorf("Expected email test2@example.com, got %s", fetched.InviteeEmail)
	}

	invites, err := repo.GetInvitesByTenant(ctx, "t-2")
	if err != nil {
		t.Fatalf("Failed to fetch invites by tenant from redis: %v", err)
	}
	if len(invites) != 1 {
		t.Errorf("Expected 1 invite, got %d", len(invites))
	}

	_, err = repo.GetInviteByID(ctx, "non-existent")
	if err == nil {
		t.Error("Expected error fetching non-existent invite from redis")
	}
}
