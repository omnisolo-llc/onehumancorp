package auth

import (
	"crypto/ed25519"
	"encoding/base64"
	"encoding/json"
	"net/http"
	"os"
	"time"

	"github.com/golang-jwt/jwt/v5"
)

var (
	cachedPowerSyncPrivKey ed25519.PrivateKey
	cachedPowerSyncPubKey  ed25519.PublicKey
	cachedPowerSyncKeyInit bool
)

func initPowerSyncKeys() error {
	if cachedPowerSyncKeyInit {
		return nil
	}
	privKeyStr := os.Getenv("OHC_POWERSYNC_PRIV_KEY")
	if privKeyStr == "" {
		return nil
	}

	privKeyBytes, err := base64.RawURLEncoding.DecodeString(privKeyStr)
	if err != nil {
		return err
	}

	if len(privKeyBytes) != ed25519.SeedSize {
		return nil
	}

	cachedPowerSyncPrivKey = ed25519.NewKeyFromSeed(privKeyBytes)
	cachedPowerSyncPubKey = cachedPowerSyncPrivKey.Public().(ed25519.PublicKey)
	cachedPowerSyncKeyInit = true
	return nil
}

// powersyncJWTClaims holds the claims for a PowerSync JWT.
type powersyncJWTClaims struct {
	jwt.RegisteredClaims
	Parameters map[string]interface{} `json:"parameters"`
}

// HandlePowerSyncToken generates a short-lived JWT for PowerSync clients.
// The JWT payload MUST include the user's organization_id.
// Uses Ed25519 for signing, seeded from OHC_POWERSYNC_PRIV_KEY.
func (h *Handlers) HandlePowerSyncToken(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	claims := ClaimsFromContext(r.Context())
	if claims == nil {
		http.Error(w, "unauthorized: missing claims", http.StatusUnauthorized)
		return
	}

	if err := initPowerSyncKeys(); err != nil || !cachedPowerSyncKeyInit {
		http.Error(w, "internal server error: failed to initialize keys", http.StatusInternalServerError)
		return
	}

	// Create the JWT claims
	now := time.Now().UTC()
	tokenClaims := powersyncJWTClaims{
		RegisteredClaims: jwt.RegisteredClaims{
			Subject:   claims.Subject,
			Issuer:    "ohc-api",
			Audience:  jwt.ClaimStrings{"powersync"},
			IssuedAt:  jwt.NewNumericDate(now),
			ExpiresAt: jwt.NewNumericDate(now.Add(1 * time.Hour)),
		},
		Parameters: map[string]interface{}{
			"organization_id": claims.OrganizationID,
		},
	}

	// Sign the token
	token := jwt.NewWithClaims(jwt.SigningMethodEdDSA, tokenClaims)

	// PowerSync expects kid in header
	token.Header["kid"] = "powersync-key-1"

	tokenString, err := token.SignedString(cachedPowerSyncPrivKey)
	if err != nil {
		http.Error(w, "internal server error: failed to sign token", http.StatusInternalServerError)
		return
	}

	powersyncUrl := os.Getenv("POWERSYNC_URL")
	if powersyncUrl == "" {
		powersyncUrl = "http://localhost:8081"
	}

	response := map[string]string{
		"token": tokenString,
		"powersync_url": powersyncUrl,
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	if err := json.NewEncoder(w).Encode(response); err != nil {
		// Log error if needed
	}
}
