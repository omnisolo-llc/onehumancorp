package utils

import (
	"bytes"
	"compress/gzip"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestGzipMiddleware_EmptyBodyWithCompressibleType(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		// Explicitly write header to trigger compression logic, but write no body.
		w.WriteHeader(http.StatusOK)
	})

	gzipHandler := GzipMiddleware(handler)

	req := httptest.NewRequest("GET", "/", nil)
	req.Header.Set("Accept-Encoding", "gzip")
	rr := httptest.NewRecorder()

	gzipHandler.ServeHTTP(rr, req)

	if rr.Header().Get("Content-Encoding") != "gzip" {
		t.Errorf("Expected Content-Encoding gzip, got %s", rr.Header().Get("Content-Encoding"))
	}

	if rr.Body.Len() == 0 {
		t.Errorf("Expected non-empty body (valid empty gzip trailer), got 0 bytes")
	}

	gr, err := gzip.NewReader(bytes.NewReader(rr.Body.Bytes()))
	if err != nil {
		t.Fatalf("Failed to create gzip reader for empty payload: %v", err)
	}
	defer gr.Close()

	body, err := io.ReadAll(gr)
	if err != nil {
		t.Fatalf("Failed to read decompressed body: %v", err)
	}

	if len(body) != 0 {
		t.Errorf("Expected 0 bytes of decompressed payload, got %d", len(body))
	}
}
