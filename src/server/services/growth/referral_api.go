package growth

import (
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
	Link             string `json:"link"`
	PreFilledMessage string `json:"pre_filled_message"`
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

	preFilled := fmt.Sprintf("Share OHC with a friend, both get 1 month free Pro! Join here: %s", link)

	resp := ReferralResponse{
		Link:             link,
		PreFilledMessage: preFilled,
	}
	w.Header().Set("Content-Type", "application/json")
	json.NewEncoder(w).Encode(resp)
}
