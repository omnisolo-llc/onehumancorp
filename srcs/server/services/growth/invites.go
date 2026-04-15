package growth

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var invitesCounter metric.Int64Counter
var invitesAcceptedCounter metric.Int64Counter
var invitesRejectedCounter metric.Int64Counter

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/ohc")
	var err error
	invitesCounter, err = meter.Int64Counter("growth_invites_total")
	if err != nil {
		panic(err)
	}
	invitesAcceptedCounter, err = meter.Int64Counter("growth_invites_accepted_total")
	if err != nil {
		panic(err)
	}
	invitesRejectedCounter, err = meter.Int64Counter("growth_invites_rejected_total")
	if err != nil {
		panic(err)
	}
}

type InviteTracker struct {
	repo *InviteRepository
}

func NewInviteTracker(database *db.DB) *InviteTracker {
	return &InviteTracker{
		repo: NewInviteRepository(database),
	}
}

func (it *InviteTracker) RecordInvite(ctx context.Context, teamID, inviterID, inviteeID string) (string, error) {
	invite := &TeamInvite{
		ID:        fmt.Sprintf("inv-%d", time.Now().UnixNano()),
		TeamID:    teamID,
		InviterID: inviterID,
		InviteeID: inviteeID,
		Status:    "PENDING",
	}

	err := it.repo.CreateInvite(ctx, invite)
	if err != nil {
		return "", err
	}

	if invitesCounter != nil {
		invitesCounter.Add(ctx, 1)
	}

	return invite.ID, nil
}

func (it *InviteTracker) AcceptInvite(ctx context.Context, inviteID string) error {
	err := it.repo.UpdateInviteStatus(ctx, inviteID, "ACCEPTED")
	if err != nil {
		return err
	}

	if invitesAcceptedCounter != nil {
		invitesAcceptedCounter.Add(ctx, 1)
	}

	return nil
}

func (it *InviteTracker) RejectInvite(ctx context.Context, inviteID string) error {
	err := it.repo.UpdateInviteStatus(ctx, inviteID, "REJECTED")
	if err != nil {
		return err
	}

	if invitesRejectedCounter != nil {
		invitesRejectedCounter.Add(ctx, 1)
	}

	return nil
}

func (it *InviteTracker) GetTeamInvitesCount(ctx context.Context, teamID string) (int, error) {
	return it.repo.GetTeamInvitesCount(ctx, teamID)
}

func (it *InviteTracker) GetTotalInvitesCount(ctx context.Context) (int, error) {
	return it.repo.GetTotalInvitesCount(ctx)
}
