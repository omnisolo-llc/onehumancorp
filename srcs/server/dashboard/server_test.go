package dashboard

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"sync"
	"testing"
	"time"
)

type SafeRecorder struct {
	*httptest.ResponseRecorder
	mu   sync.Mutex
	code int
	buf  bytes.Buffer
}

func NewSafeRecorder() *SafeRecorder {
	return &SafeRecorder{
		ResponseRecorder: httptest.NewRecorder(),
		code:             http.StatusOK,
	}
}

func (rw *SafeRecorder) WriteHeader(code int) {
	rw.mu.Lock()
	defer rw.mu.Unlock()
	rw.code = code
	rw.ResponseRecorder.WriteHeader(code)
}

func (rw *SafeRecorder) Write(b []byte) (int, error) {
	rw.mu.Lock()
	defer rw.mu.Unlock()
	rw.buf.Write(b)
	return rw.ResponseRecorder.Write(b)
}

func (rw *SafeRecorder) BodyString() string {
	rw.mu.Lock()
	defer rw.mu.Unlock()
	return rw.buf.String()
}

func (rw *SafeRecorder) GetCode() int {
	rw.mu.Lock()
	defer rw.mu.Unlock()
	return rw.code
}

func TestHandleStream(t *testing.T) {
	req, err := http.NewRequest("GET", "/api/v1/stream", nil)
	if err != nil {
		t.Fatal(err)
	}

	ctx, cancel := context.WithCancel(context.Background())
	req = req.WithContext(ctx)

	rr := NewSafeRecorder()

	go func() {
		HandleStream(rr, req)
	}()

	time.Sleep(100 * time.Millisecond)

	GlobalBroker.messages <- "test event"

	time.Sleep(100 * time.Millisecond)
	cancel()
	time.Sleep(100 * time.Millisecond)

	if rr.GetCode() != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			rr.GetCode(), http.StatusOK)
	}

	expectedPrefix := "data: test event\n\n"
	bodyStr := rr.BodyString()
	if len(bodyStr) < len(expectedPrefix) || bodyStr[:len(expectedPrefix)] != expectedPrefix {
		t.Errorf("handler returned unexpected body: got %v want prefix %v",
			bodyStr, expectedPrefix)
	}
}

func TestHandleAutoDreamSync(t *testing.T) {
	req, err := http.NewRequest("POST", "/api/v1/autodream/sync", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(HandleAutoDreamSync)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}
}

func TestHandleAutoDreamQuery(t *testing.T) {
	req, err := http.NewRequest("GET", "/api/v1/autodream/query", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(HandleAutoDreamQuery)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}
}

func TestHandleMeshBroadcast(t *testing.T) {
	req, err := http.NewRequest("POST", "/api/mesh/broadcast", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(HandleMeshBroadcast)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}
}
