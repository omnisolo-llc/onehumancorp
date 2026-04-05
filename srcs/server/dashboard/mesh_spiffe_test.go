package dashboard

import (
	"bytes"
	"context"
	"crypto/tls"
	"crypto/x509"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"net/url"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/billing"
	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type contextKey string
const claimsContextKey contextKey = "auth_claims"

func contextWithClaims(ctx context.Context, claims *auth.Claims) context.Context {
	return context.WithValue(ctx, claimsContextKey, claims)
}

func TestTeammateMeshSPIFFEAuth(t *testing.T) {
	hub := orchestration.NewHub()
	tracker := billing.NewTracker(map[string]billing.Price{})
	org := domain.Organization{ID: "test-org"}
	server := &Server{hub: hub, tracker: tracker, org: org}

	hub.RegisterAgent(orchestration.Agent{
		ID:             "system",
		Name:           "System",
		Role:           "SYSTEM",
		OrganizationID: "test-org",
		Status:         orchestration.StatusIdle,
	})
	hub.RegisterAgent(orchestration.Agent{
		ID:             "agent-2",
		Name:           "Agent 2",
		Role:           "SOFTWARE_ENGINEER",
		OrganizationID: "test-org",
		Status:         orchestration.StatusIdle,
	})


	testCases := []struct {
		name           string
		endpoint       string
		method         string
		payload        map[string]interface{}
		setupTLS       func() *tls.ConnectionState
		expectedStatus int
	}{
		{
			name:     "No TLS ConnectionState",
			endpoint: "/api/mesh/broadcast",
			method:   http.MethodPost,
			payload: map[string]interface{}{
				"channel":  "mesh:tasks",
				"agent_id": "agent-1",
				"action":   "test",
				"status":   "idle",
			},
			setupTLS:       func() *tls.ConnectionState { return nil },
			expectedStatus: http.StatusForbidden,
		},
		{
			name:     "Missing Peer Certificates",
			endpoint: "/api/mesh/broadcast",
			method:   http.MethodPost,
			payload: map[string]interface{}{
				"channel":  "mesh:tasks",
				"agent_id": "agent-1",
				"action":   "test",
				"status":   "idle",
			},
			setupTLS: func() *tls.ConnectionState {
				return &tls.ConnectionState{}
			},
			expectedStatus: http.StatusForbidden,
		},
		{
			name:     "Invalid SPIFFE URI Domain",
			endpoint: "/api/mesh/broadcast",
			method:   http.MethodPost,
			payload: map[string]interface{}{
				"channel":  "mesh:tasks",
				"agent_id": "agent-1",
				"action":   "test",
				"status":   "idle",
			},
			setupTLS: func() *tls.ConnectionState {
				cert := &x509.Certificate{
					URIs: []*url.URL{
						&url.URL{Scheme: "spiffe", Host: "evil.com", Path: "/workload/mesh"},
					},
				}
				return &tls.ConnectionState{
					PeerCertificates: []*x509.Certificate{cert},
				}
			},
			expectedStatus: http.StatusForbidden,
		},
		{
			name:     "Invalid SPIFFE URI Path",
			endpoint: "/api/mesh/broadcast",
			method:   http.MethodPost,
			payload: map[string]interface{}{
				"channel":  "mesh:tasks",
				"agent_id": "agent-1",
				"action":   "test",
				"status":   "idle",
			},
			setupTLS: func() *tls.ConnectionState {
				cert := &x509.Certificate{
					URIs: []*url.URL{
						&url.URL{Scheme: "spiffe", Host: "onehumancorp.io", Path: "/invalid/"},
					},
				}
				return &tls.ConnectionState{
					PeerCertificates: []*x509.Certificate{cert},
				}
			},
			expectedStatus: http.StatusForbidden,
		},
		{
			name:     "Valid SPIFFE URI",
			endpoint: "/api/mesh/broadcast",
			method:   http.MethodPost,
			payload: map[string]interface{}{
				"channel":  "mesh:tasks",
				"agent_id": "agent-1",
				"action":   "test",
				"status":   "idle",
			},
			setupTLS: func() *tls.ConnectionState {
				cert := &x509.Certificate{
					URIs: []*url.URL{
						&url.URL{Scheme: "spiffe", Host: "onehumancorp.io", Path: "/workload/teammate-mesh"},
					},
				}
				return &tls.ConnectionState{
					PeerCertificates: []*x509.Certificate{cert},
				}
			},
			expectedStatus: http.StatusOK,
		},
		{
			name:     "Direct Mesh - Valid SPIFFE URI",
			endpoint: "/api/mesh/direct",
			method:   http.MethodPost,
			payload: map[string]interface{}{
				"toAgent": "agent-2",
				"payload": "test-payload",
			},
			setupTLS: func() *tls.ConnectionState {
				cert := &x509.Certificate{
					URIs: []*url.URL{
						&url.URL{Scheme: "spiffe", Host: "onehumancorp.io", Path: "/workload/direct"},
					},
				}
				return &tls.ConnectionState{
					PeerCertificates: []*x509.Certificate{cert},
				}
			},
			expectedStatus: http.StatusOK,
		},
	}

	for _, tc := range testCases {
		t.Run(tc.name, func(t *testing.T) {
			body, _ := json.Marshal(tc.payload)
			req := httptest.NewRequest(tc.method, tc.endpoint, bytes.NewReader(body))

			ctx := contextWithClaims(req.Context(), &auth.Claims{
				OrganizationID: "test-org",
			})
			req = req.WithContext(ctx)

			req.TLS = tc.setupTLS()

			rr := httptest.NewRecorder()

			if tc.endpoint == "/api/mesh/broadcast" {
				server.handleMeshBroadcast(rr, req)
			} else if tc.endpoint == "/api/mesh/direct" {
				server.handleMeshDirect(rr, req)
			}

			if status := rr.Code; status != tc.expectedStatus {
				t.Errorf("handler returned wrong status code: got %v want %v",
					status, tc.expectedStatus)
			}
		})
	}
}
