package dashboard

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
)

func TestHandleSyncRules(t *testing.T) {
	s := &Server{}
	tests := []struct {
		name               string
		standaloneMode     string
		expectedQueryMatch string
	}{
		{
			name:               "Cloud Mode uses ANY array operator",
			standaloneMode:     "false",
			expectedQueryMatch: "ANY(mr.participants)",
		},
		{
			name:               "Standalone Mode uses JSON fallback",
			standaloneMode:     "true",
			expectedQueryMatch: "json_each(mr.participants)",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			originalStandalone := os.Getenv("OHC_STANDALONE")
			os.Setenv("OHC_STANDALONE", tt.standaloneMode)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")
			defer os.Setenv("OHC_STANDALONE", originalStandalone)
		os.Setenv("OHC_SQLITE_KEY", "standalone_ephemeral_key")
		defer os.Unsetenv("OHC_SQLITE_KEY")

			req := httptest.NewRequest(http.MethodGet, "/api/powersync/rules", nil)

			// Inject claims manually
			claims := &auth.Claims{
				OrganizationID: "org-123",
				UserID:         "user-123",
				Roles:          []string{auth.RoleAdmin},
			}
			ctx := context.WithValue(req.Context(), auth.ClaimsContextKey, claims)
			req = req.WithContext(ctx)

			w := httptest.NewRecorder()
			s.handleSyncRules(w, req)

			if w.Code != http.StatusOK {
				t.Fatalf("expected status 200, got %d", w.Code)
			}

			var response struct {
				Rules []struct {
					Table string `json:"table"`
					Query string `json:"query"`
				} `json:"rules"`
			}

			if err := json.Unmarshal(w.Body.Bytes(), &response); err != nil {
				t.Fatalf("failed to parse json response: %v", err)
			}

			foundMeetingRooms := false
			for _, rule := range response.Rules {
				if rule.Table == "meeting_rooms" {
					foundMeetingRooms = true
					if !strings.Contains(rule.Query, tt.expectedQueryMatch) {
						t.Errorf("expected meeting_rooms query to contain %q, but got %q", tt.expectedQueryMatch, rule.Query)
					}
				}
			}

			if !foundMeetingRooms {
				t.Errorf("meeting_rooms rule not found in response")
			}
		})
	}
}
