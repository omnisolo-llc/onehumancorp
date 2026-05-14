package dashboard

import (
	"context"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"
)

func TestHandleStream(t *testing.T) {
	req, err := http.NewRequest("GET", "/api/v1/stream", nil)
	if err != nil {
		t.Fatal(err)
	}

	ctx, cancel := context.WithCancel(context.Background())
	req = req.WithContext(ctx)

	rr := httptest.NewRecorder()

	go func() {
		HandleStream(rr, req)
	}()

	time.Sleep(100 * time.Millisecond)

	GlobalBroker.messages <- "test event"

	time.Sleep(100 * time.Millisecond)
	cancel()
	time.Sleep(100 * time.Millisecond)

	if rr.Code != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			rr.Code, http.StatusOK)
	}

	expectedPrefix := "data: test event\n\n"
	if len(rr.Body.String()) < len(expectedPrefix) || rr.Body.String()[:len(expectedPrefix)] != expectedPrefix {
		t.Errorf("handler returned unexpected body: got %v want prefix %v",
			rr.Body.String(), expectedPrefix)
	}
}
