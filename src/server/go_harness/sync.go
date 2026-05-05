package harness

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"sync"
)

type FileSyncBridge struct {
	mu sync.Mutex
	hashes map[string]string
}

func NewFileSyncBridge() *FileSyncBridge {
	return &FileSyncBridge{
		hashes: make(map[string]string),
	}
}

func (s *FileSyncBridge) SyncFile(ctx context.Context, path string, content []byte) error {
	// mock redis lock
	s.mu.Lock()
	defer s.mu.Unlock()

	hash := sha256.Sum256(content)
	hashStr := hex.EncodeToString(hash[:])

	if prevHash, ok := s.hashes[path]; ok && prevHash == hashStr {
		// already in sync
		return nil
	}

	s.hashes[path] = hashStr
	RecordSyncFile(ctx)

	return nil
}
