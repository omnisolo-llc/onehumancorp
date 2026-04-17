package domain

import (
	"context"
	"fmt"
	"sync"
	"time"

	"go.opentelemetry.io/otel"
	"go.opentelemetry.io/otel/metric"
)

var (
	invitesCreatedCounter metric.Int64Counter
	invitesAcceptedCounter metric.Int64Counter
)

func init() {
	meter := otel.Meter("github.com/onehumancorp/mono/ohc")
	var err error
	invitesCreatedCounter, err = meter.Int64Counter("growth_invites_created_total")
	if err != nil {
		panic(err)
	}
	invitesAcceptedCounter, err = meter.Int64Counter("growth_invites_accepted_total")
	if err != nil {
		panic(err)
	}
}

type Invite struct {
	ID        string    `json:"id"`
	InviterID string    `json:"inviterId"`
	InviteeID string    `json:"inviteeId"`
	Status    string    `json:"status"` // PENDING, ACCEPTED
	CreatedAt time.Time `json:"createdAt"`
}

type InviteService struct {
	mu      sync.RWMutex
	invites map[string]*Invite
}

func NewInviteService() *InviteService {
	return &InviteService{
		invites: make(map[string]*Invite),
	}
}

func (s *InviteService) CreateInvite(ctx context.Context, inviterID, inviteeID string) (*Invite, error) {
	if inviterID == "" || inviteeID == "" {
		return nil, fmt.Errorf("inviterID and inviteeID are required")
	}

	s.mu.Lock()
	defer s.mu.Unlock()

	id := "inv-" + time.Now().UTC().Format("20060102150405") + "-" + inviteeID
	invite := &Invite{
		ID:        id,
		InviterID: inviterID,
		InviteeID: inviteeID,
		Status:    "PENDING",
		CreatedAt: time.Now().UTC(),
	}
	s.invites[id] = invite

	if invitesCreatedCounter != nil {
		invitesCreatedCounter.Add(ctx, 1)
	}

	return invite, nil
}

func (s *InviteService) AcceptInvite(ctx context.Context, id string) (*Invite, error) {
	if id == "" {
		return nil, fmt.Errorf("id is required")
	}

	s.mu.Lock()
	defer s.mu.Unlock()

	invite, exists := s.invites[id]
	if !exists {
		return nil, fmt.Errorf("invite not found")
	}

	if invite.Status == "ACCEPTED" {
		return nil, fmt.Errorf("invite already accepted")
	}

	invite.Status = "ACCEPTED"

	if invitesAcceptedCounter != nil {
		invitesAcceptedCounter.Add(ctx, 1)
	}

	return invite, nil
}
