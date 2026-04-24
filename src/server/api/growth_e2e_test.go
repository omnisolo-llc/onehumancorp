package api

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestGrowthE2EFlow(t *testing.T) {
	it, vt := setupTestDependencies(t)
	handler := NewGrowthHandler(it, vt)

	// Step 1: Send invite
	reqBody1 := InviteRequest{
		TeamID:    "e2e-team",
		InviterID: "inviter-1",
		InviteeID: "invitee-1",
	}
	bodyBytes1, _ := json.Marshal(reqBody1)
	req1 := httptest.NewRequest(http.MethodPost, "/api/v1/invites", bytes.NewBuffer(bodyBytes1))
	w1 := httptest.NewRecorder()

	handler.HandleInvite(w1, req1)

	if w1.Code != http.StatusCreated {
		t.Errorf("Step 1 (Send invite): Expected status %d, got %d", http.StatusCreated, w1.Code)
	}

	// Step 2: Accept invite
	reqBody2 := AcceptInviteRequest{
		InviteeID: "invitee-1",
	}
	bodyBytes2, _ := json.Marshal(reqBody2)
	req2 := httptest.NewRequest(http.MethodPost, "/api/v1/invites/accept", bytes.NewBuffer(bodyBytes2))
	w2 := httptest.NewRecorder()

	handler.HandleAcceptInvite(w2, req2)

	if w2.Code != http.StatusOK {
		t.Errorf("Step 2 (Accept invite): Expected status %d, got %d", http.StatusOK, w2.Code)
	}
}
