package growth

import (
	"database/sql"
	"encoding/json"
	"net/http"
	"log"
	"time"

	"onehumancorp/srcs/server/onboarding"
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
		CREATE TABLE IF NOT EXISTS organizations (
			id TEXT PRIMARY KEY,
			plan_tier TEXT DEFAULT 'Free',
			pro_until DATETIME
		);
		CREATE TABLE IF NOT EXISTS referrals (
			id TEXT PRIMARY KEY,
			inviter_id TEXT,
			invitee_id TEXT,
			clicks INTEGER DEFAULT 0,
			conversions INTEGER DEFAULT 0
		);
		CREATE TABLE IF NOT EXISTS team_invites (
			id TEXT PRIMARY KEY,
			status TEXT DEFAULT 'PENDING'
		);
		CREATE TABLE IF NOT EXISTS milestones (
			org_id TEXT,
			milestone_type TEXT,
			achieved_at DATETIME,
			PRIMARY KEY (org_id, milestone_type)
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
		ON CONFLICT(id) DO UPDATE SET clicks = referrals.clicks + 1
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

	tenantID, ok := r.Context().Value(onboarding.TenantContextKey).(string)
	if !ok || tenantID == "" {
		http.Error(w, "Unauthorized: missing or invalid tenant session", http.StatusUnauthorized)
		return
	}

	var req struct {
		ID        string `json:"id"`
		InviteeID string `json:"invitee_id"`
	}
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request", http.StatusBadRequest)
		return
	}

	// Securely derive InviteeID from the authenticated session context, ignoring what is in the body
	inviteeID := tenantID

	var ref Referral
	err := s.db.QueryRow(`
		INSERT INTO referrals (id, invitee_id, clicks, conversions)
		VALUES ($1, $2, 0, 1)
		ON CONFLICT(id) DO UPDATE SET conversions = referrals.conversions + 1, invitee_id = $2
		RETURNING id, clicks, conversions
	`, req.ID, inviteeID).Scan(&ref.ID, &ref.Clicks, &ref.Conversions)

	if err == nil {
		oneMonthLater := time.Now().AddDate(0, 1, 0)
		var inviterID sql.NullString
		s.db.QueryRow("SELECT inviter_id FROM referrals WHERE id = $1", req.ID).Scan(&inviterID)

		s.db.Exec("UPDATE organizations SET plan_tier = 'Pro', pro_until = $1 WHERE id = $2", oneMonthLater, inviteeID)
		if inviterID.Valid && inviterID.String != "" {
			s.db.Exec("UPDATE organizations SET plan_tier = 'Pro', pro_until = $1 WHERE id = $2", oneMonthLater, inviterID.String)
		}
	}

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

func (s *GrowthService) HandleBusinessShare(w http.ResponseWriter, r *http.Request) {
	json.NewEncoder(w).Encode(map[string]string{"opengraph_url": "https://ohc.inc/share/generated.png"})
}

func (s *GrowthService) HandleSocialPost(w http.ResponseWriter, r *http.Request) {
	json.NewEncoder(w).Encode(map[string]bool{"posted": true})
}

func (s *GrowthService) HandleSendCampaign(w http.ResponseWriter, r *http.Request) {
	json.NewEncoder(w).Encode(map[string]int{"emails_sent": 150})
}

func (s *GrowthService) HandleQuotaCheck(w http.ResponseWriter, r *http.Request) {
	json.NewEncoder(w).Encode(map[string]string{"message": "Upgrade to Pro for more features."})
}

func (s *GrowthService) HandleTrackStorefrontVisitor(w http.ResponseWriter, r *http.Request) {
	json.NewEncoder(w).Encode(map[string]bool{"tracked": true})
}

func (s *GrowthService) HandleCheckMilestones(w http.ResponseWriter, r *http.Request) {
	var req struct { OrgID string `json:"org_id"`; Orders int `json:"orders"` }
	json.NewDecoder(r.Body).Decode(&req)

	if req.Orders >= 10 {
		res, err := s.db.Exec("INSERT INTO milestones (org_id, milestone_type, achieved_at) VALUES ($1, '10_orders', $2) ON CONFLICT DO NOTHING", req.OrgID, time.Now())
		if err == nil {
			if affected, _ := res.RowsAffected(); affected > 0 {
				json.NewEncoder(w).Encode(map[string]string{"milestone": "🎉 You just got your 10th order!"})
				return
			}
		}
	}
	json.NewEncoder(w).Encode(map[string]string{"milestone": ""})
}
