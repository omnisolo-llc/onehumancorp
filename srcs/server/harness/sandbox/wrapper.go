package sandbox

import (
	"fmt"
	"os/exec"
	"strings"
)

type BashWrapper struct {
	readOnlyPaths  []string
	blockedDomains []string
}

func NewBashWrapper() *BashWrapper {
	return &BashWrapper{
		readOnlyPaths:  []string{},
		blockedDomains: []string{},
	}
}

func (bw *BashWrapper) UpdatePolicy(policy SandboxPolicy) {
	bw.readOnlyPaths = policy.ReadOnlyPaths
	bw.blockedDomains = policy.BlockedDomains
}

func (bw *BashWrapper) Wrap(cmd string) string {
	bwrapPath, err := exec.LookPath("bwrap")
	hasBwrap := err == nil

	preamble := "set -e; umask 077; "
	if len(bw.blockedDomains) > 0 {
		var domains []string
		for _, d := range bw.blockedDomains {
			domains = append(domains, strings.ReplaceAll(d, "'", "'\\''"))
		}
		preamble += fmt.Sprintf("export BLOCKED_DOMAINS='%s'; ", strings.Join(domains, ","))
	}

	escapedCmd := strings.ReplaceAll(cmd, "'", "'\\''")
	fullCmd := fmt.Sprintf("%s%s", preamble, escapedCmd)

	if !hasBwrap {
		return fmt.Sprintf("bash -c '%s'", fullCmd)
	}

	var bwrapArgs []string
	bwrapArgs = append(bwrapArgs, bwrapPath, "--unshare-all", "--share-net")
	bwrapArgs = append(bwrapArgs, "--ro-bind", "/", "/")
	bwrapArgs = append(bwrapArgs, "--tmpfs", "/tmp", "--tmpfs", "/var")

	for _, p := range bw.readOnlyPaths {
		escapedPath := strings.ReplaceAll(p, "'", "'\\''")
		bwrapArgs = append(bwrapArgs, "--ro-bind", fmt.Sprintf("'%s'", escapedPath), fmt.Sprintf("'%s'", escapedPath))
	}

	bwrapArgs = append(bwrapArgs, "--", "bash", "-c", fmt.Sprintf("'%s'", fullCmd))

	return strings.Join(bwrapArgs, " ")
}
