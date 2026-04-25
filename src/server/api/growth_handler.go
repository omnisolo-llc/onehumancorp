package api

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/src/server/services/growth"
)

type GrowthHandler struct {
	InviteTracker    *growth.InviteTracker
	ViralLoopTracker *growth.ViralLoopTracker
}

func NewGrowthHandler(it *growth.InviteTracker, vt *growth.ViralLoopTracker) *GrowthHandler {
	return &GrowthHandler{
		InviteTracker:    it,
		ViralLoopTracker: vt,
	}
}

type InviteRequest struct {
	TeamID    string `json:"team_id"`
	InviterID string `json:"inviter_id"`
	InviteeID string `json:"invitee_id"`
}

func (h *GrowthHandler) HandleInvite(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req InviteRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if req.TeamID == "" || req.InviterID == "" || req.InviteeID == "" {
		http.Error(w, "Missing required fields", http.StatusBadRequest)
		return
	}

	ctx := r.Context()
	err := h.InviteTracker.RecordInvite(ctx, req.TeamID, req.InviterID, req.InviteeID)
	if err != nil {
		http.Error(w, "Failed to record invite", http.StatusInternalServerError)
		return
	}

	h.ViralLoopTracker.RecordInviteSent(ctx, req.InviterID)

	w.WriteHeader(http.StatusCreated)
}

type AcceptInviteRequest struct {
	InviteeID string `json:"invitee_id"`
}

func (h *GrowthHandler) HandleAcceptInvite(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req AcceptInviteRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	if req.InviteeID == "" {
		http.Error(w, "Missing invitee_id", http.StatusBadRequest)
		return
	}

	ctx := r.Context()
	h.ViralLoopTracker.RecordInviteAccepted(ctx, req.InviteeID)

	w.WriteHeader(http.StatusOK)
}
