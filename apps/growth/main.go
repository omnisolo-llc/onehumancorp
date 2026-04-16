package main

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/onehumancorp/mono/lib/analytics"
	"github.com/onehumancorp/mono/services/growth"
	"github.com/redis/go-redis/v9"
)

type ReferralRequest struct {
	SenderID      string `json:"sender_id"`
	ReceiverEmail string `json:"receiver_email"`
}

type QuotaRequest struct {
	TenantID string `json:"tenant_id"`
}

func authMiddleware(next http.HandlerFunc) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		// SPIFFE/SPIRE check.
		// Note: The actual deployment environment uses a sidecar proxy (like Envoy or internal K8s ingress)
		// that terminates the mTLS SPIFFE connection and forwards the validated SPIFFE ID via a trusted header
		// (e.g., X-Forwarded-Client-Cert or X-Spiffe-Id).
		// We validate the presence of this trusted header instead of raw TLS termination here,
		// allowing the Go service to run as a standard HTTP listener internally.
		spiffeID := r.Header.Get("X-Spiffe-Id")
		if spiffeID == "" && r.TLS == nil {
			http.Error(w, "missing SPIFFE identity", http.StatusUnauthorized)
			return
		}

		next(w, r)
	}
}

func NewGrowthMux(tracker *analytics.Tracker, rdb *redis.Client) *http.ServeMux {
	mux := http.NewServeMux()

	quotaService := growth.NewQuotaService(tracker, rdb, 100)
	referralService := growth.NewReferralService(tracker)
	referralsRepo := growth.NewReferralRepository(rdb)

	mux.HandleFunc("/growth/referral", authMiddleware(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}
		var req ReferralRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Invalid request body", http.StatusBadRequest)
			return
		}

		if req.SenderID == "" || req.ReceiverEmail == "" {
			http.Error(w, "missing sender_id or receiver_email", http.StatusBadRequest)
			return
		}

		err := referralService.ProcessInvite(context.Background(), req.SenderID, req.ReceiverEmail)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}
		w.WriteHeader(http.StatusOK)
		fmt.Fprintf(w, "Invite sent successfully\n")
	}))

	mux.HandleFunc("/api/v1/growth/referrals/invite", authMiddleware(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		spiffeID := r.Header.Get("X-Spiffe-Id")
		if spiffeID == "" {
			http.Error(w, "missing SPIFFE identity", http.StatusUnauthorized)
			return
		}

		var req struct {
			InviteeEmail string `json:"invitee_email"`
		}
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Invalid request body", http.StatusBadRequest)
			return
		}

		if req.InviteeEmail == "" {
			http.Error(w, "missing invitee_email", http.StatusBadRequest)
			return
		}

		referral := &growth.GrowthReferral{
			InviterID:    spiffeID,
			InviteeEmail: req.InviteeEmail,
			Status:       "PENDING",
			CreatedAt:    time.Now(),
		}
		referral.ID = fmt.Sprintf("ref-%d", time.Now().UnixNano())

		err := referralsRepo.SaveReferral(context.Background(), referral)
		if err != nil {
			http.Error(w, "Failed to save referral", http.StatusInternalServerError)
			return
		}

		// Also track the event
		_ = referralService.ProcessInvite(context.Background(), spiffeID, req.InviteeEmail)

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(map[string]string{
			"message": "Invite sent successfully",
			"link":    fmt.Sprintf("ohc://join?ref=%s", referral.ID),
		})
	}))


	mux.HandleFunc("/api/v1/growth/referrals/accept", authMiddleware(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		spiffeID := r.Header.Get("X-Spiffe-Id")
		if spiffeID == "" {
			http.Error(w, "missing SPIFFE identity", http.StatusUnauthorized)
			return
		}

		var req struct {
			InviteID string `json:"invite_id"`
		}
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Invalid request body", http.StatusBadRequest)
			return
		}

		if req.InviteID == "" {
			http.Error(w, "missing invite_id", http.StatusBadRequest)
			return
		}

		err := referralsRepo.AcceptReferral(context.Background(), req.InviteID)
		if err != nil {
			http.Error(w, "Failed to accept referral", http.StatusBadRequest)
			return
		}

		err = referralService.AcceptInvite(context.Background(), req.InviteID)
		if err != nil {
			http.Error(w, "Failed to track invite acceptance", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(map[string]string{
			"message": "Referral accepted successfully",
		})
	}))

	mux.HandleFunc("/api/v1/growth/referrals/stats", authMiddleware(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodGet {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		spiffeID := r.Header.Get("X-Spiffe-Id")
		if spiffeID == "" {
			http.Error(w, "missing SPIFFE identity", http.StatusUnauthorized)
			return
		}

		stats, err := referralsRepo.GetStats(context.Background(), spiffeID)
		if err != nil {
			http.Error(w, "Failed to get stats", http.StatusInternalServerError)
			return
		}

		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusOK)
		json.NewEncoder(w).Encode(stats)
	}))

	mux.HandleFunc("/growth/quota/check", authMiddleware(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		var req QuotaRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Invalid request body", http.StatusBadRequest)
			return
		}
		if req.TenantID == "" {
			http.Error(w, "missing tenant_id", http.StatusBadRequest)
			return
		}

		allowed, err := quotaService.CheckQuota(context.Background(), req.TenantID)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
		fmt.Fprintf(w, "Quota allowed: %v\n", allowed)
	}))

	mux.HandleFunc("/growth/quota/increment", authMiddleware(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}
		var req QuotaRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Invalid request body", http.StatusBadRequest)
			return
		}

		if req.TenantID == "" {
			http.Error(w, "missing tenant_id", http.StatusBadRequest)
			return
		}

		err := quotaService.IncrementUsage(context.Background(), req.TenantID)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
			return
		}

		w.WriteHeader(http.StatusOK)
		fmt.Fprintf(w, "Usage incremented successfully\n")
	}))
	return mux
}

func main() {
	fmt.Println("Starting OHC Growth Loop Service...")

	tracker := analytics.NewTracker()

	var rdb *redis.Client
	redisAddr := os.Getenv("REDIS_ADDR")
	if redisAddr != "" {
		rdb = redis.NewClient(&redis.Options{
			Addr: redisAddr,
		})
	}

	mux := NewGrowthMux(tracker, rdb)

	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	log.Printf("Listening on port %s", port)
	if err := http.ListenAndServe(":"+port, mux); err != nil {
		log.Fatalf("Server failed: %v", err)
	}
}
