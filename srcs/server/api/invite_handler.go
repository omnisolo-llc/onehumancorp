package api

import (
	"encoding/json"
	"net/http"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/domain"
	"go.opentelemetry.io/otel"
)

type InviteHandler struct {
	svc *domain.InviteService
}

func NewInviteHandler(svc *domain.InviteService) *InviteHandler {
	return &InviteHandler{svc: svc}
}

type createInviteRequest struct {
	InviterID string `json:"inviterId"`
	InviteeID string `json:"inviteeId"`
}

func (h *InviteHandler) HandleCreateInvite(w http.ResponseWriter, r *http.Request) {
	ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/api").Start(r.Context(), "HandleCreateInvite")
	defer span.End()

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req createInviteRequest
	if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}

	invite, err := h.svc.CreateInvite(ctx, req.InviterID, req.InviteeID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusCreated)
	json.NewEncoder(w).Encode(invite)
}

func (h *InviteHandler) HandleAcceptInvite(w http.ResponseWriter, r *http.Request) {
	ctx, span := otel.Tracer("github.com/onehumancorp/mono/srcs/server/api").Start(r.Context(), "HandleAcceptInvite")
	defer span.End()

	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	// Extract ID from path: /api/invites/{id}/accept
	pathParts := strings.Split(r.URL.Path, "/")
	if len(pathParts) < 5 || pathParts[4] != "accept" {
		http.Error(w, "invalid path", http.StatusBadRequest)
		return
	}
	id := pathParts[3]
	if id == "" {
		http.Error(w, "id is required", http.StatusBadRequest)
		return
	}

	invite, err := h.svc.AcceptInvite(ctx, id)
	if err != nil {
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(invite)
}
