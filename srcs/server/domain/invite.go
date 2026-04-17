package domain

import (
	"context"
	"fmt"
	"log/slog"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/attribute"
)

type TeamInvite struct {
	ID        string    `json:"id"`
	InviterID string    `json:"inviterId"`
	InviteeID string    `json:"inviteeId"`
	Status    string    `json:"status"` // PENDING, ACCEPTED
	CreatedAt time.Time `json:"createdAt"`
}

type InviteService struct {
	mu      sync.RWMutex
	invites []TeamInvite
	pool    *db.DB
}

func NewInviteService(pool *db.DB) *InviteService {
	return &InviteService{
		invites: []TeamInvite{},
		pool:    pool,
	}
}

func (s *InviteService) CreateInvite(ctx context.Context, inviterID, inviteeID string) (*TeamInvite, error) {
	_, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/domain").Start(ctx, "CreateInvite")
	defer span.End()

	if inviterID == "" || inviteeID == "" {
		return nil, fmt.Errorf("inviterId and inviteeId are required")
	}

	invite := TeamInvite{
		ID:        "inv-" + time.Now().UTC().Format("20060102150405"),
		InviterID: inviterID,
		InviteeID: inviteeID,
		Status:    "PENDING",
		CreatedAt: time.Now().UTC(),
	}

	if s.pool != nil {
		query := `INSERT INTO team_invites (id, team_id, inviter_id, invitee_id, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)`
		// team_invites schema in invites_repo.go expects team_id too. We'll use a dummy team_id for now or ignore since domain logic
		_, err := s.pool.Exec(ctx, query, invite.ID, "default-team", invite.InviterID, invite.InviteeID, invite.Status)
		if err != nil {
			return nil, fmt.Errorf("failed to create team invite in db: %w", err)
		}
	} else {
		s.mu.Lock()
		s.invites = append(s.invites, invite)
		s.mu.Unlock()
	}

	slog.Info("invite created", "id", invite.ID, "inviter", inviterID, "invitee", inviteeID)
	span.SetAttributes(attribute.String("invite.id", invite.ID))

	return &invite, nil
}

func (s *InviteService) AcceptInvite(ctx context.Context, id string) (*TeamInvite, error) {
	_, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/domain").Start(ctx, "AcceptInvite")
	defer span.End()

	if id == "" {
		return nil, fmt.Errorf("id is required")
	}

	var updated *TeamInvite
	if s.pool != nil {
		query := `UPDATE team_invites SET status = 'ACCEPTED', updated_at = CURRENT_TIMESTAMP WHERE id = $1 RETURNING id, inviter_id, invitee_id, status, created_at`
		var inv TeamInvite
		err := s.pool.QueryRow(ctx, query, id).Scan(&inv.ID, &inv.InviterID, &inv.InviteeID, &inv.Status, &inv.CreatedAt)
		if err != nil {
			// Not found or error
		} else {
			updated = &inv
		}
	}

	if updated == nil && s.pool == nil {
		s.mu.Lock()
		defer s.mu.Unlock()
		for i, inv := range s.invites {
			if inv.ID == id {
				s.invites[i].Status = "ACCEPTED"
				updated = &s.invites[i]
				break
			}
		}
	}

	if updated == nil {
		return nil, fmt.Errorf("invite not found")
	}

	slog.Info("invite accepted", "id", id)
	span.SetAttributes(attribute.String("invite.id", id))

	return updated, nil
}
