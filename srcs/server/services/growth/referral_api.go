package growth

import (
	"crypto/rand"
	"encoding/hex"
	"encoding/json"
	"fmt"
	"net/http"
	"time"
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

// GenerateToken generates a random token for sharing.
func GenerateToken() string {
	bytes := make([]byte, 16)
	if _, err := rand.Read(bytes); err != nil {
		return hex.EncodeToString([]byte(fmt.Sprintf("%d", time.Now().UnixNano())))
	}
	return hex.EncodeToString(bytes)
}
