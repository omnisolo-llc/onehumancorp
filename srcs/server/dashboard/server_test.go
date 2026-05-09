package dashboard

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"strings"
	"sync"
	"testing"
	"time"
)

// To avoid the data race on ResponseRecorder caused by reading it in
// the test goroutine while the handler writes to it in a separate goroutine,
// we create a custom ResponseRecorder that intercepts body writes safely.
type threadSafeRecorder struct {
	*httptest.ResponseRecorder
	bodyChan chan []byte
    mu       sync.Mutex
}

func newThreadSafeRecorder() *threadSafeRecorder {
	return &threadSafeRecorder{
		ResponseRecorder: httptest.NewRecorder(),
		bodyChan:         make(chan []byte, 10),
	}
}

func (r *threadSafeRecorder) WriteHeader(statusCode int) {
    r.mu.Lock()
    defer r.mu.Unlock()
    r.ResponseRecorder.WriteHeader(statusCode)
}

func (r *threadSafeRecorder) Write(buf []byte) (int, error) {
    r.mu.Lock()
    defer r.mu.Unlock()

	// Copy the buffer so we don't hold a reference to the handler's array
	cp := make([]byte, len(buf))
	copy(cp, buf)

	// Non-blocking send
	select {
	case r.bodyChan <- cp:
	default:
	}

	return r.ResponseRecorder.Write(buf)
}

func (r *threadSafeRecorder) Flush() {
    r.mu.Lock()
    defer r.mu.Unlock()
    r.ResponseRecorder.Flush()
}

func (r *threadSafeRecorder) Code() int {
    r.mu.Lock()
    defer r.mu.Unlock()
    return r.ResponseRecorder.Code
}

func TestHandleStream(t *testing.T) {
	req, err := http.NewRequest("GET", "/api/v1/stream", nil)
	if err != nil {
		t.Fatal(err)
	}

	ctx, cancel := context.WithCancel(context.Background())
	req = req.WithContext(ctx)

	rr := newThreadSafeRecorder()

	go func() {
		HandleStream(rr, req)
	}()

	expectedPrefix := "data: test event\n\n"
	success := false

	// Wait to let handler register
	time.Sleep(50 * time.Millisecond)

	var buf bytes.Buffer

	for i := 0; i < 50; i++ {
		select {
		case GlobalBroker.messages <- "test event":
			// Successfully sent message, now read from our thread-safe channel
			for j := 0; j < 50; j++ {
				select {
				case b := <-rr.bodyChan:
					buf.Write(b)
					if strings.HasPrefix(buf.String(), expectedPrefix) {
						success = true
						break
					}
				case <-time.After(10 * time.Millisecond):
				}
				if success {
					break
				}
			}
		default:
		}
		if success {
			break
		}
		time.Sleep(10 * time.Millisecond)
	}

	cancel()

    code := rr.Code()
	if code != http.StatusOK && code != 200 {
		t.Errorf("handler returned wrong status code: got %v want %v",
			code, http.StatusOK)
	}

	if !success {
		t.Errorf("handler returned unexpected body: got %v want prefix %v",
			buf.String(), expectedPrefix)
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
