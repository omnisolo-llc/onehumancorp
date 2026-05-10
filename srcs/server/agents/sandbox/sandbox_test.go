package sandbox

import (
	"context"
	"os"
	"strings"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/require"
)

func TestNewSandboxManager(t *testing.T) {
	sm, err := NewSandboxManager()
	require.NoError(t, err)
	defer sm.Cleanup()

	// Verify tmpDir is created and has correct permissions
	info, err := os.Stat(sm.tmpDir)
	require.NoError(t, err)
	assert.True(t, info.IsDir())
	assert.Equal(t, os.FileMode(0700)|os.ModeDir, info.Mode())
}

func TestSandboxManager_Execute_TMPDIR(t *testing.T) {
	sm, err := NewSandboxManager()
	require.NoError(t, err)
	defer sm.Cleanup()

	ctx := context.Background()
	stdout, stderr, err := sm.Execute(ctx, "echo $TMPDIR")
	require.NoError(t, err)
	assert.Empty(t, stderr)
	assert.Equal(t, sm.tmpDir+"\n", stdout)
}

func TestSandboxManager_Execute_ShoptExtglob(t *testing.T) {
	sm, err := NewSandboxManager()
	require.NoError(t, err)
	defer sm.Cleanup()

	ctx := context.Background()
	// shopt will exit with 1 if extglob is off, 0 if it is on
	// We use "shopt -q extglob" to quietly check the status
	// Since we disabled it, the exit status should be 1
	stdout, stderr, err := sm.Execute(ctx, "shopt extglob")

	// 'err' will be non-nil if exit status is non-zero, but we just check the output here.
	// Actually `shopt extglob` will print "extglob        	off"
	_ = err
	assert.Empty(t, stderr)
	assert.True(t, strings.Contains(stdout, "off"), "expected extglob to be off, got: %s", stdout)
}

func TestSandboxManager_Execute_Timeout(t *testing.T) {
	sm, err := NewSandboxManager()
	require.NoError(t, err)
	defer sm.Cleanup()

	ctx, cancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer cancel()

	// Sleep longer than the timeout
	_, _, err = sm.Execute(ctx, "sleep 1")
	require.Error(t, err)
	assert.Contains(t, err.Error(), "killed") // exec.Cmd returns error containing "killed" or "signal: killed" on context timeout
}
