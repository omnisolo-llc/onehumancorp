package kairos

import (
	"bytes"
	"context"
	"crypto/tls"
	"crypto/x509"
	"github.com/onehumancorp/mono/srcs/server/db"
	"net/http"
	"net/http/httptest"
	"net/url"
	"strings"
	"testing"
	"time"

	"github.com/gorilla/websocket"
)

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

func TestMeshAPI_HandleApprovals_NoRepo(t *testing.T) {
	mockMesh := &mockTeammateMesh{}
	api := NewMeshAPI(mockMesh, nil)

	req := httptest.NewRequest(http.MethodGet, "/api/kairos/mesh/approvals", nil)
	w := httptest.NewRecorder()

	api.HandleApprovals(w, req)

	if w.Code != http.StatusInternalServerError {
		t.Errorf("expected 500, got %d", w.Code)
	}
}

func TestMeshAPI_HandleApprovals_Success(t *testing.T) {
	mockMesh := &mockTeammateMesh{}
	provider := db.NewTestProvider(t)
	ctx := context.Background()

	_, err := provider.Exec(ctx, `
        CREATE TABLE IF NOT EXISTS shared_tasks (
            id TEXT PRIMARY KEY,
            agent_id TEXT,
            status TEXT,
            payload TEXT,
            created_at DATETIME,
            action_risk TEXT,
            approval_status TEXT
        );
    `)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	repo := NewSharedTaskRepo(provider)
	api := NewMeshAPI(mockMesh, repo)

	task := &SharedTask{
		ID:             "test-pending-1",
		AgentID:        "agent-1",
		Status:         "PENDING",
		ActionRisk:     "HIGH",
		ApprovalStatus: "PENDING",
		Payload:        []byte(`{"msg":"draft"}`),
		CreatedAt:      time.Now().Truncate(time.Second).UTC(),
	}
	if err := repo.Insert(ctx, task); err != nil {
		t.Fatalf("failed to insert: %v", err)
	}

	// GET test
	reqGet := httptest.NewRequest(http.MethodGet, "/api/kairos/mesh/approvals", nil)
	wGet := httptest.NewRecorder()
	api.HandleApprovals(wGet, reqGet)

	if wGet.Code != http.StatusOK {
		t.Errorf("expected 200 GET, got %d", wGet.Code)
	}
	if !strings.Contains(wGet.Body.String(), "test-pending-1") {
		t.Errorf("expected GET to return pending task, got %s", wGet.Body.String())
	}

	// POST/PUT test
	reqBody := `{"id":"test-pending-1", "status":"APPROVED"}`
	reqPost := httptest.NewRequest(http.MethodPost, "/api/kairos/mesh/approvals", bytes.NewBuffer([]byte(reqBody)))
	reqPost.Header.Set("Content-Type", "application/json")
	wPost := httptest.NewRecorder()

	api.HandleApprovals(wPost, reqPost)

	if wPost.Code != http.StatusOK {
		t.Errorf("expected 200 POST, got %d", wPost.Code)
	}

	fetched, _ := repo.Get(ctx, task.ID)
	if fetched.ApprovalStatus != "APPROVED" {
		t.Errorf("expected approval status APPROVED, got %s", fetched.ApprovalStatus)
	}
}
