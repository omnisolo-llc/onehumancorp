package growth

import (
	"github.com/onehumancorp/mono/srcs/server/db"

	"github.com/onehumancorp/mono/srcs/server/auth"

	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"net/http"
)

// GenerateReferralLink creates a deep link for sharing the standalone desktop mode.
func GenerateReferralLink(userID string) (string, error) {
	if userID == "" {
		return "", fmt.Errorf("userID cannot be empty")
	}

	bytes := make([]byte, 8)
	if _, err := rand.Read(bytes); err != nil {
		return "", err
	}
	referralCode := hex.EncodeToString(bytes)

	// Standalone mode specific deep link
	link := fmt.Sprintf("ohc://join?ref=%s&utm_source=standalone_desktop&utm_medium=team_share&inviter=%s", referralCode, userID)
	return link, nil
}

type ReferralRequest struct {
	UserID string `json:"user_id"`
}

type ReferralResponse struct {
	Link string `json:"link"`
}

func ReferralHandler(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var req ReferralRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "Invalid request body", http.StatusBadRequest)
		return
	}

	link, err := GenerateReferralLink(req.UserID)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

	resp := ReferralResponse{Link: link}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}

type ApplyReferralRequest struct {
	UserID       string `json:"user_id"`
	ReferralCode string `json:"referral_code"`
}

type ApplyReferralResponse struct {
	Success bool `json:"success"`
}

func ApplyReferralHandler(dbProvider db.Provider) http.HandlerFunc {
	return func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
			return
		}

		claims := auth.ClaimsFromContext(r.Context())
		if claims == nil {
			http.Error(w, "unauthorized", http.StatusUnauthorized)
			return
		}
		userID := claims.Subject

		var req ApplyReferralRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			http.Error(w, "Invalid request body", http.StatusBadRequest)
			return
		}

		req.UserID = userID

		if req.UserID == "" || req.ReferralCode == "" {
			http.Error(w, "user_id and referral_code are required", http.StatusBadRequest)
			return
		}

		ctx := r.Context()

		if dbProvider == nil {
			http.Error(w, "Database unavailable", http.StatusInternalServerError)
			return
		}
		tx, err := dbProvider.Begin(ctx)
		if err != nil {
			http.Error(w, "Failed to start transaction", http.StatusInternalServerError)
			return
		}
		defer tx.Rollback(ctx)

		var inviterID string
		err = tx.QueryRow(ctx, "SELECT id FROM users WHERE referral_code = $1", req.ReferralCode).Scan(&inviterID)
		if err != nil {
			http.Error(w, "Invalid referral code", http.StatusBadRequest)
			return
		}

		if inviterID == req.UserID {
			http.Error(w, "Cannot refer yourself", http.StatusBadRequest)
			return
		}

		res, err := tx.Exec(ctx, "UPDATE users SET referred_by = $1 WHERE id = $2 AND referred_by IS NULL", inviterID, req.UserID)
		if err != nil {
			http.Error(w, "Failed to apply referral", http.StatusInternalServerError)
			return
		}

		if res == 0 {
			http.Error(w, "User already referred or not found", http.StatusBadRequest)
			return
		}

		if err := tx.Commit(ctx); err != nil {
			http.Error(w, "Failed to commit transaction", http.StatusInternalServerError)
			return
		}

		resp := ApplyReferralResponse{Success: true}
		w.Header().Set("Content-Type", "application/json")
		json.NewEncoder(w).Encode(resp)
	}
}
