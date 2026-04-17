package domain

import (
	"context"
	"testing"
)

func TestInviteService(t *testing.T) {
	ctx := context.Background()
	svc := NewInviteService()

	_, err := svc.CreateInvite(ctx, "", "invitee1")
	if err == nil {
		t.Fatalf("expected error for empty inviterID")
	}
	_, err = svc.CreateInvite(ctx, "inviter1", "")
	if err == nil {
		t.Fatalf("expected error for empty inviteeID")
	}

	inv, err := svc.CreateInvite(ctx, "inviter1", "invitee1")
	if err != nil {
		t.Fatalf("unexpected error creating invite: %v", err)
	}
	if inv.Status != "PENDING" {
		t.Fatalf("expected status PENDING, got %s", inv.Status)
	}
	if inv.InviterID != "inviter1" || inv.InviteeID != "invitee1" {
		t.Fatalf("mismatched inviter or invitee")
	}

	_, err = svc.AcceptInvite(ctx, "")
	if err == nil {
		t.Fatalf("expected error for empty id")
	}

	_, err = svc.AcceptInvite(ctx, "invalid-id")
	if err == nil {
		t.Fatalf("expected error for invalid id")
	}

	accepted, err := svc.AcceptInvite(ctx, inv.ID)
	if err != nil {
		t.Fatalf("unexpected error accepting invite: %v", err)
	}
	if accepted.Status != "ACCEPTED" {
		t.Fatalf("expected status ACCEPTED, got %s", accepted.Status)
	}

	_, err = svc.AcceptInvite(ctx, inv.ID)
	if err == nil {
		t.Fatalf("expected error for already accepted invite")
	}
}
