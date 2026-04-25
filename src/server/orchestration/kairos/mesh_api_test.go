package kairos

import (
	"errors"
	"github.com/onehumancorp/mono/src/server/auth"
	"bytes"
	"context"
	"crypto/tls"
	"crypto/x509"
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
)

type mockWorkflowEngine struct {
	pendingTasks []interface{}
	err          error
}

func (m *mockWorkflowEngine) GetPendingApprovalTasks(ctx context.Context, orgID string) (interface{}, error) {
	if m.err != nil {
		return nil, m.err
	}
	return m.pendingTasks, nil
}

func (m *mockWorkflowEngine) ApproveTask(ctx context.Context, taskID string, agentID string) error {
	return m.err
}

func (m *mockWorkflowEngine) RejectTask(ctx context.Context, taskID string, agentID string) error {
	return m.err
}

type mockTeammateMesh struct {
	TeammateMesh
	publishCalled bool
	lastChannel   string
	lastMessage   []byte
}

func (m *mockTeammateMesh) Publish(ctx context.Context, channel string, message []byte) error {
	m.publishCalled = true
	m.lastChannel = channel
	m.lastMessage = message
	return nil
}

func createMockTLSRequest(method, urlStr string, body []byte, hasCert bool) *http.Request {
	req := httptest.NewRequest(method, urlStr, bytes.NewBuffer(body))
	if hasCert {
		req.TLS = &tls.ConnectionState{
			PeerCertificates: []*x509.Certificate{{
				URIs: []*url.URL{{Scheme: "spiffe"}},
			}},
		}
	} else {
		req.TLS = nil
	}
	return req
}

func TestMeshAPI_HandlePublish(t *testing.T) {
	mockMesh := &mockTeammateMesh{}
	api := NewMeshAPI(mockMesh, nil)

	tests := []struct {
		name       string
		method     string
		body       string
		statusCode int
		hasCert    bool
	}{
		{"Method Not Allowed", http.MethodGet, "", http.StatusMethodNotAllowed, true},
		{"Missing Cert", http.MethodPost, `{"channel":"test_channel","message":{"key":"value"}}`, http.StatusForbidden, false},
		{"Invalid JSON", http.MethodPost, "{invalid}", http.StatusBadRequest, true},
		{"Missing Channel", http.MethodPost, `{"message":{"key":"value"}}`, http.StatusBadRequest, true},
		{"Success", http.MethodPost, `{"channel":"test_channel","message":{"key":"value"}}`, http.StatusOK, true},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			req := createMockTLSRequest(tt.method, "/api/kairos/mesh/publish", []byte(tt.body), tt.hasCert)
			w := httptest.NewRecorder()
			api.HandlePublish(w, req)
			if w.Code != tt.statusCode {
				t.Errorf("expected %d, got %d", tt.statusCode, w.Code)
			}
		})
	}

	if !mockMesh.publishCalled {
		t.Errorf("expected Publish to be called")
	}
	if mockMesh.lastChannel != "test_channel" {
		t.Errorf("expected channel test_channel, got %s", mockMesh.lastChannel)
	}
}

type mockTeammateMeshWithSubscribe struct {
	TeammateMesh
	subChan chan []byte
}

func (m *mockTeammateMeshWithSubscribe) Subscribe(ctx context.Context, channel string) (<-chan []byte, error) {
	return m.subChan, nil
}

func TestMeshAPI_HandleSubscribe_Errors(t *testing.T) {
	mockMesh := &mockTeammateMeshWithSubscribe{
		subChan: make(chan []byte, 1),
	}
	api := NewMeshAPI(mockMesh, nil)

	req2 := httptest.NewRequest(http.MethodPost, "/api/kairos/mesh/subscribe", nil)
	w2 := httptest.NewRecorder()
	api.HandleSubscribe(w2, req2)
	if w2.Code != http.StatusMethodNotAllowed {
		t.Errorf("expected 405, got %d", w2.Code)
	}

	req3 := httptest.NewRequest(http.MethodGet, "/api/kairos/mesh/subscribe", nil)
	w3 := httptest.NewRecorder()
	api.HandleSubscribe(w3, req3)
	if w3.Code != http.StatusBadRequest {
		t.Errorf("expected 400, got %d", w3.Code)
	}
}

func TestMeshAPI_HandleSubscribe_Success(t *testing.T) {
	mockMesh := &mockTeammateMeshWithSubscribe{
		subChan: make(chan []byte, 10),
	}
	api := NewMeshAPI(mockMesh, nil)

	mux := http.NewServeMux()
	api.RegisterRoutes(mux)

	server := httptest.NewServer(mux)
	defer server.Close()

	wsURL := "ws" + strings.TrimPrefix(server.URL, "http") + "/api/kairos/mesh/subscribe?channel=test_sub_channel"

	dialer := websocket.Dialer{}
	ws, _, err := dialer.Dial(wsURL, nil)
	if err != nil {
		t.Fatalf("failed to connect to websocket: %v", err)
	}
	defer ws.Close()

	// Send a message from the "mesh" to the client
	expectedMsg := []byte("hello from mesh")
	mockMesh.subChan <- expectedMsg

	// Read message from the websocket
	ws.SetReadDeadline(time.Now().Add(2 * time.Second))
	_, msg, err := ws.ReadMessage()
	if err != nil {
		t.Fatalf("failed to read message: %v", err)
	}

	if string(msg) != string(expectedMsg) {
		t.Errorf("expected %s, got %s", expectedMsg, msg)
	}
}

func TestMeshAPI_HandleGetPendingActions(t *testing.T) {
	mockEngine := &mockWorkflowEngine{pendingTasks: []interface{}{}}
	api := NewMeshAPI(&mockTeammateMesh{}, mockEngine)

	// Test unauthorized
	req := httptest.NewRequest(http.MethodGet, "/api/kairos/actions/pending", nil)
	w := httptest.NewRecorder()
	api.HandleGetPendingActions(w, req)
	if w.Code != http.StatusUnauthorized {
		t.Errorf("expected 401, got %d", w.Code)
	}

	// Test success
	claims := &auth.Claims{OrganizationID: "test-org"}
	req = httptest.NewRequest(http.MethodGet, "/api/kairos/actions/pending", nil)
	req = req.WithContext(context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims))
	w = httptest.NewRecorder()
	api.HandleGetPendingActions(w, req)
	if w.Code != http.StatusOK {
		t.Errorf("expected 200, got %d", w.Code)
	}

	// Test error
	mockEngine.err = errors.New("db error")
	w = httptest.NewRecorder()
	api.HandleGetPendingActions(w, req)
	if w.Code != http.StatusInternalServerError {
		t.Errorf("expected 500, got %d", w.Code)
	}
}

func TestMeshAPI_HandleApproveRejectAction(t *testing.T) {
	mockEngine := &mockWorkflowEngine{}
	api := NewMeshAPI(&mockTeammateMesh{}, mockEngine)

	tests := []struct {
		name       string
		method     string
		endpoint   string
		handler    func(w http.ResponseWriter, r *http.Request)
	}{
		{"Approve", http.MethodPost, "/api/kairos/actions/approve", api.HandleApproveAction},
		{"Reject", http.MethodPost, "/api/kairos/actions/reject", api.HandleRejectAction},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			// Test method not allowed
			req := httptest.NewRequest(http.MethodGet, tt.endpoint, nil)
			w := httptest.NewRecorder()
			tt.handler(w, req)
			if w.Code != http.StatusMethodNotAllowed {
				t.Errorf("expected 405, got %d", w.Code)
			}

			// Test unauthorized
			req = httptest.NewRequest(http.MethodPost, tt.endpoint, nil)
			w = httptest.NewRecorder()
			tt.handler(w, req)
			if w.Code != http.StatusUnauthorized {
				t.Errorf("expected 401, got %d", w.Code)
			}

			// Test success
			claims := &auth.Claims{OrganizationID: "test-org", Subject: "user1"}
			body := bytes.NewBuffer([]byte(`{"task_id":"task1"}`))
			req = httptest.NewRequest(http.MethodPost, tt.endpoint, body)
			req = req.WithContext(context.WithValue(req.Context(), auth.ClaimsContextKeyForTest, claims))
			w = httptest.NewRecorder()
			tt.handler(w, req)
			if w.Code != http.StatusOK {
				t.Errorf("expected 200, got %d", w.Code)
			}
		})
	}
}
