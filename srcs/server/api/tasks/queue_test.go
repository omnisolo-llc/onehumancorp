package tasks

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestQueueHandler(t *testing.T) {
	req, err := http.NewRequest("GET", "/api/tasks/queue", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(QueueHandler)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}
