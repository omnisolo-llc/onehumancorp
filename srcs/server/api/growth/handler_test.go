package growth

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/onehumancorp/mono/lib/analytics"
)

func TestGrowthHandler_HandleTrackVisit(t *testing.T) {
	tracker := analytics.NewTracker()
	handler := NewGrowthHandler(tracker)

	reqBody := TrackVisitRequest{
		PageID:    "page1",
		VisitorID: "visitor1",
	}
	body, _ := json.Marshal(reqBody)

	req, _ := http.NewRequest(http.MethodPost, "/visit", bytes.NewBuffer(body))
	rr := httptest.NewRecorder()

	handler.HandleTrackVisit(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}

func TestGrowthHandler_HandleTrackConversion(t *testing.T) {
	tracker := analytics.NewTracker()
	handler := NewGrowthHandler(tracker)

	reqBody := TrackConversionRequest{
		PageID:    "page1",
		VisitorID: "visitor1",
	}
	body, _ := json.Marshal(reqBody)

	req, _ := http.NewRequest(http.MethodPost, "/conversion", bytes.NewBuffer(body))
	rr := httptest.NewRecorder()

	handler.HandleTrackConversion(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}

func TestGrowthHandler_HandleInviteTeam(t *testing.T) {
	tracker := analytics.NewTracker()
	handler := NewGrowthHandler(tracker)

	reqBody := InviteTeamRequest{
		TeamID:       "team1",
		InviterID:    "inviter1",
		InviteeEmail: "invitee@example.com",
	}
	body, _ := json.Marshal(reqBody)

	req, _ := http.NewRequest(http.MethodPost, "/invite", bytes.NewBuffer(body))
	rr := httptest.NewRecorder()

	handler.HandleInviteTeam(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}

func TestGrowthHandler_HandleAcceptInvite(t *testing.T) {
	tracker := analytics.NewTracker()
	handler := NewGrowthHandler(tracker)

	reqBody := AcceptInviteRequest{
		InviteID:  "invite1",
		InviteeID: "invitee1",
	}
	body, _ := json.Marshal(reqBody)

	req, _ := http.NewRequest(http.MethodPost, "/accept", bytes.NewBuffer(body))
	rr := httptest.NewRecorder()

	handler.HandleAcceptInvite(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}
