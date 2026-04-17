package api

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/domain"
)

func TestInviteHandler_HandleInvites(t *testing.T) {
	svc := domain.NewInviteService()
	handler := NewInviteHandler(svc)

	req := httptest.NewRequest(http.MethodGet, "/api/invites", nil)
	rr := httptest.NewRecorder()
	handler.HandleInvites(rr, req)
	if rr.Code != http.StatusMethodNotAllowed {
		t.Fatalf("expected 405, got %d", rr.Code)
	}

	req = httptest.NewRequest(http.MethodPost, "/api/invites", bytes.NewBufferString("{invalid json}"))
	rr = httptest.NewRecorder()
	handler.HandleInvites(rr, req)
	if rr.Code != http.StatusBadRequest {
		t.Fatalf("expected 400 for invalid json, got %d", rr.Code)
	}

	req = httptest.NewRequest(http.MethodPost, "/api/invites", bytes.NewBufferString(`{}`))
	rr = httptest.NewRecorder()
	handler.HandleInvites(rr, req)
	if rr.Code != http.StatusBadRequest {
		t.Fatalf("expected 400 for missing fields, got %d", rr.Code)
	}

	reqBody := `{"inviterId": "user1", "inviteeId": "user2"}`
	req = httptest.NewRequest(http.MethodPost, "/api/invites", bytes.NewBufferString(reqBody))
	rr = httptest.NewRecorder()
	handler.HandleInvites(rr, req)
	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", rr.Code)
	}

	var invite domain.Invite
	if err := json.Unmarshal(rr.Body.Bytes(), &invite); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	if invite.InviterID != "user1" || invite.InviteeID != "user2" || invite.Status != "PENDING" {
		t.Fatalf("unexpected invite in response: %+v", invite)
	}
}

func TestInviteHandler_HandleInviteAccept(t *testing.T) {
	svc := domain.NewInviteService()
	handler := NewInviteHandler(svc)

	reqBody := `{"inviterId": "user1", "inviteeId": "user2"}`
	req := httptest.NewRequest(http.MethodPost, "/api/invites", bytes.NewBufferString(reqBody))
	rr := httptest.NewRecorder()
	handler.HandleInvites(rr, req)

	var invite domain.Invite
	json.Unmarshal(rr.Body.Bytes(), &invite)
	id := invite.ID

	req = httptest.NewRequest(http.MethodGet, "/api/invites/"+id+"/accept", nil)
	rr = httptest.NewRecorder()
	handler.HandleInviteAccept(rr, req)
	if rr.Code != http.StatusMethodNotAllowed {
		t.Fatalf("expected 405, got %d", rr.Code)
	}

	req = httptest.NewRequest(http.MethodPost, "/api/invites/invalid", nil)
	rr = httptest.NewRecorder()
	handler.HandleInviteAccept(rr, req)
	if rr.Code != http.StatusBadRequest {
		t.Fatalf("expected 400 for invalid path, got %d", rr.Code)
	}

	req = httptest.NewRequest(http.MethodPost, "/api/invites/non-existent/accept", nil)
	rr = httptest.NewRecorder()
	handler.HandleInviteAccept(rr, req)
	if rr.Code != http.StatusNotFound {
		t.Fatalf("expected 404 for non-existent invite, got %d", rr.Code)
	}

	req = httptest.NewRequest(http.MethodPost, "/api/invites/"+id+"/accept", nil)
	rr = httptest.NewRecorder()
	handler.HandleInviteAccept(rr, req)
	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d: %s", rr.Code, rr.Body.String())
	}

	var accepted domain.Invite
	json.Unmarshal(rr.Body.Bytes(), &accepted)
	if accepted.Status != "ACCEPTED" {
		t.Fatalf("expected status ACCEPTED, got %s", accepted.Status)
	}
}
