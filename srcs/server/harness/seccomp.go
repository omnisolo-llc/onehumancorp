package harness

import (
	"encoding/binary"
	"fmt"
	"os"

	"golang.org/x/net/bpf"
)

// CompileSeccompBPF compiles a seccomp BPF filter that blocks AF_UNIX socket creation
// and writes the raw instructions to the given file path.
func CompileSeccompBPF(path string) error {
	insts := []bpf.Instruction{
		// Load AUDIT_ARCH (offset 4 in seccomp_data)
		bpf.LoadAbsolute{Off: 4, Size: 4},
		// Check if architecture is X86_64 (0xc000003e)
		bpf.JumpIf{Cond: bpf.JumpEqual, Val: 0xc000003e, SkipTrue: 1},
		// Allow other architectures for now (or could be strict and block them)
		bpf.RetConstant{Val: 0x7fff0000}, // SECCOMP_RET_ALLOW

		// Load syscall number (offset 0 in seccomp_data)
		bpf.LoadAbsolute{Off: 0, Size: 4},
		// Check if syscall is SYS_SOCKET (41 on x86_64)
		bpf.JumpIf{Cond: bpf.JumpEqual, Val: 41, SkipFalse: 3},

		// Load first argument (domain) (offset 16 in seccomp_data)
		bpf.LoadAbsolute{Off: 16, Size: 4},
		// Check if domain is AF_UNIX (1)
		bpf.JumpIf{Cond: bpf.JumpEqual, Val: 1, SkipFalse: 1},
		// Return SECCOMP_RET_ERRNO | EACCES (13)
		bpf.RetConstant{Val: 0x00050000 | 13},

		// Default: Return SECCOMP_RET_ALLOW
		bpf.RetConstant{Val: 0x7fff0000},
	}

	raw, err := bpf.Assemble(insts)
	if err != nil {
		return fmt.Errorf("failed to assemble BPF instructions: %w", err)
	}

	f, err := os.Create(path)
	if err != nil {
		return fmt.Errorf("failed to create seccomp BPF file: %w", err)
	}
	defer f.Close()

	for _, inst := range raw {
		if err := binary.Write(f, binary.LittleEndian, inst); err != nil {
			return fmt.Errorf("failed to write BPF instruction: %w", err)
		}
	}

	return nil
}
