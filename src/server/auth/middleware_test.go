package auth_test

import (
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/stretchr/testify/require"
)

func TestMiddleware_SPIFFEFallback(t *testing.T) {
	s := setupTestStore(t)

	// A valid local test token for user
	userToken, err := s.GenerateToken(&auth.Claims{
		Subject: "user1",
		Roles:   []string{"user"},
		IssuedAt: time.Now().Unix(),
		ExpiresAt: time.Now().Add(time.Hour).Unix(),
	})
	require.NoError(t, err)

	mw := auth.Middleware(s)

	tests := []struct{
		name string
		token string
		wantStatus int
		wantSub string
	}{
		{
			name: "Valid user JWT",
			token: userToken,
			wantStatus: http.StatusOK,
			wantSub: "user1",
		},
		{
			name: "Valid SPIFFE ID",
			token: "spiffe://example.org/agent/123",
			wantStatus: http.StatusOK,
			wantSub: "spiffe://example.org/agent/123",
		},
		{
			name: "Invalid JWT and Invalid SPIFFE",
			token: "invalid-token",
			wantStatus: http.StatusUnauthorized,
		},
		{
			name: "Empty token",
			token: "",
			wantStatus: http.StatusUnauthorized,
		},
	}

	for _, tc := range tests {
		t.Run(tc.name, func(t *testing.T) {
			req := httptest.NewRequest(http.MethodGet, "/api/protected", nil)
			if tc.token != "" {
				req.Header.Set("Authorization", "Bearer " + tc.token)
			}

			rr := httptest.NewRecorder()
			var gotSub string
			handler := mw(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
				claims := auth.ClaimsFromContext(r.Context())
				if claims != nil {
					gotSub = claims.Subject
				}
				w.WriteHeader(http.StatusOK)
			}))

			handler.ServeHTTP(rr, req)

			require.Equal(t, tc.wantStatus, rr.Code)
			if tc.wantStatus == http.StatusOK {
				require.Equal(t, tc.wantSub, gotSub)
			}
		})
	}
}
