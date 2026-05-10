package growth

import (
	"database/sql"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"strings"
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
			id TEXT,
			tenant_id TEXT NOT NULL DEFAULT 'default',
			clicks INTEGER DEFAULT 0,
			conversions INTEGER DEFAULT 0,
			PRIMARY KEY(id, tenant_id)
		);
		CREATE TABLE IF NOT EXISTS team_invites (
			id TEXT,
			tenant_id TEXT NOT NULL DEFAULT 'default',
			status TEXT DEFAULT 'PENDING',
			PRIMARY KEY(id, tenant_id)
		);
	`)
	if err != nil {
		log.Fatalf("Failed to initialize growth tables: %v", err)
	}

	// Apply RLS and Policies only for PostgreSQL to prevent default-deny isolation failures
	if !strings.Contains(fmt.Sprintf("%T", db.Driver()), "sqlite") {
		_, err = db.Exec(`
			ALTER TABLE referrals ENABLE ROW LEVEL SECURITY;
			ALTER TABLE team_invites ENABLE ROW LEVEL SECURITY;

			DROP POLICY IF EXISTS referrals_tenant_isolation ON referrals;
			CREATE POLICY referrals_tenant_isolation ON referrals
			    USING (tenant_id = current_setting('app.current_tenant', true));

			DROP POLICY IF EXISTS team_invites_tenant_isolation ON team_invites;
			CREATE POLICY team_invites_tenant_isolation ON team_invites
			    USING (tenant_id = current_setting('app.current_tenant', true));
		`)
		if err != nil {
			log.Printf("Warning: Failed to enable RLS policies on growth tables: %v", err)
		}
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

	tenantID, _ := r.Context().Value("tenant_id").(string)
	if tenantID == "" {
		tenantID = "default"
	}

	var req IDRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	var ref Referral
	tx, err := s.db.Begin()
	if err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}
	defer tx.Rollback()

	if !strings.Contains(fmt.Sprintf("%T", s.db.Driver()), "sqlite") {
		_, err = tx.Exec(fmt.Sprintf("SET LOCAL app.current_tenant = '%s'", tenantID))
		if err != nil {
			http.Error(w, "Database error setting tenant", http.StatusInternalServerError)
			return
		}
	}

	err = tx.QueryRow(`
		INSERT INTO referrals (id, tenant_id, clicks, conversions)
		VALUES ($1, $2, 1, 0)
		ON CONFLICT(id, tenant_id) DO UPDATE SET clicks = referrals.clicks + 1
		RETURNING id, tenant_id, clicks, conversions
	`, req.ID, tenantID).Scan(&ref.ID, &ref.TenantID, &ref.Clicks, &ref.Conversions)

	if err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}

	if err := tx.Commit(); err != nil {
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

	tenantID, _ := r.Context().Value("tenant_id").(string)
	if tenantID == "" {
		tenantID = "default"
	}

	var req IDRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	var ref Referral
	tx, err := s.db.Begin()
	if err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}
	defer tx.Rollback()

	if !strings.Contains(fmt.Sprintf("%T", s.db.Driver()), "sqlite") {
		_, err = tx.Exec(fmt.Sprintf("SET LOCAL app.current_tenant = '%s'", tenantID))
		if err != nil {
			http.Error(w, "Database error setting tenant", http.StatusInternalServerError)
			return
		}
	}

	err = tx.QueryRow(`
		INSERT INTO referrals (id, tenant_id, clicks, conversions)
		VALUES ($1, $2, 0, 1)
		ON CONFLICT(id, tenant_id) DO UPDATE SET conversions = referrals.conversions + 1
		RETURNING id, tenant_id, clicks, conversions
	`, req.ID, tenantID).Scan(&ref.ID, &ref.TenantID, &ref.Clicks, &ref.Conversions)

	if err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}

	if err := tx.Commit(); err != nil {
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

	tenantID, _ := r.Context().Value("tenant_id").(string)
	if tenantID == "" {
		tenantID = "default"
	}

	var req IDRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	var inv TeamInvite
	tx, err := s.db.Begin()
	if err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}
	defer tx.Rollback()

	if !strings.Contains(fmt.Sprintf("%T", s.db.Driver()), "sqlite") {
		_, err = tx.Exec(fmt.Sprintf("SET LOCAL app.current_tenant = '%s'", tenantID))
		if err != nil {
			http.Error(w, "Database error setting tenant", http.StatusInternalServerError)
			return
		}
	}

	err = tx.QueryRow(`
		INSERT INTO team_invites (id, tenant_id, status)
		VALUES ($1, $2, 'ACCEPTED')
		ON CONFLICT(id, tenant_id) DO UPDATE SET status = 'ACCEPTED'
		RETURNING id, tenant_id, status
	`, req.ID, tenantID).Scan(&inv.ID, &inv.TenantID, &inv.Status)

	if err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}

	if err := tx.Commit(); err != nil {
		http.Error(w, "Database error", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(inv)
}
