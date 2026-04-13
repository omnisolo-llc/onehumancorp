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

// TeamInvite defines a viral team invite loop.
type TeamInvite struct {
	ID        string    `json:"id"`
	InviterID string    `json:"inviterId"`
	InviteeID string    `json:"inviteeId"`
	Status    string    `json:"status"` // PENDING, ACCEPTED
	CreatedAt time.Time `json:"createdAt"`
}

type teamInviteCreateRequest struct {
	InviterID string `json:"inviterId"`
	InviteeID string `json:"inviteeId"`
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
		if req.InviterID == "" || req.InviteeID == "" {
			http.Error(w, "inviterId and inviteeId are required", http.StatusBadRequest)
			return
		}
		invite := TeamInvite{
			ID:        "inv-" + time.Now().UTC().Format("20060102150405"),
			InviterID: req.InviterID,
			InviteeID: req.InviteeID,
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

// OnboardingFunnel defines a tracked onboarding funnel drop-off.
type OnboardingFunnel struct {
	ID        string    `json:"id"`
	UserID    string    `json:"userId"`
	Step      string    `json:"step"`
	CreatedAt time.Time `json:"createdAt"`
}

type onboardingCreateRequest struct {
	UserID string `json:"userId"`
	Step   string `json:"step"`
}

func (s *Server) handleOnboardingFunnel(w http.ResponseWriter, r *http.Request) {
	switch r.Method {
	case http.MethodGet:
		s.mu.RLock()
		funnels := append([]OnboardingFunnel(nil), s.onboardingFunnels...)
		s.mu.RUnlock()
		writeJSON(w, funnels)
	case http.MethodPost:
		var req onboardingCreateRequest
		if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
			http.Error(w, "invalid JSON payload", http.StatusBadRequest)
			return
		}
		if req.UserID == "" || req.Step == "" {
			http.Error(w, "userId and step are required", http.StatusBadRequest)
			return
		}
		funnel := OnboardingFunnel{
			ID:        "funnel-" + time.Now().UTC().Format("20060102150405"),
			UserID:    req.UserID,
			Step:      req.Step,
			CreatedAt: time.Now().UTC(),
		}
		s.mu.Lock()
		s.onboardingFunnels = append(s.onboardingFunnels, funnel)
		s.mu.Unlock()
		writeJSON(w, funnel)
	default:
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
	}
}

// OnboardingMetric represents the aggregated count of users at a specific onboarding step.
type OnboardingMetric struct {
	Step  string `json:"step"`
	Count int    `json:"count"`
}

func (s *Server) handleOnboardingMetrics(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	s.mu.RLock()
	funnels := append([]OnboardingFunnel(nil), s.onboardingFunnels...)
	s.mu.RUnlock()

	counts := make(map[string]int)
	for _, f := range funnels {
		counts[f.Step]++
	}

	var metrics []OnboardingMetric
	for step, count := range counts {
		metrics = append(metrics, OnboardingMetric{
			Step:  step,
			Count: count,
		})
	}

	writeJSON(w, metrics)
}

type growthIdRequest struct {
	ID string `json:"id"`
}

func (s *Server) handleTeamInviteAccept(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req growthIdRequest
	if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}
	if req.ID == "" {
		http.Error(w, "id is required", http.StatusBadRequest)
		return
	}

	s.mu.Lock()
	defer s.mu.Unlock()

	found := false
	var updated TeamInvite
	for i, inv := range s.teamInvites {
		if inv.ID == req.ID {
			s.teamInvites[i].Status = "ACCEPTED"
			updated = s.teamInvites[i]
			found = true
			break
		}
	}

	if !found {
		http.Error(w, "invite not found", http.StatusNotFound)
		return
	}
	writeJSON(w, updated)
}

func (s *Server) handleReferralClick(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req growthIdRequest
	if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}
	if req.ID == "" {
		http.Error(w, "id is required", http.StatusBadRequest)
		return
	}

	s.mu.Lock()
	defer s.mu.Unlock()

	found := false
	var updated Referral
	for i, ref := range s.referrals {
		if ref.ID == req.ID {
			s.referrals[i].Clicks++
			updated = s.referrals[i]
			found = true
			break
		}
	}

	if !found {
		http.Error(w, "referral not found", http.StatusNotFound)
		return
	}
	writeJSON(w, updated)
}

func (s *Server) handleReferralConvert(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}
	var req growthIdRequest
	if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
		http.Error(w, "invalid JSON payload", http.StatusBadRequest)
		return
	}
	if req.ID == "" {
		http.Error(w, "id is required", http.StatusBadRequest)
		return
	}

	s.mu.Lock()
	defer s.mu.Unlock()

	found := false
	var updated Referral
	for i, ref := range s.referrals {
		if ref.ID == req.ID {
			s.referrals[i].Conversions++
			updated = s.referrals[i]
			found = true
			break
		}
	}

	if !found {
		http.Error(w, "referral not found", http.StatusNotFound)
		return
	}
	writeJSON(w, updated)
}

type WaitlistEntry struct {
    ID        string    `json:"id"`
    Email     string    `json:"email"`
    CreatedAt time.Time `json:"createdAt"`
}

type waitlistCreateRequest struct {
    Email string `json:"email"`
}

func (s *Server) handleWaitlist(w http.ResponseWriter, r *http.Request) {
    switch r.Method {
    case http.MethodGet:
        s.mu.RLock()
        wl := append([]WaitlistEntry(nil), s.waitlist...)
        s.mu.RUnlock()
        writeJSON(w, wl)
    case http.MethodPost:
        var req waitlistCreateRequest
        if err := json.NewDecoder(http.MaxBytesReader(w, r.Body, 1<<20)).Decode(&req); err != nil {
            http.Error(w, "invalid JSON payload", http.StatusBadRequest)
            return
        }
        if req.Email == "" {
            http.Error(w, "email is required", http.StatusBadRequest)
            return
        }
        entry := WaitlistEntry{
            ID:        "wl-" + time.Now().UTC().Format("20060102150405"),
            Email:     req.Email,
            CreatedAt: time.Now().UTC(),
        }
        s.mu.Lock()
        s.waitlist = append(s.waitlist, entry)
        s.mu.Unlock()
        writeJSON(w, entry)
    default:
        http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
    }
}
