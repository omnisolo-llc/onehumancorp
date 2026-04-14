package tests

import (
	"context"
	"fmt"
	"os"
	"path/filepath"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/lib/resilience"
)

// TestMeshFallback_MailboxCorruption simulates a scenario where .agent-task/mailbox/
// becomes corrupted or read-only, and resilience.WithRetry helps the system recover
// once the permission issue is resolved.
func TestMeshFallback_MailboxCorruption(t *testing.T) {
	tmpDir := t.TempDir()
	mailboxDir := filepath.Join(tmpDir, ".agent-task", "mailbox")
	if err := os.MkdirAll(mailboxDir, 0755); err != nil {
		t.Fatalf("Failed to create mailbox dir: %v", err)
	}
	messageFile := filepath.Join(mailboxDir, "message.txt")

	// Pre-corrupt the directory permissions
	os.Chmod(mailboxDir, 0400)
	defer os.Chmod(mailboxDir, 0755)

	var wg sync.WaitGroup
	ctx, cancel := context.WithTimeout(context.Background(), 10*time.Second)
	defer cancel()

	// Simulate a background agent that eventually fixes the directory
	wg.Add(1)
	go func() {
		defer wg.Done()
		time.Sleep(500 * time.Millisecond)
		os.Chmod(mailboxDir, 0755)
	}()

	err := resilience.WithRetry(ctx, 10, 100*time.Millisecond, func(c context.Context) error {
		f, err := os.OpenFile(messageFile, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0666)
		if err != nil {
			return fmt.Errorf("failed to write to mailbox: %w", err)
		}
		defer f.Close()
		_, err = f.WriteString("chaos message")
		return err
	})

	wg.Wait()

	if err != nil {
		t.Errorf("Expected mailbox write to succeed after recovery, got error: %v", err)
	} else {
		t.Logf("Successfully recovered from mailbox corruption via retry.")
	}
}
