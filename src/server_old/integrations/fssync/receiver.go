package fssync

import (
	"encoding/json"
	"net/http"
	"sync"
)

// Receiver handles incoming file chunks and reconstructs files
type Receiver struct {
	mu           sync.Mutex
	Chunks       map[string][]FileChunk
	Reconstructed map[string][]byte
}

// NewReceiver creates a new Receiver
func NewReceiver() *Receiver {
	return &Receiver{
		Chunks:        make(map[string][]FileChunk),
		Reconstructed: make(map[string][]byte),
	}
}

// HandleSyncFS is the HTTP handler for POST /api/hybrid/sync/fs
func (r *Receiver) HandleSyncFS(w http.ResponseWriter, req *http.Request) {
	if req.Method != http.MethodPost {
		http.Error(w, "Method not allowed", http.StatusMethodNotAllowed)
		return
	}

	var chunk FileChunk
	if err := json.NewDecoder(req.Body).Decode(&chunk); err != nil {
		http.Error(w, "Bad request", http.StatusBadRequest)
		return
	}

	r.mu.Lock()
	defer r.mu.Unlock()

	// Store chunk
	r.Chunks[chunk.Path] = append(r.Chunks[chunk.Path], chunk)

	// Check if all chunks have been received
	if len(r.Chunks[chunk.Path]) == chunk.TotalChunks {
		// Reconstruct file
		reconstructed := make([]byte, 0)
		// Need to sort or ensure chunks are in order, but for simplicity assuming sequential or small tests
		// Create a slice of the right size and put chunks in place
		chunks := r.Chunks[chunk.Path]
		orderedChunks := make([]FileChunk, chunk.TotalChunks)
		for _, c := range chunks {
			if c.ChunkIndex >= 0 && c.ChunkIndex < chunk.TotalChunks {
				orderedChunks[c.ChunkIndex] = c
			}
		}

		for _, c := range orderedChunks {
			reconstructed = append(reconstructed, c.Data...)
		}

		r.Reconstructed[chunk.Path] = reconstructed
		// Clear chunks as it's fully reconstructed
		delete(r.Chunks, chunk.Path)
	}

	w.WriteHeader(http.StatusOK)
}
