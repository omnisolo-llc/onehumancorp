package auth

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestHandlePowerSyncToken_Success(t *testing.T) {
	store := NewStore()
	h := NewHandlers(store)

	req := httptest.NewRequest("GET", "/api/auth/powersync/token", nil)
	// Inject mock claims
	claims := &Claims{
		Subject:        "user-1",
		OrganizationID: "org-1",
		Roles:          []string{"system"},
	}
	ctx := context.WithValue(req.Context(), claimsContextKey, claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	h.HandlePowerSyncToken(rr, req)

	require.Equal(t, http.StatusOK, rr.Code)

	var resp map[string]interface{}
	err := json.NewDecoder(rr.Body).Decode(&resp)
	require.NoError(t, err)

	assert.Contains(t, resp, "token")
	assert.Contains(t, resp, "expires_at")
}

func TestHandlePowerSyncToken_Unauthorized(t *testing.T) {
	store := NewStore()
	h := NewHandlers(store)

	req := httptest.NewRequest("GET", "/api/auth/powersync/token", nil)

	rr := httptest.NewRecorder()
	h.HandlePowerSyncToken(rr, req)

	require.Equal(t, http.StatusUnauthorized, rr.Code)
}

func TestHandlePowerSyncJWKS(t *testing.T) {
	store := NewStore()
	h := NewHandlers(store)

	req := httptest.NewRequest("GET", "/api/auth/powersync/jwks", nil)

	rr := httptest.NewRecorder()
	h.HandlePowerSyncJWKS(rr, req)

	require.Equal(t, http.StatusOK, rr.Code)

	var resp map[string]interface{}
	err := json.NewDecoder(rr.Body).Decode(&resp)
	require.NoError(t, err)

	assert.Contains(t, resp, "keys")
	keys := resp["keys"].([]interface{})
	assert.Len(t, keys, 1)

	key := keys[0].(map[string]interface{})
	assert.Equal(t, "OKP", key["kty"])
	assert.Equal(t, "Ed25519", key["crv"])
	assert.Equal(t, "powersync-key-1", key["kid"])
	assert.Equal(t, "sig", key["use"])
	assert.Contains(t, key, "x")
}
