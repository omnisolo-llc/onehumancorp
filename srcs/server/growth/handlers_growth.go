package growth

import (
	"database/sql"
	"encoding/json"
	"net/http"
	"log"
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
	db *sql.DB
}

func NewGrowthService(db *sql.DB) *GrowthService {
	_, err := db.Exec(`
		CREATE TABLE IF NOT EXISTS referrals (
			id TEXT PRIMARY KEY,
			clicks INTEGER DEFAULT 0,
			conversions INTEGER DEFAULT 0
		);
		CREATE TABLE IF NOT EXISTS team_invites (
			id TEXT PRIMARY KEY,
			status TEXT DEFAULT 'PENDING'
		);
	`)
	if err != nil {
		log.Fatalf("Failed to initialize growth tables: %v", err)
	}
	return &GrowthService{
		db: db,
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

	var ref Referral
	err := s.db.QueryRow(`
		INSERT INTO referrals (id, clicks, conversions)
		VALUES ($1, 1, 0)
		ON CONFLICT(id) DO UPDATE SET clicks = clicks + 1
		RETURNING id, clicks, conversions
	`, req.ID).Scan(&ref.ID, &ref.Clicks, &ref.Conversions)

	if err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}

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

	var ref Referral
	err := s.db.QueryRow(`
		INSERT INTO referrals (id, clicks, conversions)
		VALUES ($1, 0, 1)
		ON CONFLICT(id) DO UPDATE SET conversions = conversions + 1
		RETURNING id, clicks, conversions
	`, req.ID).Scan(&ref.ID, &ref.Clicks, &ref.Conversions)

	if err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}

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

	var inv TeamInvite
	err := s.db.QueryRow(`
		INSERT INTO team_invites (id, status)
		VALUES ($1, 'ACCEPTED')
		ON CONFLICT(id) DO UPDATE SET status = 'ACCEPTED'
		RETURNING id, status
	`, req.ID).Scan(&inv.ID, &inv.Status)

	if err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(inv)
}
