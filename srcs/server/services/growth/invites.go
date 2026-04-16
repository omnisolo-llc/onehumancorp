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

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/ohc")
	var err error
	invitesCounter, err = meter.Int64Counter("growth_invites_total")
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

func (it *InviteTracker) RecordInvite(ctx context.Context, teamID, inviterID, inviteeID string) error {
	invite := &TeamInvite{
		ID:        fmt.Sprintf("inv-%d", time.Now().UnixNano()),
		TeamID:    teamID,
		InviterID: inviterID,
		InviteeID: inviteeID,
		Status:    "PENDING",
	}

	err := it.repo.CreateInvite(ctx, invite)
	if err != nil {
		return err
	}

	if invitesCounter != nil {
		invitesCounter.Add(ctx, 1)
	}

	return nil
}

func (it *InviteTracker) GetTeamInvitesCount(ctx context.Context, teamID string) (int, error) {
	return it.repo.GetTeamInvitesCount(ctx, teamID)
}

func (it *InviteTracker) GetTotalInvitesCount(ctx context.Context) (int, error) {
	return it.repo.GetTotalInvitesCount(ctx)
}

func (it *InviteTracker) RecordInvites(ctx context.Context, teamID, inviterID string, inviteeIDs []string) error {
	var invites []*TeamInvite
	for _, inviteeID := range inviteeIDs {
		invite := &TeamInvite{
			ID:        fmt.Sprintf("inv-%d-%s", time.Now().UnixNano(), inviteeID),
			TeamID:    teamID,
			InviterID: inviterID,
			InviteeID: inviteeID,
			Status:    "PENDING",
		}
		invites = append(invites, invite)
	}

	err := it.repo.CreateInvites(ctx, invites)
	if err != nil {
		return err
	}

	if invitesCounter != nil {
		invitesCounter.Add(ctx, int64(len(invites)))
	}

	return nil
}
