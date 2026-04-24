package utils

import (
	"compress/gzip"
	"io"
	"net/http"
	"strings"
	"sync"
)

var gzipWriterPool = sync.Pool{
	New: func() interface{} {
		return gzip.NewWriter(io.Discard)
	},
}

type gzipResponseWriter struct {
	http.ResponseWriter
	gz             *gzip.Writer
	wroteHeader    bool
	shouldCompress bool
}

func isCompressible(contentType string) bool {
	if idx := strings.Index(contentType, ";"); idx != -1 {
		contentType = contentType[:idx]
	}
	contentType = strings.TrimSpace(contentType)

	if strings.HasPrefix(contentType, "text/") {
		return true
	}

	switch contentType {
	case "application/json",
		"application/javascript",
		"application/x-javascript",
		"application/xml",
		"application/xhtml+xml",
		"application/rss+xml",
		"application/atom+xml",
		"application/vnd.ms-fontobject",
		"application/x-font-ttf",
		"application/x-font-opentype",
		"application/x-font-truetype",
		"image/svg+xml":
		return true
	}
	return false
}

func (w *gzipResponseWriter) WriteHeader(statusCode int) {
	if w.wroteHeader {
		return
	}
	w.wroteHeader = true

	// Do not compress responses that must not have a body,
	// or if the content is already encoded.
	if statusCode == http.StatusNoContent || statusCode == http.StatusNotModified || statusCode < 200 || w.Header().Get("Content-Encoding") != "" {
		w.shouldCompress = false
	} else {
		contentType := w.Header().Get("Content-Type")
		if contentType == "" {
			// If not set before WriteHeader, assume we might compress if the content is text
			// But usually it's set by standard library sniffing in Write().
			// If explicitly called without Content-Type, we will not compress to be safe,
			// or we can allow it. It's safer not to compress if unknown.
			w.shouldCompress = false
		} else {
			if isCompressible(contentType) {
				w.shouldCompress = true
				w.Header().Set("Content-Encoding", "gzip")
				w.Header().Del("Content-Length")
			} else {
				w.shouldCompress = false
			}
		}
	}

	w.ResponseWriter.WriteHeader(statusCode)
}

func (w *gzipResponseWriter) Write(b []byte) (int, error) {
	if !w.wroteHeader {
		if w.Header().Get("Content-Type") == "" {
			w.Header().Set("Content-Type", http.DetectContentType(b))
		}
		w.WriteHeader(http.StatusOK)
	}

	if w.shouldCompress {
		if w.gz == nil {
			w.gz = gzipWriterPool.Get().(*gzip.Writer)
			w.gz.Reset(w.ResponseWriter)
		}
		return w.gz.Write(b)
	}

	return w.ResponseWriter.Write(b)
}

func (w *gzipResponseWriter) Flush() {
	if w.gz != nil {
		w.gz.Flush()
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

		gzw := &gzipResponseWriter{ResponseWriter: w}

		defer func() {
			if gzw.gz != nil {
				gzw.gz.Close()
				gzipWriterPool.Put(gzw.gz)
			} else if gzw.shouldCompress {
				// If shouldCompress is true but Write was never called, we must still write a valid empty gzip payload
				gzw.gz = gzipWriterPool.Get().(*gzip.Writer)
				gzw.gz.Reset(w)
				gzw.gz.Close()
				gzipWriterPool.Put(gzw.gz)
			}
		}()

		next.ServeHTTP(gzw, r)
	})
}
