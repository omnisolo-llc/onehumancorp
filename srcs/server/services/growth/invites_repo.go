package growth

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type TeamInvite struct {
	ID        string
	TeamID    string
	InviterID string
	InviteeID string
	Status    string
	CreatedAt time.Time
	UpdatedAt time.Time
}

type InviteRepository struct {
	db *db.DB
}

func NewInviteRepository(database *db.DB) *InviteRepository {
	return &InviteRepository{db: database}
}

func (r *InviteRepository) CreateInvite(ctx context.Context, invite *TeamInvite) error {
	query := `
		INSERT INTO team_invites (id, team_id, inviter_id, invitee_id, status, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`
	_, err := r.db.Exec(ctx, query, invite.ID, invite.TeamID, invite.InviterID, invite.InviteeID, invite.Status)
	if err != nil {
		return fmt.Errorf("failed to create team invite: %w", err)
	}
	return nil
}

func (r *InviteRepository) GetTeamInvitesCount(ctx context.Context, teamID string) (int, error) {
	query := `SELECT COUNT(*) FROM team_invites WHERE team_id = $1`
	var count int
	row := r.db.QueryRow(ctx, query, teamID)
	err := row.Scan(&count)
	if err != nil {
		return 0, fmt.Errorf("failed to get team invites count: %w", err)
	}
	return count, nil
}

func (r *InviteRepository) GetTotalInvitesCount(ctx context.Context) (int, error) {
	query := `SELECT COUNT(*) FROM team_invites`
	var count int
	row := r.db.QueryRow(ctx, query)
	err := row.Scan(&count)
	if err != nil {
		return 0, fmt.Errorf("failed to get total invites count: %w", err)
	}
	return count, nil
}

func (r *InviteRepository) CreateInvites(ctx context.Context, invites []*TeamInvite) error {
	tx, err := r.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
		INSERT INTO team_invites (id, team_id, inviter_id, invitee_id, status, created_at, updated_at)
		VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`

	for _, invite := range invites {
		_, err := tx.Exec(ctx, query, invite.ID, invite.TeamID, invite.InviterID, invite.InviteeID, invite.Status)
		if err != nil {
			tx.Rollback(ctx)
			return fmt.Errorf("failed to create team invite: %w", err)
		}
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit transaction: %w", err)
	}

	return nil
}
