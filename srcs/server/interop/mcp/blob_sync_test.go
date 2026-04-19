package mcp

import (
	"context"
	"crypto/tls"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
)

func TestBlobSyncTool_Execute(t *testing.T) {
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/mcp/blob_metadata/valid-blob-id" {
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(MetadataResponse{
				DownloadUrl: "http://" + r.Host + "/api/mcp/blob/valid-blob-id",
			})
			return
		}
		if r.URL.Path == "/api/mcp/blob/valid-blob-id" {
			w.WriteHeader(http.StatusOK)
			w.Write([]byte("mock blob data"))
			return
		}
		if r.URL.Path == "/api/mcp/blob_metadata/not-found" {
			w.WriteHeader(http.StatusNotFound)
			return
		}
		if r.URL.Path == "/api/mcp/blob_metadata/invalid-json" {
			w.WriteHeader(http.StatusOK)
			w.Write([]byte("invalid json"))
			return
		}
		if r.URL.Path == "/api/mcp/blob_metadata/io-error" {
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(MetadataResponse{
				DownloadUrl: "http://" + r.Host + "/api/mcp/blob/io-error",
			})
			return
		}
		if r.URL.Path == "/api/mcp/blob/io-error" {
			w.Header().Set("Content-Length", "100")
			w.WriteHeader(http.StatusOK)
			w.Write([]byte("short"))
			return
		}
		if r.URL.Path == "/api/mcp/blob_metadata/invalid-download-url" {
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(MetadataResponse{
				DownloadUrl: "http://127.0.0.1:0/api/mcp/blob/download-error",
			})
			return
		}
		if r.URL.Path == "/api/mcp/blob_metadata/download-error" {
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(MetadataResponse{
				DownloadUrl: "http://" + r.Host + "/api/mcp/blob/download-error",
			})
			return
		}
		if r.URL.Path == "/api/mcp/blob/download-error" {
			w.WriteHeader(http.StatusInternalServerError)
			return
		}
        if r.URL.Path == "/api/mcp/blob_metadata/invalid-new-req" {
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(MetadataResponse{
				DownloadUrl: "http://\x00invalid",
			})
			return
		}
	}))
	defer ts.Close()

	proxy := NewMcpSyncProxy(nil, &tls.Config{}, ts.URL)
	tool := NewBlobSyncTool(proxy)

	t.Run("successful download", func(t *testing.T) {
		err := tool.Execute(context.Background(), "valid-blob-id")
		if err != nil {
			t.Fatalf("expected no error, got %v", err)
		}

		blobDir := os.TempDir()
		content, err := os.ReadFile(filepath.Join(blobDir, "valid-blob-id"))
		if err != nil {
			t.Fatalf("failed to read created blob file: %v", err)
		}
		if string(content) != "mock blob data" {
			t.Errorf("expected 'mock blob data', got '%s'", string(content))
		}

		os.Remove(filepath.Join(blobDir, "valid-blob-id"))
	})

	t.Run("download metadata failure", func(t *testing.T) {
		err := tool.Execute(context.Background(), "not-found")
		if err == nil {
			t.Fatalf("expected error, got nil")
		}
		if err.Error() != "failed to download blob metadata: status 404" {
			t.Errorf("unexpected error message: %v", err)
		}
	})

	t.Run("invalid blob id", func(t *testing.T) {
		err := tool.Execute(context.Background(), "../../../etc/shadow")
		if err == nil {
			t.Fatalf("expected error for path traversal, got nil")
		}
	})

	t.Run("invalid request", func(t *testing.T) {
		badProxy := NewMcpSyncProxy(nil, nil, "http://\x00invalid")
		badTool := NewBlobSyncTool(badProxy)
		err := badTool.Execute(context.Background(), "valid-blob-id")
		if err == nil {
			t.Fatalf("expected error for invalid url, got nil")
		}
	})

	t.Run("connection error", func(t *testing.T) {
		badProxy := NewMcpSyncProxy(nil, nil, "http://127.0.0.1:0")
		badTool := NewBlobSyncTool(badProxy)
		err := badTool.Execute(context.Background(), "valid-blob-id")
		if err == nil {
			t.Fatalf("expected error for connection error, got nil")
		}
	})

	t.Run("invalid json", func(t *testing.T) {
		err := tool.Execute(context.Background(), "invalid-json")
		if err == nil {
			t.Fatalf("expected error for invalid json, got nil")
		}
	})

	t.Run("io copy error", func(t *testing.T) {
		err := tool.Execute(context.Background(), "io-error")
		if err == nil {
			t.Fatalf("expected error for short read, got nil")
		}
		blobDir := os.TempDir()
		os.Remove(filepath.Join(blobDir, "io-error"))
	})

	t.Run("invalid download url", func(t *testing.T) {
		err := tool.Execute(context.Background(), "invalid-download-url")
		if err == nil {
			t.Fatalf("expected error for invalid download url, got nil")
		}
	})

	t.Run("download error", func(t *testing.T) {
		err := tool.Execute(context.Background(), "download-error")
		if err == nil {
			t.Fatalf("expected error for download error, got nil")
		}
	})

	t.Run("create file error", func(t *testing.T) {
		blobDir := os.TempDir()
		os.MkdirAll(filepath.Join(blobDir, "valid-blob-id"), 0755)

		err := tool.Execute(context.Background(), "valid-blob-id")
		if err == nil {
			t.Fatalf("expected error when file creation fails")
		}

		os.RemoveAll(filepath.Join(blobDir, "valid-blob-id"))
	})

    t.Run("metadata context error", func(t *testing.T) {
        ctx, cancel := context.WithCancel(context.Background())
        cancel()

        err := tool.Execute(ctx, "valid-blob-id")
        if err == nil {
             t.Fatalf("expected error when context canceled")
        }
    })

    t.Run("download metadata nil context error", func(t *testing.T) {
        err := tool.Execute(nil, "valid-blob-id")
        if err == nil {
             t.Fatalf("expected error when context is nil")
        }
    })

    t.Run("invalid new req", func(t *testing.T) {
        err := tool.Execute(context.Background(), "invalid-new-req")
        if err == nil {
             t.Fatalf("expected error for invalid new req")
        }
    })
}
