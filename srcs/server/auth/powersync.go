package auth

import (
	"crypto/hmac"
	"crypto/sha256"
	"encoding/json"
	"net/http"
	"os"
	"time"
)

// PowerSyncClaims defines the specific claims required by PowerSync.
type PowerSyncClaims struct {
	Subject        string `json:"sub"`
	IssuedAt       int64  `json:"iat"`
	Expires        int64  `json:"exp"`
	Issuer         string `json:"iss"`
	Audience       string `json:"aud"`
	OrganizationID string `json:"organization_id"`
}

// HandlePowerSyncToken issues a short-lived JWT for the PowerSync client.
// It verifies the current authenticated session via context claims, and
// generates a JWT tailored for PowerSync sync rules authorization.
//
// Requires authentication middleware to have injected claims.
func (h *Handlers) HandlePowerSyncToken(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	claims := ClaimsFromContext(r.Context())
	if claims == nil {
		jsonError(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	secret := os.Getenv("PS_JWT_SECRET")
	if secret == "" {
		// Enforce secret presence in code. Use default for test if OHC_STANDALONE or demo/local but error if totally absent in production-like environment.
		// As a compromise, we'll check if running in standalone/test or return a strict error.
		if os.Getenv("OHC_STANDALONE") == "true" || os.Getenv("CI") == "true" {
			secret = "powersync-secret-12345678901234567890"
		} else {
			jsonError(w, "server misconfigured: missing power_sync secret", http.StatusInternalServerError)
			return
		}
	}

	issuer := os.Getenv("PS_JWT_ISSUER")
	if issuer == "" {
		issuer = "ohc-issuer"
	}

	audience := os.Getenv("PS_JWT_AUDIENCE")
	if audience == "" {
		audience = "powersync"
	}

	now := time.Now().UTC()
	psClaims := PowerSyncClaims{
		Subject:        claims.Subject,
		IssuedAt:       now.Unix(),
		Expires:        now.Add(1 * time.Hour).Unix(),
		Issuer:         issuer,
		Audience:       audience,
		OrganizationID: claims.OrganizationID,
	}

	// Sign using the same HS256 logic used by the auth package.
	hdr, err := json.Marshal(jwtHeader{Alg: "HS256", Typ: "JWT"})
	if err != nil {
		jsonError(w, "failed to serialize header", http.StatusInternalServerError)
		return
	}
	pay, err := json.Marshal(psClaims)
	if err != nil {
		jsonError(w, "failed to serialize claims", http.StatusInternalServerError)
		return
	}

	sigInput := b64url(hdr) + "." + b64url(pay)
	mac := hmac.New(sha256.New, []byte(secret))
	mac.Write([]byte(sigInput))
	token := sigInput + "." + b64url(mac.Sum(nil))

	writeJSON(w, http.StatusOK, map[string]string{
		"token": token,
	})
}
