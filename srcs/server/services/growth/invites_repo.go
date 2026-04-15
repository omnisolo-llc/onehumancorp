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

func (r *InviteRepository) GetInvite(ctx context.Context, inviteID string) (*TeamInvite, error) {
	query := `
		SELECT id, team_id, inviter_id, invitee_id, status, created_at, updated_at
		FROM team_invites
		WHERE id = $1
	`
	var invite TeamInvite
	var createdAt, updatedAt db.FlexTime
	err := r.db.QueryRow(ctx, query, inviteID).Scan(
		&invite.ID,
		&invite.TeamID,
		&invite.InviterID,
		&invite.InviteeID,
		&invite.Status,
		&createdAt,
		&updatedAt,
	)
	if err != nil {
		return nil, fmt.Errorf("failed to get team invite: %w", err)
	}
	invite.CreatedAt = createdAt.Time
	invite.UpdatedAt = updatedAt.Time
	return &invite, nil
}

func (r *InviteRepository) UpdateInviteStatus(ctx context.Context, inviteID string, status string) error {
	query := `
		UPDATE team_invites
		SET status = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
	`
	rowsAffected, err := r.db.Exec(ctx, query, status, inviteID)
	if err != nil {
		return fmt.Errorf("failed to update invite status: %w", err)
	}

	if rowsAffected == 0 {
		return fmt.Errorf("invite with id %s not found", inviteID)
	}

	return nil
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
