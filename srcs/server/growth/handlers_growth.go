package growth

import (
	"database/sql"
	"encoding/json"
	"net/http"
	"onehumancorp/srcs/server/onboarding"
	"log"
)

type Referral struct {
	ID          string `json:"id"`
	TenantID    string `json:"tenant_id"`
	Clicks      int    `json:"clicks"`
	Conversions int    `json:"conversions"`
}

type TeamInvite struct {
	ID       string `json:"id"`
	TenantID string `json:"tenant_id"`
	Status   string `json:"status"`
}

type GrowthService struct {
	db *sql.DB
}

func NewGrowthService(db *sql.DB) *GrowthService {
	_, err := db.Exec(`
		CREATE TABLE IF NOT EXISTS referrals (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			clicks INTEGER DEFAULT 0,
			conversions INTEGER DEFAULT 0
		);
		CREATE TABLE IF NOT EXISTS team_invites (
			id TEXT PRIMARY KEY,
			tenant_id TEXT NOT NULL,
			status TEXT DEFAULT 'PENDING'
		);
	`)
	if err != nil {
		log.Fatalf("Failed to initialize growth tables: %v", err)
	}
	db.Exec(`
		ALTER TABLE referrals ENABLE ROW LEVEL SECURITY;
		CREATE POLICY tenant_isolation_referrals ON referrals USING (tenant_id = current_setting('app.current_tenant', true));
	`)
	db.Exec(`
		ALTER TABLE team_invites ENABLE ROW LEVEL SECURITY;
		CREATE POLICY tenant_isolation_team_invites ON team_invites USING (tenant_id = current_setting('app.current_tenant', true));
	`)
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

	tenantID, ok := r.Context().Value(onboarding.TenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized: missing or invalid tenant session", http.StatusUnauthorized)
		return
	}

	var ref Referral
	err := s.db.QueryRow(`
		INSERT INTO referrals (id, tenant_id, clicks, conversions)
		VALUES ($1, $2, 1, 0)
		ON CONFLICT(id) DO UPDATE SET clicks = referrals.clicks + 1 WHERE referrals.tenant_id = $2
		RETURNING id, tenant_id, clicks, conversions
	`, req.ID, tenantID).Scan(&ref.ID, &ref.TenantID, &ref.Clicks, &ref.Conversions)

	if err == sql.ErrNoRows {
		http.Error(w, "Not found or unauthorized", http.StatusNotFound)
		return
	} else if err != nil {
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

	tenantID, ok := r.Context().Value(onboarding.TenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized: missing or invalid tenant session", http.StatusUnauthorized)
		return
	}

	var ref Referral
	err := s.db.QueryRow(`
		INSERT INTO referrals (id, tenant_id, clicks, conversions)
		VALUES ($1, $2, 0, 1)
		ON CONFLICT(id) DO UPDATE SET conversions = referrals.conversions + 1 WHERE referrals.tenant_id = $2
		RETURNING id, tenant_id, clicks, conversions
	`, req.ID, tenantID).Scan(&ref.ID, &ref.TenantID, &ref.Clicks, &ref.Conversions)

	if err == sql.ErrNoRows {
		http.Error(w, "Not found or unauthorized", http.StatusNotFound)
		return
	} else if err != nil {
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

	tenantID, ok := r.Context().Value(onboarding.TenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized: missing or invalid tenant session", http.StatusUnauthorized)
		return
	}

	var inv TeamInvite
	err := s.db.QueryRow(`
		INSERT INTO team_invites (id, tenant_id, status)
		VALUES ($1, $2, 'ACCEPTED')
		ON CONFLICT(id) DO UPDATE SET status = 'ACCEPTED' WHERE team_invites.tenant_id = $2
		RETURNING id, tenant_id, status
	`, req.ID, tenantID).Scan(&inv.ID, &inv.TenantID, &inv.Status)

	if err == sql.ErrNoRows {
		http.Error(w, "Not found or unauthorized", http.StatusNotFound)
		return
	} else if err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(inv)
}
