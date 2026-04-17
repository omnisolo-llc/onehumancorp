package growth

import (
	"context"
	"encoding/json"
	"fmt"
	"sync"
	"time"

	"github.com/redis/go-redis/v9"
)

type TeamInvite struct {
	ID           string    `json:"id"`
	TenantID     string    `json:"tenant_id"`
	InviterID    string    `json:"inviter_id"`
	InviteeEmail string    `json:"invitee_email"`
	Status       string    `json:"status"` // PENDING, ACCEPTED
	CreatedAt    time.Time `json:"created_at"`
	UpdatedAt    time.Time `json:"updated_at"`
}

type TeamInviteRepository struct {
	rdb *redis.Client

	// In-memory fallback
	mu      sync.RWMutex
	invites map[string]*TeamInvite
}

func NewTeamInviteRepository(rdb *redis.Client) *TeamInviteRepository {
	return &TeamInviteRepository{
		rdb:     rdb,
		invites: make(map[string]*TeamInvite),
	}
}

func (r *TeamInviteRepository) SaveInvite(ctx context.Context, invite *TeamInvite) error {
	now := time.Now()
	if invite.CreatedAt.IsZero() {
		invite.CreatedAt = now
	}
	invite.UpdatedAt = now

	if r.rdb != nil {
		key := fmt.Sprintf("growth:team_invites:%s", invite.TenantID)
		data, err := json.Marshal(invite)
		if err != nil {
			return err
		}
		err = r.rdb.HSet(ctx, key, invite.ID, data).Err()
		if err != nil {
			return err
		}
		indexKey := fmt.Sprintf("growth:team_invite_index:%s", invite.ID)
		return r.rdb.Set(ctx, indexKey, invite.TenantID, 0).Err()
	}

	r.mu.Lock()
	defer r.mu.Unlock()
	r.invites[invite.ID] = invite
	return nil
}

func (r *TeamInviteRepository) GetInviteByID(ctx context.Context, inviteID string) (*TeamInvite, error) {
	if r.rdb != nil {
		indexKey := fmt.Sprintf("growth:team_invite_index:%s", inviteID)
		tenantID, err := r.rdb.Get(ctx, indexKey).Result()
		if err != nil {
			return nil, err
		}

		key := fmt.Sprintf("growth:team_invites:%s", tenantID)
		dataStr, err := r.rdb.HGet(ctx, key, inviteID).Result()
		if err != nil {
			return nil, err
		}

		var inv TeamInvite
		if err := json.Unmarshal([]byte(dataStr), &inv); err != nil {
			return nil, err
		}
		return &inv, nil
	}

	r.mu.RLock()
	defer r.mu.RUnlock()
	inv, exists := r.invites[inviteID]
	if !exists {
		return nil, fmt.Errorf("invite not found")
	}
	return inv, nil
}

func (r *TeamInviteRepository) GetInvitesByTenant(ctx context.Context, tenantID string) ([]*TeamInvite, error) {
	if r.rdb != nil {
		key := fmt.Sprintf("growth:team_invites:%s", tenantID)

		resultsMap, err := r.rdb.HGetAll(ctx, key).Result()
		if err != nil {
			return nil, err
		}

		var results []*TeamInvite
		for _, dataStr := range resultsMap {
			var inv TeamInvite
			if err := json.Unmarshal([]byte(dataStr), &inv); err == nil {
				results = append(results, &inv)
			}
		}
		return results, nil
	}

	r.mu.RLock()
	defer r.mu.RUnlock()
	var results []*TeamInvite
	for _, inv := range r.invites {
		if inv.TenantID == tenantID {
			results = append(results, inv)
		}
	}
	return results, nil
}
