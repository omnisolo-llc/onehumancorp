package growth

import (
	"context"
	"crypto/rand"
	"encoding/hex"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type ViralLoopService struct {
	db db.Provider
}

type Referral struct {
	ID             string    `json:"id"`
	OrganizationID string    `json:"organization_id"`
	InviterID      string    `json:"inviter_id"`
	InviteeEmail   string    `json:"invitee_email"`
	Status         string    `json:"status"` // PENDING, CONVERTED
	CreatedAt      time.Time `json:"created_at"`
}

func NewViralLoopService(db db.Provider) *ViralLoopService {
	return &ViralLoopService{db: db}
}

func generateID() string {
	b := make([]byte, 16)
	_, _ = rand.Read(b)
	return hex.EncodeToString(b[0:4]) + "-" + hex.EncodeToString(b[4:6]) + "-" + hex.EncodeToString(b[6:8]) + "-" + hex.EncodeToString(b[8:10]) + "-" + hex.EncodeToString(b[10:])
}

func (s *ViralLoopService) ProcessReferral(ctx context.Context, organizationID, inviterID, inviteeEmail string) (*Referral, error) {
	id := generateID()
	now := time.Now()

	query := `
		INSERT INTO growth_referrals (id, organization_id, inviter_id, invitee_email, status, created_at)
		VALUES ($1, $2, $3, $4, $5, $6)
	`
	_, err := s.db.Exec(ctx, query, id, organizationID, inviterID, inviteeEmail, "PENDING", now.Format(time.RFC3339Nano))
	if err != nil {
		return nil, fmt.Errorf("failed to insert referral: %w", err)
	}

	telemetry.RecordViralReferral(ctx, organizationID)

	return &Referral{
		ID:             id,
		OrganizationID: organizationID,
		InviterID:      inviterID,
		InviteeEmail:   inviteeEmail,
		Status:         "PENDING",
		CreatedAt:      now,
	}, nil
}
