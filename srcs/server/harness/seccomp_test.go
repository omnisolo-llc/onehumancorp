package harness

import (
	"os"
	"path/filepath"
	"testing"
)

func TestCompileSeccompBPF(t *testing.T) {
	tmpDir := t.TempDir()
	path := filepath.Join(tmpDir, "seccomp.bpf")

	err := CompileSeccompBPF(path)
	if err != nil {
		t.Fatalf("expected no error compiling seccomp BPF, got %v", err)
	}

	info, err := os.Stat(path)
	if err != nil {
		t.Fatalf("expected seccomp BPF file to exist, got error: %v", err)
	}

	if info.Size() == 0 {
		t.Fatalf("expected seccomp BPF file to have content, got size 0")
	}

	// Try assembling with an invalid path
	err = CompileSeccompBPF("/nonexistent/path/that/will/fail/seccomp.bpf")
	if err == nil {
		t.Fatalf("expected error when writing to invalid path, got nil")
	}
}
