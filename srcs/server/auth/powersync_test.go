package auth_test

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/stretchr/testify/require"
	"context"
)

func TestHandlePowerSyncToken(t *testing.T) {
	s := auth.NewStore()
	h := auth.NewHandlers(s)

	req, err := http.NewRequest("GET", "/api/auth/powersync/token", nil)
	require.NoError(t, err)

	// Inject claims into context
	claims := &auth.Claims{
		Subject: "user-123",
		OrganizationID: "org-1",
		Roles: []string{"system"},
	}
	ctx := context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims)
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()
	h.HandlePowerSyncToken(rr, req)

	require.Equal(t, http.StatusOK, rr.Code)

	var resp auth.PowerSyncTokenResponse
	err = json.Unmarshal(rr.Body.Bytes(), &resp)
	require.NoError(t, err)

	require.NotEmpty(t, resp.Token)
	require.Equal(t, "http://localhost:8081", resp.PowerSyncURL)
}
