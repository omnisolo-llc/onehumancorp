package domain

import (
	"context"
	"testing"
)

func TestCreateInvite(t *testing.T) {
	svc := NewInviteService(nil)

	// Test successful creation
	inv, err := svc.CreateInvite(context.Background(), "user1", "user2")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if inv == nil {
		t.Fatal("expected invite to be returned")
	}
	if inv.Status != "PENDING" {
		t.Fatalf("expected status to be PENDING, got %s", inv.Status)
	}
	if inv.InviterID != "user1" || inv.InviteeID != "user2" {
		t.Fatalf("expected inviter user1 and invitee user2, got %s and %s", inv.InviterID, inv.InviteeID)
	}

	// Test error conditions
	_, err = svc.CreateInvite(context.Background(), "", "user2")
	if err == nil {
		t.Fatal("expected error for empty inviter, got none")
	}

	_, err = svc.CreateInvite(context.Background(), "user1", "")
	if err == nil {
		t.Fatal("expected error for empty invitee, got none")
	}
}

func TestAcceptInvite(t *testing.T) {
	svc := NewInviteService(nil)
	inv, _ := svc.CreateInvite(context.Background(), "user1", "user2")

	// Test successful acceptance
	accepted, err := svc.AcceptInvite(context.Background(), inv.ID)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if accepted == nil {
		t.Fatal("expected accepted invite to be returned")
	}
	if accepted.Status != "ACCEPTED" {
		t.Fatalf("expected status to be ACCEPTED, got %s", accepted.Status)
	}

	// Test error conditions
	_, err = svc.AcceptInvite(context.Background(), "")
	if err == nil {
		t.Fatal("expected error for empty id, got none")
	}

	_, err = svc.AcceptInvite(context.Background(), "non-existent-id")
	if err == nil {
		t.Fatal("expected error for non-existent id, got none")
	}
}
