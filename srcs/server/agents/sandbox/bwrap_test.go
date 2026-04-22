package sandbox

import (
	"context"
	"reflect"
	"strings"
	"testing"
)

func TestLinuxBwrapAdapter_BuildArgs(t *testing.T) {
	adapter := NewLinuxBwrapAdapter()

	cfg := Config{
		Binds: map[string]string{
			"/tmp/host_rw": "/tmp/sandbox_rw",
			"/var/host_rw": "/var/sandbox_rw",
		},
		RoBinds: map[string]string{
			"/usr": "/usr",
			"/etc": "/etc",
		},
	}

	cmd := "echo 'hello world'"

	args := adapter.BuildArgs(cmd, cfg)

	expectedArgs := []string{
		"--unshare-pid",
		"--unshare-net",
		"--dev", "/dev",
		"--bind", "/tmp/host_rw", "/tmp/sandbox_rw",
		"--bind", "/var/host_rw", "/var/sandbox_rw",
		"--ro-bind", "/etc", "/etc",
		"--ro-bind", "/usr", "/usr",
		"--", "bash", "-c", cmd,
	}

	if !reflect.DeepEqual(args, expectedArgs) {
		t.Errorf("BuildArgs() returned \n%v\n expected \n%v", args, expectedArgs)
	}
}

func TestLinuxBwrapAdapter_Execute(t *testing.T) {
	// Instead of requiring bwrap to be installed on the system to pass the test,
	// we will mock the executable to simply be `echo`.
	// This will just output the arguments we gave it.
	adapter := &LinuxBwrapAdapter{
		BwrapPath: "echo",
	}

	cfg := Config{
		RoBinds: map[string]string{
			"/bin": "/bin",
		},
	}

	cmd := "ls -la"

	res, err := adapter.Execute(context.Background(), cmd, cfg)
	if err != nil {
		t.Fatalf("Execute() failed: %v", err)
	}

	// Because we replaced `bwrap` with `echo`, the result will be the arguments printed out
	// space-separated.
	expectedOutputSubstring := "--unshare-pid --unshare-net --dev /dev --ro-bind /bin /bin -- bash -c ls -la"

	if !strings.Contains(strings.TrimSpace(res.Output), expectedOutputSubstring) {
		t.Errorf("Execute() output %q did not contain %q", res.Output, expectedOutputSubstring)
	}
}

func TestLinuxBwrapAdapter_Execute_Error(t *testing.T) {
	// Test the error path by providing a non-existent binary
	adapter := &LinuxBwrapAdapter{
		BwrapPath: "nonexistent-bwrap-binary-12345",
	}

	cfg := Config{}
	cmd := "ls"

	_, err := adapter.Execute(context.Background(), cmd, cfg)
	if err == nil {
		t.Fatalf("Expected Execute() to fail with non-existent binary")
	}

	if !strings.Contains(err.Error(), "bwrap execution failed") {
		t.Errorf("Expected error to contain 'bwrap execution failed', got: %v", err)
	}
}
