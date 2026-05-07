package growth

import (
	"encoding/json"
	"net/http"
	"sync"
)

type Referral struct {
	ID          string `json:"id"`
	Clicks      int    `json:"clicks"`
	Conversions int    `json:"conversions"`
}

type TeamInvite struct {
	ID     string `json:"id"`
	Status string `json:"status"`
}

type GrowthService struct {
	referrals   map[string]*Referral
	teamInvites map[string]*TeamInvite
	mu          sync.Mutex
}

func NewGrowthService() *GrowthService {
	return &GrowthService{
		referrals:   make(map[string]*Referral),
		teamInvites: make(map[string]*TeamInvite),
	}
}

type IDRequest struct {
	ID string `json:"id"`
}

func (s *GrowthService) HandleReferralClick(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req IDRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	s.mu.Lock()
	defer s.mu.Unlock()

	ref, ok := s.referrals[req.ID]
	if !ok {
		ref = &Referral{ID: req.ID}
		s.referrals[req.ID] = ref
	}
	ref.Clicks++

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(ref)
}

func (s *GrowthService) HandleReferralConvert(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req IDRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	s.mu.Lock()
	defer s.mu.Unlock()

	ref, ok := s.referrals[req.ID]
	if !ok {
		ref = &Referral{ID: req.ID}
		s.referrals[req.ID] = ref
	}
	ref.Conversions++

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(ref)
}

func (s *GrowthService) HandleTeamInviteAccept(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req IDRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	s.mu.Lock()
	defer s.mu.Unlock()

	inv, ok := s.teamInvites[req.ID]
	if !ok {
		inv = &TeamInvite{ID: req.ID, Status: "PENDING"}
		s.teamInvites[req.ID] = inv
	}
	inv.Status = "ACCEPTED"

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(inv)
}
