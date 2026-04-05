package utils

import (
	"compress/gzip"
	"io"
	"net/http"
	"strings"
)

type gzipResponseWriter struct {
	io.Writer
	http.ResponseWriter
	wroteHeader bool
}

func (w *gzipResponseWriter) WriteHeader(statusCode int) {
	if w.wroteHeader {
		return
	}
	w.wroteHeader = true

	// Do not compress responses that must not have a body
	if statusCode == http.StatusNoContent || statusCode == http.StatusNotModified || statusCode < 200 {
		w.Header().Del("Content-Encoding")
	} else {
		// If we are compressing, the content length will change
		w.Header().Del("Content-Length")
	}
	w.ResponseWriter.WriteHeader(statusCode)
}

func (w *gzipResponseWriter) Write(b []byte) (int, error) {
	if !w.wroteHeader {
		w.WriteHeader(http.StatusOK)
	}

	// Sniff content type before compressing if not set
	if w.Header().Get("Content-Type") == "" {
		w.Header().Set("Content-Type", http.DetectContentType(b))
	}

	if w.Header().Get("Content-Encoding") == "gzip" {
		return w.Writer.Write(b)
	}
	// Fallback to normal write if compression was disabled (e.g. for 204)
	return w.ResponseWriter.Write(b)
}

func (w *gzipResponseWriter) Flush() {
	if f, ok := w.Writer.(*gzip.Writer); ok {
		f.Flush()
	}
	if f, ok := w.ResponseWriter.(http.Flusher); ok {
		f.Flush()
	}
}

func GzipMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if !strings.Contains(r.Header.Get("Accept-Encoding"), "gzip") {
			next.ServeHTTP(w, r)
			return
		}

		// WebSockets and SSE shouldn't be blindly gzipped by naive middleware
		if r.Header.Get("Upgrade") != "" || r.Header.Get("Accept") == "text/event-stream" {
			next.ServeHTTP(w, r)
			return
		}

		w.Header().Add("Vary", "Accept-Encoding")
		w.Header().Set("Content-Encoding", "gzip")

		gz := gzip.NewWriter(w)
		gzw := &gzipResponseWriter{Writer: gz, ResponseWriter: w}

		defer func() {
			if gzw.Header().Get("Content-Encoding") == "gzip" {
				gz.Close()
			}
		}()

		next.ServeHTTP(gzw, r)
	})
}
