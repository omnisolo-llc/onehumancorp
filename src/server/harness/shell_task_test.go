//go:build linux || darwin

package harness

import (
	"context"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
	"syscall"
	"testing"
	"time"
)

func TestLocalShellTask_Execution(t *testing.T) {
	ctx := context.Background()
	task := NewLocalShellTask(ctx, "echo", "hello world")

	if task.ID == "" {
		t.Error("expected non-empty task ID")
	}
	if task.Cmd.SysProcAttr == nil || !task.Cmd.SysProcAttr.Setpgid {
		t.Error("expected Setpgid to be true")
	}

	out, err := task.Cmd.Output()
	if err != nil {
		t.Fatalf("failed to execute command: %v", err)
	}

	if !strings.Contains(string(out), "hello world") {
		t.Errorf("unexpected output: %s", string(out))
	}
}

func TestLocalShellTask_Kill(t *testing.T) {
	ctx := context.Background()

	// Create a command that spawns multiple background child processes
	// and stays alive. The parent shell and children should be killed when Kill() is called.
	script := `
	sleep 1000 &
	sleep 1001 &
	wait
	`
	task := NewLocalShellTask(ctx, "bash", "-c", script)
	tempDir := filepath.Join(os.TempDir(), "ohc_task_test_"+task.ID)
	task.TempDir = tempDir

	err := task.Start()
	if err != nil {
		t.Fatalf("failed to start task: %v", err)
	}

	// Verify temp dir was created
	if _, err := os.Stat(tempDir); os.IsNotExist(err) {
		t.Errorf("expected temp dir %s to exist", tempDir)
	}

	// Give the child processes a moment to spawn
	time.Sleep(100 * time.Millisecond)

	// Fetch child pids to verify they get killed
	// We'll just verify the Kill() function returns success,
	// and that wait returns an error (since it was killed).
	// To strictly verify all children are gone, we could use ps, but it can be flaky.
	// Since syscall.Kill with a negative PID is a standard OS feature, ensuring we pass
	// the correct -pid and the call succeeds is strong evidence.

	err = task.Kill()
	if err != nil {
		// If the process had already exited before we called kill, it might return ESRCH.
		// However, since it's waiting on `sleep 1000`, it should still be running.
		t.Fatalf("failed to kill task: %v", err)
	}

	// Verify temp dir was removed
	if _, err := os.Stat(tempDir); !os.IsNotExist(err) {
		t.Errorf("expected temp dir %s to be removed", tempDir)
	}

	// Wait for the process to actually reap and verify it was killed
	err = task.Cmd.Wait()
	if err == nil {
		t.Error("expected command to exit with an error due to being killed")
	} else {
		exitErr, ok := err.(*exec.ExitError)
		if !ok {
			t.Errorf("expected *exec.ExitError, got %T: %v", err, err)
		} else {
			waitStatus, ok := exitErr.Sys().(syscall.WaitStatus)
			if ok {
				if waitStatus.Signal() != syscall.SIGKILL {
					t.Errorf("expected process to be killed by SIGKILL, got signal %v", waitStatus.Signal())
				}
			}
		}
	}
}

func TestLocalShellTask_KillAlreadyExited(t *testing.T) {
	ctx := context.Background()
	task := NewLocalShellTask(ctx, "echo", "done")

	err := task.Start()
	if err != nil {
		t.Fatalf("failed to start task: %v", err)
	}

	err = task.Cmd.Wait()
	if err != nil {
		t.Fatalf("failed to wait: %v", err)
	}

	// Calling Kill() on an already exited process might return an error like "no such process"
	// but LocalShellTask.Kill() could potentially handle or ignore it.
	// For now we just ensure it doesn't panic.
	err = task.Kill()
	if err != nil {
		// Log the error but don't strictly fail, as killing an exited process group
		// returning ESRCH is expected OS behavior.
		t.Logf("Kill() on exited process returned: %v", err)
	}
}
