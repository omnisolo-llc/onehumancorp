package api

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandleGetVideoTutorials(t *testing.T) {
	handler := NewHelpHandler()
	mux := http.NewServeMux()
	handler.RegisterRoutes(mux)

	req, err := http.NewRequest("GET", "/api/v1/help/video-tutorials", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var tutorials []VideoTutorial
	err = json.NewDecoder(rr.Body).Decode(&tutorials)
	if err != nil {
		t.Fatal(err)
	}

	if len(tutorials) != 10 {
		t.Errorf("handler returned wrong number of tutorials: got %v want %v", len(tutorials), 10)
	}

	if tutorials[0].Title != "How to set up your store" {
		t.Errorf("first tutorial title wrong: got %v", tutorials[0].Title)
	}
}
