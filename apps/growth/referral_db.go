package growth

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"errors"
	"time"
	"database/sql"
)

type Invite struct {
	Code      string
	TeamID    string
	InviterID string
	CreatedAt time.Time
	Redeemed  bool
}

type DBReferralManager struct {
	db *sql.DB
}

func NewDBReferralManager(db *sql.DB) *DBReferralManager {
	return &DBReferralManager{db: db}
}

func generateCode() (string, error) {
	b := make([]byte, 8)
	_, err := rand.Read(b)
	if err != nil {
		return "", err
	}
	return hex.EncodeToString(b), nil
}

func (rm *DBReferralManager) GenerateInvite(ctx context.Context, teamID, inviterID string) (*Invite, error) {
	if teamID == "" || inviterID == "" {
		return nil, errors.New("teamID and inviterID must not be empty")
	}

	code, err := generateCode()
	if err != nil {
		return nil, err
	}

	invite := &Invite{
		Code:      code,
		TeamID:    teamID,
		InviterID: inviterID,
		CreatedAt: time.Now(),
		Redeemed:  false,
	}

	query := `INSERT INTO invites (code, team_id, inviter_id, created_at, redeemed) VALUES ($1, $2, $3, $4, $5)`
	_, err = rm.db.ExecContext(ctx, query, invite.Code, invite.TeamID, invite.InviterID, invite.CreatedAt, invite.Redeemed)
	if err != nil {
		return nil, err
	}

	return invite, nil
}

func (rm *DBReferralManager) RedeemInvite(ctx context.Context, code string) (*Invite, error) {
	if code == "" {
		return nil, errors.New("code must not be empty")
	}

	tx, err := rm.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	// Use standard SQLite lookup (no FOR UPDATE since modernc.org/sqlite doesn't support it directly)
	query := `SELECT code, team_id, inviter_id, created_at, redeemed FROM invites WHERE code = $1`
	row := tx.QueryRowContext(ctx, query, code)

	var invite Invite
	err = row.Scan(&invite.Code, &invite.TeamID, &invite.InviterID, &invite.CreatedAt, &invite.Redeemed)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, errors.New("invite not found")
		}
		return nil, err
	}

	if invite.Redeemed {
		return nil, errors.New("invite already redeemed")
	}

	updateQuery := `UPDATE invites SET redeemed = true WHERE code = $1`
	_, err = tx.ExecContext(ctx, updateQuery, code)
	if err != nil {
		return nil, err
	}

	err = tx.Commit()
	if err != nil {
		return nil, err
	}

	invite.Redeemed = true
	return &invite, nil
}
