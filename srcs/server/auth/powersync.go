package auth

import (
	"crypto/ed25519"
	"crypto/rand"
	"net/http"
	"time"

	"gopkg.in/square/go-jose.v2"
	"gopkg.in/square/go-jose.v2/jwt"
)

var (
	// PowerSync uses Ed25519 or RSA keys to sign JWTs. We generate an Ed25519 key pair on startup.
	psPubKey  ed25519.PublicKey
	psPrivKey ed25519.PrivateKey
	psKeyID   = "ohc-powersync-key"
)

func init() {
	var err error
	psPubKey, psPrivKey, err = ed25519.GenerateKey(rand.Reader)
	if err != nil {
		panic("failed to generate powersync signing key: " + err.Error())
	}
}

// PowerSyncTokenResponse is the response format expected by PowerSync client.
type PowerSyncTokenResponse struct {
	Token     string `json:"token"`
	PowerSyncURL string `json:"powersync_url"`
	ExpiresAt int64  `json:"expiresAt"`
}

// HandlePowerSyncToken generates a short-lived JWT for PowerSync clients.
// The JWT payload MUST include the user's organization_id.
// Accepts parameters: h *Handlers (No Constraints).
// Returns nothing.
// Produces no errors.
// Has no side effects.
func (h *Handlers) HandlePowerSyncToken(w http.ResponseWriter, r *http.Request) {
	claims := ClaimsFromContext(r.Context())
	if claims == nil {
		jsonError(w, "unauthorized", http.StatusUnauthorized)
		return
	}

	orgID := claims.OrganizationID
	if orgID == "" {
		// Default to system org if multi-tenant is disabled or not set, so local sync still works
		orgID = "sys"
	}

	sig, err := jose.NewSigner(jose.SigningKey{Algorithm: jose.EdDSA, Key: psPrivKey}, (&jose.SignerOptions{}).WithType("JWT").WithHeader("kid", psKeyID))
	if err != nil {
		jsonError(w, "failed to create signer", http.StatusInternalServerError)
		return
	}

	now := time.Now().UTC()
	exp := now.Add(5 * time.Minute)

	// PowerSync expects standard claims plus custom claims (like organization_id)
	cl := struct {
		jwt.Claims
		OrganizationID string `json:"organization_id"`
	}{
		Claims: jwt.Claims{
			Subject:   claims.Subject,
			Issuer:    "ohc-server",
			Audience:  jwt.Audience{"powersync"},
			IssuedAt:  jwt.NewNumericDate(now),
			Expiry:    jwt.NewNumericDate(exp),
		},
		OrganizationID: orgID,
	}

	raw, err := jwt.Signed(sig).Claims(cl).CompactSerialize()
	if err != nil {
		jsonError(w, "failed to sign token", http.StatusInternalServerError)
		return
	}

	// We leave PowerSyncURL empty so the client uses the same domain it used to fetch the token.
	resp := PowerSyncTokenResponse{
		Token:        raw,
		PowerSyncURL: "",
		ExpiresAt:    exp.Unix(),
	}

	writeJSON(w, http.StatusOK, resp)
}

// HandlePowerSyncJwks serves the public key in JWKS format for the PowerSync backend.
// Accepts parameters: h *Handlers (No Constraints).
// Returns nothing.
// Produces no errors.
// Has no side effects.
func (h *Handlers) HandlePowerSyncJwks(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		jsonError(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	jwks := jose.JSONWebKeySet{
		Keys: []jose.JSONWebKey{
			{
				Key:       psPubKey,
				KeyID:     psKeyID,
				Algorithm: string(jose.EdDSA),
				Use:       "sig",
			},
		},
	}

	writeJSON(w, http.StatusOK, jwks)
}
