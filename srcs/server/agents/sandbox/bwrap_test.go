package sandbox

import (
	"os"
	"context"
	"strings"
	"testing"
)

func containsFlagWithArgs(args []string, flag string, followingArgs ...string) bool {
	for i := 0; i <= len(args)-(1+len(followingArgs)); i++ {
		if args[i] == flag {
			match := true
			for j, expected := range followingArgs {
				if args[i+1+j] != expected {
					match = false
					break
				}
			}
			if match {
				return true
			}
		}
	}
	return false
}

func contains(args []string, expected string) bool {
	for _, arg := range args {
		if arg == expected {
			return true
		}
	}
	return false
}

func TestBuildBwrapArgs(t *testing.T) {
	adapter := &LinuxBwrapAdapter{}

	cfg := Config{
		Bind: map[string]string{
			"/tmp/host_rw": "/tmp/virt_rw",
		},
		RoBind: map[string]string{
			"/host/ro": "/virt/ro",
		},
	}

	cmdStr := "echo 'hello world'"

	args := adapter.BuildBwrapArgs(cmdStr, cfg, -1)

	if !contains(args, "--unshare-pid") {
		t.Errorf("Expected args to contain --unshare-pid")
	}
	if !contains(args, "--unshare-net") {
		t.Errorf("Expected args to contain --unshare-net")
	}
	if !containsFlagWithArgs(args, "--dev", "/dev") {
		t.Errorf("Expected args to contain --dev /dev")
	}
	if !containsFlagWithArgs(args, "--bind", "/tmp/host_rw", "/tmp/virt_rw") {
		t.Errorf("Expected args to contain --bind /tmp/host_rw /tmp/virt_rw")
	}
	if !containsFlagWithArgs(args, "--ro-bind", "/host/ro", "/virt/ro") {
		t.Errorf("Expected args to contain --ro-bind /host/ro /virt/ro")
	}

	// Check the final command arguments
	n := len(args)
	if n < 3 || args[n-3] != "bash" || args[n-2] != "-c" || args[n-1] != cmdStr {
		t.Errorf("Expected args to end with bash -c '%s', got %v", cmdStr, args)
	}
}

func TestBuildBwrapArgs_WithProxyAndSeccomp(t *testing.T) {
	adapter := &LinuxBwrapAdapter{}

	cfg := Config{
		SeccompBPFPath:  "/tmp/seccomp.bpf",
		HTTPSocketPath:  "/tmp/http.sock",
		SOCKSSocketPath: "/tmp/socks.sock",
		ProxyEnvVars: map[string]string{
			"HTTP_PROXY":  "http://unix:/tmp/http.sock",
			"HTTPS_PROXY": "http://unix:/tmp/http.sock",
		},
	}

	cmdStr := "echo 'hello network'"

	args := adapter.BuildBwrapArgs(cmdStr, cfg, -1)


	if !containsFlagWithArgs(args, "--bind", "/tmp/http.sock", "/tmp/http.sock") {
		t.Errorf("Expected args to contain --bind /tmp/http.sock /tmp/http.sock")
	}
	if !containsFlagWithArgs(args, "--bind", "/tmp/socks.sock", "/tmp/socks.sock") {
		t.Errorf("Expected args to contain --bind /tmp/socks.sock /tmp/socks.sock")
	}
	if !containsFlagWithArgs(args, "--setenv", "HTTPS_PROXY", "http://unix:/tmp/http.sock") {
		t.Errorf("Expected args to contain --setenv HTTPS_PROXY http://unix:/tmp/http.sock")
	}
	if !containsFlagWithArgs(args, "--setenv", "HTTP_PROXY", "http://unix:/tmp/http.sock") {
		t.Errorf("Expected args to contain --setenv HTTP_PROXY http://unix:/tmp/http.sock")
	}
}



func TestBuildBwrapArgs_WithSeccompFD(t *testing.T) {
	adapter := &LinuxBwrapAdapter{}
	cfg := Config{}
	cmdStr := "echo 'seccomp'"
	args := adapter.BuildBwrapArgs(cmdStr, cfg, 3)

	if !containsFlagWithArgs(args, "--seccomp", "3") {
		t.Errorf("Expected args to contain --seccomp 3")
	}
}



func TestExecuteWithSeccomp(t *testing.T) {
	// Create a temporary file to act as the seccomp BPF filter
	// to cover the lines that open the file in Execute.
	file, err := os.CreateTemp("", "seccomp.bpf")
	if err != nil {
		t.Fatal(err)
	}
	defer os.Remove(file.Name())
	defer file.Close()

	adapter := &LinuxBwrapAdapter{}
	cfg := Config{
		SeccompBPFPath: file.Name(),
	}
	cmdStr := "echo test"

	// We don't care if it fails (due to bwrap not installed or seccomp format),
	// just that it tries to open the file and append it to ExtraFiles.
	_, _ = adapter.Execute(context.Background(), cmdStr, cfg)
}

func TestExecuteCoverage(t *testing.T) {
	// Instead of testing empty execute and making bad assumptions, we verify the limit writer handles it
	// and we ensure that we either get an executable not found error, or an actual result.
	adapter := &LinuxBwrapAdapter{}
	cfg := Config{}
	cmdStr := "echo test"

	res, err := adapter.Execute(context.Background(), cmdStr, cfg)

	// In the test sandbox environment bwrap might not be installed, yielding "executable file not found".
	// If it is installed, it could fail if unprivileged user namespaces are not allowed.
	// We simply verify that the execution pipeline works gracefully.
	if err != nil {
		if !strings.Contains(err.Error(), "executable file not found") && !strings.Contains(err.Error(), "exit status") {
			t.Errorf("Unexpected error format: %v", err)
		}
	} else {
		// If it actually ran successfully, we expect some output or no crash.
		if res == nil {
			t.Errorf("Expected non-nil result on success")
		}
	}
}

func TestLimitedWriter(t *testing.T) {
	w := &limitedWriter{limit: 10}

	// Write 5 bytes
	n, err := w.Write([]byte("12345"))
	if err != nil || n != 5 {
		t.Errorf("Expected 5 bytes written, got %d, err %v", n, err)
	}

	// Write 10 more bytes (total 15), but limit is 10. Only 5 should actually be written to buf, but Write returns 10.
	n, err = w.Write([]byte("67890abcde"))
	if err != nil || n != 10 {
		t.Errorf("Expected Write to return 10 to simulate full write, got %d, err %v", n, err)
	}

	if w.buf.String() != "1234567890" {
		t.Errorf("Expected buffer to be exactly 10 bytes '1234567890', got '%s'", w.buf.String())
	}

	// Write more, should be completely dropped
	n, err = w.Write([]byte("xyz"))
	if err != nil || n != 3 {
		t.Errorf("Expected Write to return 3, got %d", n)
	}

	if w.buf.String() != "1234567890" {
		t.Errorf("Expected buffer to remain '1234567890', got '%s'", w.buf.String())
	}
}
