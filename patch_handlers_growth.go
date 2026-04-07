package dashboard

import (
	"encoding/json"
	"net/http"
	"time"
)

// TeamInvite represents an invitation to join a team.
type TeamInvite struct {
	ID        string    `json:"id"`
	InviterID string    `json:"inviterId"`
	Email     string    `json:"email"`
	Status    string    `json:"status"` // PENDING, ACCEPTED
	CreatedAt time.Time `json:"createdAt"`
}

type teamInviteCreateRequest struct {
	InviterID string `json:"inviterId"`
	Email     string `json:"email"`
}

// FreeTierQuota represents the usage limits for a free tier user.
type FreeTierQuota struct {
	UserID        string `json:"userId"`
	AgentMissions int    `json:"agentMissions"`
	MaxMissions   int    `json:"maxMissions"`
	Status        string `json:"status"` // ACTIVE, EXHAUSTED
}

func (s *Server) handleTeamInvites(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		s.mu.RLock()
		invites := append([]TeamInvite(nil), s.teamInvites...)
		s.mu.RUnlock()
		writeJSON(w, invites)
	case http.MethodPost:
		var req teamInviteCreateRequest
		if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}
		if req.InviterID == "" || req.Email == "" {
			http.Error(w, "inviterId and email are required", http.StatusBadRequest)
			return
		}
		invite := TeamInvite{
			ID:        "invite-" + time.Now().UTC().Format("20060102150405"),
			InviterID: req.InviterID,
			Email:     req.Email,
			Status:    "PENDING",
			CreatedAt: time.Now().UTC(),
		}
		s.mu.Lock()
		s.teamInvites = append(s.teamInvites, invite)
		s.mu.Unlock()
		writeJSON(w, invite)
	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

func (s *Server) handleFreeTierQuotas(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		userID := r.URL.Query().Get("userId")
		if userID == "" {
			http.Error(w, "userId is required", http.StatusBadRequest)
			return
		}

		s.mu.RLock()
		var quota *FreeTierQuota
		for _, q := range s.quotas {
			if q.UserID == userID {
				quota = &FreeTierQuota{
					UserID:        q.UserID,
					AgentMissions: q.AgentMissions,
					MaxMissions:   q.MaxMissions,
					Status:        q.Status,
				}
				break
			}
		}
		s.mu.RUnlock()

		if quota == nil {
			// Initialize default free tier quota
			quota = &FreeTierQuota{
				UserID:        userID,
				AgentMissions: 0,
				MaxMissions:   5, // 5 free missions default
				Status:        "ACTIVE",
			}
			s.mu.Lock()
			s.quotas = append(s.quotas, *quota)
			s.mu.Unlock()
		}

		writeJSON(w, quota)

	case http.MethodPost:
		// Increment quota usage
		userID := r.URL.Query().Get("userId")
		if userID == "" {
			http.Error(w, "userId is required", http.StatusBadRequest)
			return
		}

		s.mu.Lock()
		defer s.mu.Unlock()

		var found bool
		var currentQuota FreeTierQuota
		for i, q := range s.quotas {
			if q.UserID == userID {
				found = true
				s.quotas[i].AgentMissions++
				if s.quotas[i].AgentMissions >= s.quotas[i].MaxMissions {
					s.quotas[i].Status = "EXHAUSTED"
				}
				currentQuota = s.quotas[i]
				break
			}
		}

		if !found {
			currentQuota = FreeTierQuota{
				UserID:        userID,
				AgentMissions: 1,
				MaxMissions:   5,
				Status:        "ACTIVE",
			}
			s.quotas = append(s.quotas, currentQuota)
		}

		writeJSON(w, currentQuota)

	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}
