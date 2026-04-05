package auth_test

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHandlePowerSyncToken(t *testing.T) {
	handler := auth.HandlePowerSyncToken()

	t.Run("missing context claims", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)
		rec := httptest.NewRecorder()
		handler.ServeHTTP(rec, req)

		assert.Equal(t, http.StatusUnauthorized, rec.Code)
	})

	t.Run("success", func(t *testing.T) {
		req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/token", nil)
		claims := &auth.Claims{
			Subject:        "user-1",
			OrganizationID: "org-1",
			Roles:          []string{"system"},
		}

		// In go auth package ContextWithClaims is not exported, we use Context injection here if needed but tests are external.
		// Wait, I should use auth.ClaimsContextKeyForTest
		ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
		req = req.WithContext(ctx)

		rec := httptest.NewRecorder()
		handler.ServeHTTP(rec, req)

		assert.Equal(t, http.StatusOK, rec.Code)

		var res map[string]string
		err := json.NewDecoder(rec.Body).Decode(&res)
		assert.NoError(t, err)

		assert.NotEmpty(t, res["token"])
		assert.NotEmpty(t, res["expires_at"])
		_, err = time.Parse(time.RFC3339, res["expires_at"])
		assert.NoError(t, err)
	})
}

func TestHandlePowerSyncJWKS(t *testing.T) {
	handler := auth.HandlePowerSyncJWKS()
	req := httptest.NewRequest(http.MethodGet, "/api/auth/powersync/jwks", nil)
	rec := httptest.NewRecorder()

	handler.ServeHTTP(rec, req)

	assert.Equal(t, http.StatusOK, rec.Code)
	assert.Equal(t, "application/json", rec.Header().Get("Content-Type"))

	var res map[string]interface{}
	err := json.NewDecoder(rec.Body).Decode(&res)
	assert.NoError(t, err)

	keys, ok := res["keys"].([]interface{})
	assert.True(t, ok)
	assert.Len(t, keys, 1)

	key := keys[0].(map[string]interface{})
	assert.Equal(t, "OKP", key["kty"])
	assert.Equal(t, "Ed25519", key["crv"])
	assert.NotEmpty(t, key["kid"])
	assert.NotEmpty(t, key["x"])
}
