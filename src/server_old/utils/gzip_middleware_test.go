package utils

import (
	"compress/gzip"
	"io"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestGzipMiddleware(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/plain")
		w.Write([]byte("hello world"))
	})

	gzipHandler := GzipMiddleware(handler)

	req := httptest.NewRequest("GET", "/", nil)
	req.Header.Set("Accept-Encoding", "gzip")
	rr := httptest.NewRecorder()

	gzipHandler.ServeHTTP(rr, req)

	if rr.Header().Get("Content-Encoding") != "gzip" {
		t.Errorf("Expected Content-Encoding gzip, got %s", rr.Header().Get("Content-Encoding"))
	}

	gr, err := gzip.NewReader(rr.Body)
	if err != nil {
		t.Fatalf("Failed to create gzip reader: %v", err)
	}
	defer gr.Close()

	body, err := io.ReadAll(gr)
	if err != nil {
		t.Fatalf("Failed to read decompressed body: %v", err)
	}

	if string(body) != "hello world" {
		t.Errorf("Expected 'hello world', got '%s'", string(body))
	}
}

func TestGzipMiddleware_NoAcceptEncoding(t *testing.T) {
	handler := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "text/plain")
		w.Write([]byte("hello world"))
	})

	gzipHandler := GzipMiddleware(handler)

	req := httptest.NewRequest("GET", "/", nil)
	rr := httptest.NewRecorder()

	gzipHandler.ServeHTTP(rr, req)

	if rr.Header().Get("Content-Encoding") == "gzip" {
		t.Errorf("Did not expect Content-Encoding gzip")
	}

	if rr.Body.String() != "hello world" {
		t.Errorf("Expected 'hello world', got '%s'", rr.Body.String())
	}
}
