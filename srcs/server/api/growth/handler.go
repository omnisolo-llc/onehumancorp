package growth

import (
	"encoding/json"
	"net/http"

	"github.com/onehumancorp/mono/lib/analytics"
	"github.com/onehumancorp/mono/services/growth"
)

type GrowthHandler struct {
	landingService *growth.LandingService
	teamService    *growth.TeamService
}

func NewGrowthHandler(tracker *analytics.Tracker) *GrowthHandler {
	return &GrowthHandler{
		landingService: growth.NewLandingService(tracker),
		teamService:    growth.NewTeamService(tracker),
	}
}

type TrackVisitRequest struct {
	PageID    string `json:"page_id"`
	VisitorID string `json:"visitor_id"`
}

func (h *GrowthHandler) HandleTrackVisit(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req TrackVisitRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}
	if err := h.landingService.TrackVisit(r.Context(), req.PageID, req.VisitorID); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	w.WriteHeader(http.StatusOK)
}

type TrackConversionRequest struct {
	PageID    string `json:"page_id"`
	VisitorID string `json:"visitor_id"`
}

func (h *GrowthHandler) HandleTrackConversion(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req TrackConversionRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}
	if err := h.landingService.TrackConversion(r.Context(), req.PageID, req.VisitorID); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	w.WriteHeader(http.StatusOK)
}

type InviteTeamRequest struct {
	TeamID       string `json:"team_id"`
	InviterID    string `json:"inviter_id"`
	InviteeEmail string `json:"invitee_email"`
}

func (h *GrowthHandler) HandleInviteTeam(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req InviteTeamRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}
	if err := h.teamService.InviteToTeam(r.Context(), req.TeamID, req.InviterID, req.InviteeEmail); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	w.WriteHeader(http.StatusOK)
}

type AcceptInviteRequest struct {
	InviteID  string `json:"invite_id"`
	InviteeID string `json:"invitee_id"`
}

func (h *GrowthHandler) HandleAcceptInvite(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req AcceptInviteRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}
	if err := h.teamService.AcceptTeamInvite(r.Context(), req.InviteID, req.InviteeID); err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}
	w.WriteHeader(http.StatusOK)
}
