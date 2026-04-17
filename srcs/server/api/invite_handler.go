package api

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/domain"
)

type InviteHandler struct {
	service *domain.InviteService
}

func NewInviteHandler(service *domain.InviteService) *InviteHandler {
	return &InviteHandler{
		service: service,
	}
}

type createInviteRequest struct {
	InviterID string `json:"inviterId"`
	InviteeID string `json:"inviteeId"`
}

func (h *InviteHandler) HandleInvites(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req createInviteRequest
	if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}

	invite, err := h.service.CreateInvite(r.Context(), req.InviterID, req.InviteeID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(invite)
}

func (h *InviteHandler) HandleInviteAccept(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	parts := strings.Split(r.URL.Path, "/")
	if len(parts) < 5 || parts[4] != "accept" {
		http.Error(w, "invalid path", http.StatusBadRequest)
		return
	}
	id := parts[3]

	invite, err := h.service.AcceptInvite(r.Context(), id)
	if err != nil {
		status := http.StatusBadRequest
		if err.Error() == "invite not found" {
			status = http.StatusNotFound
		}
		http.Error(w, err.Error(), status)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(invite)
}
