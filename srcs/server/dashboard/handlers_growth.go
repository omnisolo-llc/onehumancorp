package dashboard

import (
	"encoding/json"
	"net/http"
	"time"
)

// LandingPageExperiment defines a growth experiment for the OHC platform.
type LandingPageExperiment struct {
	ID           string    `json:"id"`
	Title        string    `json:"title"`
	TrafficSplit float64   `json:"trafficSplit"`
	Status       string    `json:"status"`
	CreatedAt    time.Time `json:"createdAt"`
}

type experimentCreateRequest struct {
	Title        string  `json:"title"`
	TrafficSplit float64 `json:"trafficSplit"`
}

// Referral defines a viral referral link.
type Referral struct {
	ID           string    `json:"id"`
	UserID       string    `json:"userId"`
	ReferralCode string    `json:"referralCode"`
	Clicks       int       `json:"clicks"`
	Conversions  int       `json:"conversions"`
	CreatedAt    time.Time `json:"createdAt"`
}

type referralCreateRequest struct {
	UserID       string `json:"userId"`
	ReferralCode string `json:"referralCode"`
}

func (s *Server) handleLandingPageExperiments(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		s.mu.RLock()
		exps := append([]LandingPageExperiment(nil), s.experiments...)
		s.mu.RUnlock()
		writeJSON(w, exps)
	case http.MethodPost:
		var req experimentCreateRequest
		if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}
		if req.Title == "" {
			http.Error(w, "title is required", http.StatusBadRequest)
			return
		}
		exp := LandingPageExperiment{
			ID:           "exp-" + time.Now().UTC().Format("20060102150405"),
			Title:        req.Title,
			TrafficSplit: req.TrafficSplit,
			Status:       "ACTIVE",
			CreatedAt:    time.Now().UTC(),
		}
		s.mu.Lock()
		s.experiments = append(s.experiments, exp)
		s.mu.Unlock()
		writeJSON(w, exp)
	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

func (s *Server) handleReferrals(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		s.mu.RLock()
		refs := append([]Referral(nil), s.referrals...)
		s.mu.RUnlock()
		writeJSON(w, refs)
	case http.MethodPost:
		var req referralCreateRequest
		if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}
		if req.UserID == "" || req.ReferralCode == "" {
			http.Error(w, "userId and referralCode are required", http.StatusBadRequest)
			return
		}
		ref := Referral{
			ID:           "ref-" + time.Now().UTC().Format("20060102150405"),
			UserID:       req.UserID,
			ReferralCode: req.ReferralCode,
			Clicks:       0,
			Conversions:  0,
			CreatedAt:    time.Now().UTC(),
		}
		s.mu.Lock()
		s.referrals = append(s.referrals, ref)
		s.mu.Unlock()
		writeJSON(w, ref)
	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

// Download defines a tracked desktop app download.
type Download struct {
	ID        string    `json:"id"`
	OS        string    `json:"os"`
	Version   string    `json:"version"`
	CreatedAt time.Time `json:"createdAt"`
}

type downloadCreateRequest struct {
	OS      string `json:"os"`
	Version string `json:"version"`
}

// ViralCoefficientResponse represents the computed K-factor for growth.
type ViralCoefficientResponse struct {
	TotalReferrals   int     `json:"totalReferrals"`
	TotalConversions int     `json:"totalConversions"`
	UniqueInviters   int     `json:"uniqueInviters"`
	KFactor          float64 `json:"kFactor"` // conversions per unique inviter
}

func (s *Server) handleDownloads(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		s.mu.RLock()
		dl := append([]Download(nil), s.downloads...)
		s.mu.RUnlock()
		writeJSON(w, dl)
	case http.MethodPost:
		var req downloadCreateRequest
		if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}
		if req.OS == "" {
			http.Error(w, "os is required", http.StatusBadRequest)
			return
		}
		d := Download{
			ID:        "dl-" + time.Now().UTC().Format("20060102150405"),
			OS:        req.OS,
			Version:   req.Version,
			CreatedAt: time.Now().UTC(),
		}
		s.mu.Lock()
		s.downloads = append(s.downloads, d)
		s.mu.Unlock()
		writeJSON(w, d)
	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

func (s *Server) handleViralCoefficient(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	s.mu.RLock()
	refs := append([]Referral(nil), s.referrals...)
	s.mu.RUnlock()

	var totalConversions int
	var totalReferrals = len(refs)
	inviters := make(map[string]bool)

	for _, ref := range refs {
		totalConversions += ref.Conversions
		inviters[ref.UserID] = true
	}

	uniqueInviters := len(inviters)
	kFactor := 0.0
	if uniqueInviters > 0 {
		kFactor = float64(totalConversions) / float64(uniqueInviters)
	}

	res := ViralCoefficientResponse{
		TotalReferrals:   totalReferrals,
		TotalConversions: totalConversions,
		UniqueInviters:   uniqueInviters,
		KFactor:          kFactor,
	}
	writeJSON(w, res)
}

// TeamInvite represents an invitation sent to a prospective team member.
type TeamInvite struct {
	ID        string    `json:"id"`
	Email     string    `json:"email"`
	InviterID string    `json:"inviterId"`
	Status    string    `json:"status"`
	CreatedAt time.Time `json:"createdAt"`
}

type teamInviteCreateRequest struct {
	Email     string `json:"email"`
	InviterID string `json:"inviterId"`
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
		if req.Email == "" || req.InviterID == "" {
			http.Error(w, "email and inviterId are required", http.StatusBadRequest)
			return
		}
		inv := TeamInvite{
			ID:        "inv-" + time.Now().UTC().Format("20060102150405"),
			Email:     req.Email,
			InviterID: req.InviterID,
			Status:    "PENDING",
			CreatedAt: time.Now().UTC(),
		}
		s.mu.Lock()
		s.teamInvites = append(s.teamInvites, inv)
		s.mu.Unlock()
		writeJSON(w, inv)
	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}
